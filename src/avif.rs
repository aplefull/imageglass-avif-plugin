use crate::abi::*;
use crate::sys;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::{Arc, LazyLock, Mutex, MutexGuard};
use std::time::UNIX_EPOCH;

/// Open files kept around
const MAX_CACHED_FILES: usize = 4;

/// Locks a mutex even if a panic poisoned it
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}

/// Threads for AV1 decoding and YUV conversion
fn worker_threads() -> usize {
    std::thread::available_parallelism()
        .map_or(1, |n| n.get())
        .min(8)
}

fn result_text(result: sys::avifResult) -> String {
    // ACTUALLY_SAFE: libavif returns a static C string for every result
    unsafe {
        CStr::from_ptr(sys::avifResultToString(result))
            .to_string_lossy()
            .into_owned()
    }
}

pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub has_alpha: bool,
    pub frame_count: u32,
    pub file_size: u64,
    pub color_space: i32,
}

pub struct Frame {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct AvifFile {
    decoder: *mut sys::avifDecoder,
    _data: Vec<u8>,
    has_frame: bool,
}

// ACTUALLY_SAFE: a libavif decoder has no thread affinity, the cache wraps every file in a
// Mutex, so it is never used from two threads at once.
unsafe impl Send for AvifFile {}

impl Drop for AvifFile {
    fn drop(&mut self) {
        // ACTUALLY_SAFE: created by avifDecoderCreate and destroyed exactly once
        unsafe { sys::avifDecoderDestroy(self.decoder) };
    }
}

impl AvifFile {
    fn open(path: &str) -> Result<Self, i32> {
        let data = std::fs::read(path).map_err(|_| IG_STATUS_IO_ERROR)?;

        // ACTUALLY_SAFE: plain libavif calls, the decoder only reads `data`, which the struct keeps alive
        unsafe {
            let decoder = sys::avifDecoderCreate();
            if decoder.is_null() {
                return Err(IG_STATUS_OUT_OF_MEMORY);
            }

            let file = Self {
                decoder,
                _data: data,
                has_frame: false,
            };

            (*decoder).maxThreads = worker_threads() as i32;
            (*decoder).strictFlags = sys::AVIF_STRICT_DISABLED as _;
            (*decoder).ignoreExif = sys::AVIF_TRUE as _;
            (*decoder).ignoreXMP = sys::AVIF_TRUE as _;

            let result =
                sys::avifDecoderSetIOMemory(decoder, file._data.as_ptr(), file._data.len());
            if result != sys::AVIF_RESULT_OK {
                return Err(IG_STATUS_INTERNAL);
            }

            let result = sys::avifDecoderParse(decoder);
            if result != sys::AVIF_RESULT_OK {
                crate::host::log(
                    LOG_WARN,
                    &format!("AVIF: cannot parse '{path}': {}", result_text(result)),
                );
                return Err(IG_STATUS_DECODE_FAILED);
            }

            Ok(file)
        }
    }

    /// Size of the displayed image, after the container's rotation is applied.
    fn oriented_size(&self) -> (u32, u32) {
        // ACTUALLY_SAFE: decoder.image is valid after a successful parse
        unsafe {
            let image = &*(*self.decoder).image;
            if has_rotation(image) && image.irot.angle % 2 == 1 {
                (image.height, image.width)
            } else {
                (image.width, image.height)
            }
        }
    }

    /// Returns metadata about the image, duh
    pub fn metadata(&self) -> ImageMetadata {
        let (width, height) = self.oriented_size();

        // ACTUALLY_SAFE: decoder.image is valid after a successful parse
        unsafe {
            let decoder = &*self.decoder;
            let image = &*decoder.image;

            ImageMetadata {
                width,
                height,
                has_alpha: decoder.alphaPresent != 0,
                frame_count: decoder.imageCount.max(1) as u32,
                file_size: self._data.len() as u64,
                color_space: match image.colorPrimaries as i32 {
                    sys::AVIF_COLOR_PRIMARIES_BT2020 => IG_COLOR_SPACE_REC2020,
                    sys::AVIF_COLOR_PRIMARIES_SMPTE432 => IG_COLOR_SPACE_DISPLAY_P3,
                    _ => IG_COLOR_SPACE_SRGB,
                },
            }
        }
    }

    /// The embedded ICC profile, if any
    pub fn icc_profile(&self) -> &[u8] {
        // ACTUALLY_SAFE: the ICC buffer belongs to decoder.image and lives as long as `self`
        unsafe {
            let icc = &(*(*self.decoder).image).icc;
            if icc.data.is_null() || icc.size == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(icc.data, icc.size)
            }
        }
    }

    /// Per-frame durations in milliseconds and the loop count
    pub fn animation_timing(&self) -> (Vec<i32>, i32) {
        // ACTUALLY_SAFE: timing reads only the parsed container
        unsafe {
            let decoder = &*self.decoder;
            let count = decoder.imageCount.max(1) as u32;

            let durations = (0..count)
                .map(|i| {
                    let mut timing: sys::avifImageTiming = std::mem::zeroed();
                    if sys::avifDecoderNthImageTiming(self.decoder, i, &mut timing)
                        == sys::AVIF_RESULT_OK
                    {
                        (timing.duration * 1000.0).round() as i32
                    } else {
                        0
                    }
                })
                .collect();

            let loop_count = if decoder.repetitionCount < 0 {
                0
            } else {
                decoder.repetitionCount + 1
            };

            (durations, loop_count)
        }
    }

    /// Decodes `index` frame and converts it to BGRA
    pub fn decode_frame(&mut self, index: u32) -> Result<Frame, i32> {
        // ACTUALLY_SAFE: decoder is valid and the Mutex around self makes access exclusive
        unsafe {
            let decoder = self.decoder;
            let count = (*decoder).imageCount.max(1) as u32;
            if index >= count {
                return Err(IG_STATUS_INVALID_ARG);
            }

            let current = (*decoder).imageIndex;
            let is_current = self.has_frame && current == index as i32;

            if !is_current {
                let result = if current + 1 == index as i32 {
                    sys::avifDecoderNextImage(decoder)
                } else {
                    sys::avifDecoderNthImage(decoder, index)
                };

                self.has_frame = result == sys::AVIF_RESULT_OK;
                if !self.has_frame {
                    crate::host::log(
                        LOG_WARN,
                        &format!("AVIF: frame {index} failed: {}", result_text(result)),
                    );
                    return Err(IG_STATUS_DECODE_FAILED);
                }
            }

            let frame = to_bgra(&*(*decoder).image)?;
            apply_transforms(frame, &*(*decoder).image)
        }
    }
}

/// Converts the decoder's YUV image to straight-alpha BGRA8
fn to_bgra(image: &sys::avifImage) -> Result<Frame, i32> {
    let width = image.width;
    let height = image.height;
    let row_bytes = width as usize * 4;
    let size = row_bytes
        .checked_mul(height as usize)
        .ok_or(IG_STATUS_OUT_OF_MEMORY)?;

    if size > i32::MAX as usize {
        return Err(IG_STATUS_OUT_OF_MEMORY);
    }

    let mut pixels = alloc_pixels(size)?;

    // ACTUALLY_SAFE: rgb points at `pixels`, sized for width x height x 4
    unsafe {
        let mut rgb: sys::avifRGBImage = std::mem::zeroed();
        sys::avifRGBImageSetDefaults(&mut rgb, image);
        rgb.format = sys::AVIF_RGB_FORMAT_BGRA;
        rgb.depth = 8;
        rgb.alphaPremultiplied = sys::AVIF_FALSE as _;
        rgb.maxThreads = worker_threads() as i32;
        rgb.pixels = pixels.as_mut_ptr();
        rgb.rowBytes = row_bytes as u32;

        let result = sys::avifImageYUVToRGB(image, &mut rgb);
        if result != sys::AVIF_RESULT_OK {
            crate::host::log(
                LOG_WARN,
                &format!("AVIF: YUV to RGB failed: {}", result_text(result)),
            );
            return Err(IG_STATUS_DECODE_FAILED);
        }
    }

    Ok(Frame {
        pixels,
        width,
        height,
    })
}

/// A zeroed pixel buffer, or an error if memory runs out
fn alloc_pixels(size: usize) -> Result<Vec<u8>, i32> {
    let mut pixels = Vec::new();
    pixels
        .try_reserve_exact(size)
        .map_err(|_| IG_STATUS_OUT_OF_MEMORY)?;
    pixels.resize(size, 0);
    Ok(pixels)
}

/// Returns true if the image has a rotation transform
fn has_rotation(image: &sys::avifImage) -> bool {
    image.transformFlags & sys::AVIF_TRANSFORM_IROT as u32 != 0 && image.irot.angle % 4 != 0
}

/// Returns true if the image has a mirror transform
fn has_mirror(image: &sys::avifImage) -> bool {
    image.transformFlags & sys::AVIF_TRANSFORM_IMIR as u32 != 0
}

/// Applies the container's 'irot' then 'imir'
fn apply_transforms(frame: Frame, image: &sys::avifImage) -> Result<Frame, i32> {
    let ccw = if has_rotation(image) {
        image.irot.angle % 4
    } else {
        0
    };
    let mirror = has_mirror(image).then_some(image.imir.axis);

    let (flip, cw) = match mirror {
        None => (false, (4 - ccw) % 4),
        Some(0) => (true, ccw),
        Some(_) => (true, (ccw + 2) % 4),
    };

    if !flip && cw == 0 {
        return Ok(frame);
    }

    let (w, h) = (frame.width, frame.height);
    let (dst_w, dst_h) = if cw % 2 == 1 { (h, w) } else { (w, h) };
    let mut pixels = alloc_pixels(frame.pixels.len())?;

    // ACTUALLY_SAFE: both buffers hold w x h BGRA pixels
    let result = unsafe {
        sys::ARGBRotate(
            frame.pixels.as_ptr(),
            w as i32 * 4,
            pixels.as_mut_ptr(),
            dst_w as i32 * 4,
            w as i32,
            if flip { -(h as i32) } else { h as i32 },
            cw as i32 * 90,
        )
    };

    if result != 0 {
        return Err(IG_STATUS_INTERNAL);
    }

    Ok(Frame {
        pixels,
        width: dst_w,
        height: dst_h,
    })
}

struct CacheEntry {
    path: String,
    stamp: (u64, u64),
    file: Arc<Mutex<AvifFile>>,
}

static CACHE: LazyLock<Mutex<Vec<CacheEntry>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Modification time and size
fn file_stamp(path: &str) -> Result<(u64, u64), i32> {
    let meta = std::fs::metadata(path).map_err(|_| IG_STATUS_IO_ERROR)?;
    let modified = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos() as u64);
    Ok((modified, meta.len()))
}

/// Returns the open file for `path`, parsing it on a miss
pub fn acquire(path: &str) -> Result<Arc<Mutex<AvifFile>>, i32> {
    let stamp = file_stamp(path)?;
    let mut cache = lock(&CACHE);

    if let Some(pos) = cache.iter().position(|e| e.path == path) {
        let entry = cache.remove(pos);
        if entry.stamp == stamp {
            let file = entry.file.clone();
            cache.insert(0, entry);
            return Ok(file);
        }
    }

    let file = Arc::new(Mutex::new(AvifFile::open(path)?));
    cache.insert(
        0,
        CacheEntry {
            path: path.to_owned(),
            stamp,
            file: file.clone(),
        },
    );
    cache.truncate(MAX_CACHED_FILES);

    Ok(file)
}

/// Drops every open file
pub fn clear_cache() {
    lock(&CACHE).clear();
}

/// Runs `f` on the locked file for `path`
pub fn with_file<T>(path: &str, f: impl FnOnce(&mut AvifFile) -> Result<T, i32>) -> Result<T, i32> {
    let file = acquire(path)?;
    let mut file = lock(&file);
    f(&mut file)
}

static LIVE_BUFFERS: LazyLock<Mutex<HashMap<usize, Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Moves the frame into an IGPixelBuffer the host owns until it calls FreePixelBuffer
pub fn hand_over(frame: Frame, out: &mut IGPixelBuffer) {
    let mut pixels = frame.pixels;
    let data = pixels.as_mut_ptr();

    out.data = data;
    out.width = frame.width as i32;
    out.height = frame.height as i32;
    out.stride = frame.width as i32 * 4;
    out.pixel_format = IG_PIXEL_FORMAT_BGRA8_UNORM;
    out.release_context = data.cast();

    lock(&LIVE_BUFFERS).insert(data as usize, pixels);
}

/// Frees a buffer from `hand_over`
pub fn release(data: *mut u8) {
    let pixels = lock(&LIVE_BUFFERS).remove(&(data as usize));
    drop(pixels);
}
