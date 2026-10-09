use crate::abi::*;
use crate::sys;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard};
use std::time::UNIX_EPOCH;

/// Open files kept around
const MAX_CACHED_FILES: usize = 4;

/// Memory all frame caches may hold together, an animation is cached only if all of it fits
const FRAME_CACHE_BUDGET: usize = 256 << 20;

/// Bytes currently reserved by frame caches
static FRAME_CACHE_USED: AtomicUsize = AtomicUsize::new(0);

/// Locks a mutex even if a panic poisoned it
pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
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
    pub hdr_transfer_fn: i32,
    pub pixel_format: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Bgra8,
    Rgba16,
}

impl PixelFormat {
    pub fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Bgra8 => 4,
            Self::Rgba16 => 8,
        }
    }

    pub fn ig_format(self) -> i32 {
        match self {
            Self::Bgra8 => IG_PIXEL_FORMAT_BGRA8_UNORM,
            Self::Rgba16 => IG_PIXEL_FORMAT_RGBA16_UNORM,
        }
    }
}

pub struct Frame {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
}

struct FrameCache {
    frame_bytes: usize,
    frame_count: usize,
    size: usize,
    frames: Mutex<Option<Vec<Option<Arc<Frame>>>>>,
}

impl FrameCache {
    fn new(frame_bytes: usize, frame_count: u32) -> Self {
        let size = frame_bytes
            .checked_mul(frame_count as usize)
            .filter(|&size| frame_count > 1 && size <= FRAME_CACHE_BUDGET)
            .unwrap_or(0);

        Self {
            frame_bytes,
            frame_count: frame_count as usize,
            size,
            frames: Mutex::new(None),
        }
    }

    fn wants_room(&self) -> bool {
        self.size > 0 && lock(&self.frames).is_none()
    }

    fn is_reserved(&self) -> bool {
        lock(&self.frames).is_some()
    }

    fn try_reserve(&self) -> bool {
        let mut frames = lock(&self.frames);
        if frames.is_some() {
            return true;
        }
        if self.size == 0 {
            return false;
        }

        let reserved = FRAME_CACHE_USED
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(self.size)
                    .filter(|&total| total <= FRAME_CACHE_BUDGET)
            })
            .is_ok();

        if reserved {
            *frames = Some(vec![None; self.frame_count]);
        }
        reserved
    }

    fn release(&self) {
        if lock(&self.frames).take().is_some() {
            FRAME_CACHE_USED.fetch_sub(self.size, Ordering::AcqRel);
        }
    }

    fn get(&self, index: u32) -> Option<Arc<Frame>> {
        lock(&self.frames).as_ref()?.get(index as usize)?.clone()
    }

    fn contains(&self, index: u32) -> bool {
        lock(&self.frames)
            .as_ref()
            .and_then(|frames| frames.get(index as usize))
            .is_some_and(Option::is_some)
    }

    fn insert(&self, index: u32, frame: Frame) -> Option<Frame> {
        if frame.pixels.len() != self.frame_bytes {
            return Some(frame);
        }

        let mut frames = lock(&self.frames);
        match frames
            .as_mut()
            .and_then(|frames| frames.get_mut(index as usize))
        {
            Some(slot) => {
                *slot = Some(Arc::new(frame));
                None
            }
            None => Some(frame),
        }
    }
}

impl Drop for FrameCache {
    fn drop(&mut self) {
        self.release();
    }
}

pub struct AvifFile {
    decoder: *mut sys::avifDecoder,
    _data: Vec<u8>,
    has_frame: bool,
    frames: Arc<FrameCache>,
    next_wanted: Option<u32>,
    prefetched: Option<(u32, Frame)>,
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
        let mut data = std::fs::read(path).map_err(|_| IG_STATUS_IO_ERROR)?;

        // AD-HOC: repair duplicate ipma boxes
        if crate::bmff::split_duplicate_ipma(&mut data) {
            crate::host::log(
                LOG_WARN,
                &format!("AVIF: '{path}' has duplicate ipma boxes, repaired in memory"),
            );
        }

        // ACTUALLY_SAFE: plain libavif calls, the decoder only reads `data`, which the struct keeps alive
        unsafe {
            let decoder = sys::avifDecoderCreate();
            if decoder.is_null() {
                return Err(IG_STATUS_OUT_OF_MEMORY);
            }

            let mut file = Self {
                decoder,
                _data: data,
                has_frame: false,
                frames: Arc::new(FrameCache::new(0, 0)),
                next_wanted: None,
                prefetched: None,
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

            let image = &*(*decoder).image;
            let frame_bytes = (image.width as usize)
                .checked_mul(image.height as usize)
                .and_then(|pixels| pixels.checked_mul(4))
                .unwrap_or(0);
            file.frames = Arc::new(FrameCache::new(frame_bytes, file.frame_count()));

            Ok(file)
        }
    }

    fn frame_count(&self) -> u32 {
        // ACTUALLY_SAFE: the decoder lives as long as `self`
        unsafe { (*self.decoder).imageCount.max(1) as u32 }
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

    /// Format of full size still images: 16 bits when the file has more than 8 and the
    /// buffer stays under the host's 2 GiB limit
    fn still_format(&self) -> PixelFormat {
        // ACTUALLY_SAFE: decoder.image is valid after a successful parse
        let image = unsafe { &*(*self.decoder).image };
        let fits = (image.width as usize)
            .checked_mul(image.height as usize)
            .and_then(|pixels| pixels.checked_mul(PixelFormat::Rgba16.bytes_per_pixel()))
            .is_some_and(|size| size <= i32::MAX as usize);

        if image.depth > 8 && fits {
            PixelFormat::Rgba16
        } else {
            PixelFormat::Bgra8
        }
    }

    /// Returns metadata about the image, duh
    pub fn metadata(&self) -> ImageMetadata {
        let (width, height) = self.oriented_size();

        // ACTUALLY_SAFE: decoder.image is valid after a successful parse
        unsafe {
            let decoder = &*self.decoder;
            let image = &*decoder.image;
            let transfer = image.transferCharacteristics as i32;
            let linear = transfer == sys::AVIF_TRANSFER_CHARACTERISTICS_LINEAR;

            ImageMetadata {
                width,
                height,
                has_alpha: decoder.alphaPresent != 0,
                frame_count: decoder.imageCount.max(1) as u32,
                file_size: self._data.len() as u64,
                color_space: match image.colorPrimaries as i32 {
                    sys::AVIF_COLOR_PRIMARIES_BT2020 if linear => IG_COLOR_SPACE_REC2020_LINEAR,
                    sys::AVIF_COLOR_PRIMARIES_BT2020 => IG_COLOR_SPACE_REC2020,
                    sys::AVIF_COLOR_PRIMARIES_SMPTE432 => IG_COLOR_SPACE_DISPLAY_P3,
                    sys::AVIF_COLOR_PRIMARIES_BT709 | sys::AVIF_COLOR_PRIMARIES_UNSPECIFIED
                        if linear =>
                    {
                        IG_COLOR_SPACE_LINEAR_SRGB
                    }
                    _ => IG_COLOR_SPACE_SRGB,
                },
                hdr_transfer_fn: match transfer {
                    sys::AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084 => IG_HDR_TRANSFER_FN_PQ,
                    sys::AVIF_TRANSFER_CHARACTERISTICS_HLG => IG_HDR_TRANSFER_FN_HLG,
                    _ if !image.gainMap.is_null() => IG_HDR_TRANSFER_FN_GAIN_MAP,
                    _ => IG_HDR_TRANSFER_FN_NONE,
                },
                pixel_format: self.still_format().ig_format(),
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

    /// Returns `index` frame as BGRA8 from the frame cache or the prefetched slot if they have it
    fn decode_frame(&mut self, index: u32, format: PixelFormat) -> Result<Frame, i32> {
        if index >= self.frame_count() {
            return Err(IG_STATUS_INVALID_ARG);
        }

        if format != PixelFormat::Bgra8 {
            return self.decode_uncached(index, format, true);
        }

        if let Some(cached) = self.frames.get(index) {
            return copy_frame(&cached);
        }

        let frame = match self.prefetched.take() {
            Some((prefetched, frame)) if prefetched == index => frame,
            other => {
                self.prefetched = other;
                self.decode_uncached(index, format, true)?
            }
        };

        if self.frames.is_reserved()
            && let Ok(copy) = copy_frame(&frame)
        {
            self.frames.insert(index, copy);
        }

        Ok(frame)
    }

    /// Picks the frame to decode ahead after playback showed `shown`, if it is not ready yet
    fn plan_prefetch(&mut self, shown: u32) -> Option<u32> {
        let count = self.frame_count();
        if count < 2 {
            return None;
        }

        // thumbnails ask for frame 0 alone, it is playback only when the loop wraps around
        if shown == 0 && self.next_wanted != Some(0) {
            return None;
        }

        let next = (shown + 1) % count;
        self.next_wanted = Some(next);

        if self
            .prefetched
            .as_ref()
            .is_some_and(|(index, _)| *index != next)
        {
            self.prefetched = None;
        }

        let ready = self.prefetched.is_some() || self.frames.contains(next);
        (!ready).then_some(next)
    }

    /// Decodes `index` on the prefetch thread, unless playback has moved on or it is ready
    fn decode_ahead(&mut self, index: u32) {
        if self.next_wanted != Some(index)
            || self.frames.contains(index)
            || self.prefetched.as_ref().is_some_and(|(i, _)| *i == index)
        {
            return;
        }

        let Ok(frame) = self.decode_uncached(index, PixelFormat::Bgra8, false) else {
            return;
        };

        if let Some(frame) = self.frames.insert(index, frame) {
            self.prefetched = Some((index, frame));
        }
    }

    /// Decodes `index` frame with libavif and converts it to `format`
    fn decode_uncached(
        &mut self,
        index: u32,
        format: PixelFormat,
        log_failure: bool,
    ) -> Result<Frame, i32> {
        // ACTUALLY_SAFE: decoder is valid and the Mutex around self makes access exclusive
        unsafe {
            let decoder = self.decoder;
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
                    if log_failure {
                        crate::host::log(
                            LOG_WARN,
                            &format!("AVIF: frame {index} failed: {}", result_text(result)),
                        );
                    }
                    return Err(IG_STATUS_DECODE_FAILED);
                }
            }

            let frame = to_rgb(&*(*decoder).image, format, log_failure)?;
            apply_transforms(frame, &*(*decoder).image)
        }
    }
}

/// Converts the decoder's YUV image to straight-alpha `format`
fn to_rgb(image: &sys::avifImage, format: PixelFormat, log_failure: bool) -> Result<Frame, i32> {
    let width = image.width;
    let height = image.height;
    let row_bytes = width as usize * format.bytes_per_pixel();
    let size = row_bytes
        .checked_mul(height as usize)
        .ok_or(IG_STATUS_OUT_OF_MEMORY)?;

    if size > i32::MAX as usize {
        return Err(IG_STATUS_OUT_OF_MEMORY);
    }

    let mut pixels = alloc_pixels(size)?;

    if pixels.as_ptr().align_offset(2) != 0 {
        return Err(IG_STATUS_INTERNAL);
    }

    // AD-HOC: libavif refuses the constant luminance matrices, their non-constant twins are close enough to substitute
    let substitute = match image.matrixCoefficients as i32 {
        sys::AVIF_MATRIX_COEFFICIENTS_BT2020_CL => Some(sys::AVIF_MATRIX_COEFFICIENTS_BT2020_NCL),
        sys::AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL => {
            Some(sys::AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL)
        }
        _ => None,
    };
    let patched;
    let image = match substitute {
        Some(matrix) => {
            patched = sys::avifImage {
                matrixCoefficients: matrix as _,
                ..*image
            };
            &patched
        }
        None => image,
    };

    // ACTUALLY_SAFE: rgb points at `pixels`, sized for width x height pixels of `format`
    unsafe {
        let mut rgb: sys::avifRGBImage = std::mem::zeroed();
        sys::avifRGBImageSetDefaults(&mut rgb, image);
        (rgb.format, rgb.depth) = match format {
            PixelFormat::Bgra8 => (sys::AVIF_RGB_FORMAT_BGRA, 8),
            PixelFormat::Rgba16 => (sys::AVIF_RGB_FORMAT_RGBA, 16),
        };
        rgb.alphaPremultiplied = sys::AVIF_FALSE as _;
        rgb.maxThreads = worker_threads() as i32;
        rgb.pixels = pixels.as_mut_ptr();
        rgb.rowBytes = row_bytes as u32;

        let result = sys::avifImageYUVToRGB(image, &mut rgb);
        if result != sys::AVIF_RESULT_OK {
            if log_failure {
                crate::host::log(
                    LOG_WARN,
                    &format!("AVIF: YUV to RGB failed: {}", result_text(result)),
                );
            }
            return Err(IG_STATUS_DECODE_FAILED);
        }
    }

    Ok(Frame {
        pixels,
        width,
        height,
        format,
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

/// A copy of `frame`, or an error if memory runs out
fn copy_frame(frame: &Frame) -> Result<Frame, i32> {
    let mut pixels = Vec::new();
    pixels
        .try_reserve_exact(frame.pixels.len())
        .map_err(|_| IG_STATUS_OUT_OF_MEMORY)?;
    pixels.extend_from_slice(&frame.pixels);

    Ok(Frame {
        pixels,
        width: frame.width,
        height: frame.height,
        format: frame.format,
    })
}

/// Returns true if the image has a rotation transform
fn has_rotation(image: &sys::avifImage) -> bool {
    image.transformFlags & sys::AVIF_TRANSFORM_IROT as u32 != 0
        && !image.irot.angle.is_multiple_of(4)
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

    match frame.format {
        PixelFormat::Bgra8 => {
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
        }
        // libyuv has no rotation for 64-bit pixels
        PixelFormat::Rgba16 => rotate_pixels::<8>(&frame.pixels, &mut pixels, w, h, flip, cw),
    }

    Ok(Frame {
        pixels,
        width: dst_w,
        height: dst_h,
        format: frame.format,
    })
}

/// What `ARGBRotate` does, for `N` byte pixels: flips `src` upside down if `flip`, then
/// rotates it clockwise by `cw` quarter turns into `dst`
fn rotate_pixels<const N: usize>(
    src: &[u8],
    dst: &mut [u8],
    width: u32,
    height: u32,
    flip: bool,
    cw: u8,
) {
    let (w, h) = (width as usize, height as usize);
    let dst_w = if cw % 2 == 1 { h } else { w };
    let (src, _) = src.as_chunks::<N>();
    let (dst, _) = dst.as_chunks_mut::<N>();

    for (y, row) in dst.chunks_exact_mut(dst_w).enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let (src_x, src_y) = match cw % 4 {
                1 => (y, h - 1 - x),
                2 => (w - 1 - x, h - 1 - y),
                3 => (w - 1 - y, x),
                _ => (x, y),
            };
            let src_y = if flip { h - 1 - src_y } else { src_y };
            *pixel = src[src_y * w + src_x];
        }
    }
}

/// The largest size inside `max_width` x `max_height` with the aspect ratio of `width` x `height`
fn fit_box(width: u32, height: u32, max_width: u32, max_height: u32) -> (u32, u32) {
    let scale = (max_width as f64 / width as f64).min(max_height as f64 / height as f64);
    if scale >= 1.0 {
        return (width, height);
    }

    let fit = |side: u32| ((side as f64 * scale).round() as u32).max(1);
    (fit(width), fit(height))
}

/// Shrinks `frame` to fit the box
fn scale_to_fit(
    mut frame: Frame,
    has_alpha: bool,
    max_width: u32,
    max_height: u32,
) -> Result<Frame, i32> {
    let (width, height) = fit_box(frame.width, frame.height, max_width, max_height);
    if (width, height) == (frame.width, frame.height) {
        return Ok(frame);
    }

    if frame.format != PixelFormat::Bgra8 {
        return Err(IG_STATUS_INTERNAL);
    }

    let (src_w, src_h) = (frame.width as i32, frame.height as i32);
    let (dst_w, dst_h) = (width as i32, height as i32);
    let src_plane = frame.width as usize * frame.height as usize;
    let dst_plane = width as usize * height as usize;
    let channels = if has_alpha { 4 } else { 3 };

    let mut src_planes = alloc_pixels(src_plane * channels)?;
    let mut dst_planes = alloc_pixels(dst_plane * channels)?;
    let mut pixels = alloc_pixels(dst_plane * 4)?;

    // ACTUALLY_SAFE: `frame.pixels` holds src_w x src_h and `pixels` dst_w x dst_h BGRA pixels,
    // which libyuv calls ARGB, the plane buffers hold `channels` planes of each size
    let failed = unsafe {
        let src = frame.pixels.as_mut_ptr();
        let dst = pixels.as_mut_ptr();
        let (src_base, dst_base) = (src_planes.as_mut_ptr(), dst_planes.as_mut_ptr());
        let src_ptr = |c: usize| src_base.add(c * src_plane);
        let dst_ptr = |c: usize| dst_base.add(c * dst_plane);

        let (src_a, dst_a) = if has_alpha {
            (src_ptr(3), dst_ptr(3))
        } else {
            (std::ptr::null_mut(), std::ptr::null_mut())
        };

        if has_alpha && sys::ARGBAttenuate(src, src_w * 4, src, src_w * 4, src_w, src_h) != 0 {
            return Err(IG_STATUS_INTERNAL);
        }

        sys::SplitARGBPlane(
            src,
            src_w * 4,
            src_ptr(0),
            src_w,
            src_ptr(1),
            src_w,
            src_ptr(2),
            src_w,
            src_a,
            src_w,
            src_w,
            src_h,
        );

        let scale_failed = (0..channels).any(|c| {
            sys::ScalePlane(
                src_ptr(c),
                src_w,
                src_w,
                src_h,
                dst_ptr(c),
                dst_w,
                dst_w,
                dst_h,
                sys::FILTER_BOX,
            ) != 0
        });

        if !scale_failed {
            sys::MergeARGBPlane(
                dst_ptr(0),
                dst_w,
                dst_ptr(1),
                dst_w,
                dst_ptr(2),
                dst_w,
                dst_a,
                dst_w,
                dst,
                dst_w * 4,
                dst_w,
                dst_h,
            );
        }

        scale_failed
            || (has_alpha
                && sys::ARGBUnattenuate(dst, dst_w * 4, dst, dst_w * 4, dst_w, dst_h) != 0)
    };

    if failed {
        return Err(IG_STATUS_INTERNAL);
    }

    Ok(Frame {
        pixels,
        width,
        height,
        format: PixelFormat::Bgra8,
    })
}

struct CacheEntry {
    path: String,
    stamp: (u64, u64),
    file: Arc<Mutex<AvifFile>>,
    frames: Arc<FrameCache>,
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
fn acquire(path: &str, playing: bool) -> Result<Arc<Mutex<AvifFile>>, i32> {
    let stamp = file_stamp(path)?;
    let mut cache = lock(&CACHE);

    let reused = cache
        .iter()
        .position(|e| e.path == path)
        .map(|pos| cache.remove(pos))
        .filter(|entry| entry.stamp == stamp);

    let entry = match reused {
        Some(entry) => entry,
        None => {
            let file = AvifFile::open(path)?;
            CacheEntry {
                path: path.to_owned(),
                stamp,
                frames: file.frames.clone(),
                file: Arc::new(Mutex::new(file)),
            }
        }
    };

    let file = entry.file.clone();
    cache.insert(0, entry);
    cache.truncate(MAX_CACHED_FILES);

    if playing {
        make_room(&cache);
    }

    Ok(file)
}

/// Reserves the frame cache of the first entry, releasing the others' from the oldest on
fn make_room(cache: &[CacheEntry]) {
    let Some((current, others)) = cache.split_first() else {
        return;
    };

    if !current.frames.wants_room() {
        return;
    }

    for other in others.iter().rev() {
        if current.frames.try_reserve() {
            return;
        }
        other.frames.release();
    }
    current.frames.try_reserve();
}

/// Drops every open file
pub fn clear_cache() {
    lock(&CACHE).clear();
}

/// Runs `f` on the locked file for `path`
pub fn with_file<T>(path: &str, f: impl FnOnce(&mut AvifFile) -> Result<T, i32>) -> Result<T, i32> {
    let file = acquire(path, false)?;
    let mut file = lock(&file);
    f(&mut file)
}

/// Decodes `index` frame of `path`
pub fn decode(path: &str, index: u32, animating: bool) -> Result<Frame, i32> {
    let file = acquire(path, animating && index > 0)?;

    let (frame, next) = {
        let mut guard = lock(&file);
        let format = if animating {
            PixelFormat::Bgra8
        } else {
            guard.still_format()
        };
        let frame = guard.decode_frame(index, format)?;
        let next = if animating {
            guard.plan_prefetch(index)
        } else {
            None
        };
        (frame, next)
    };

    if let Some(next) = next {
        let file = Arc::downgrade(&file);
        crate::prefetch::request(move || {
            if let Some(file) = file.upgrade() {
                lock(&file).decode_ahead(next);
            }
        });
    }

    Ok(frame)
}

/// Decodes `index` frame of `path` shrunk to fit `max_width` x `max_height`, for thumbnails
pub fn decode_scaled(
    path: &str,
    index: u32,
    max_width: u32,
    max_height: u32,
) -> Result<Frame, i32> {
    let file = acquire(path, false)?;

    let (frame, has_alpha) = {
        let mut file = lock(&file);
        let frame = file.decode_frame(index, PixelFormat::Bgra8)?;
        (frame, file.metadata().has_alpha)
    };

    scale_to_fit(frame, has_alpha, max_width, max_height)
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
    out.stride = (frame.width as usize * frame.format.bytes_per_pixel()) as i32;
    out.pixel_format = frame.format.ig_format();
    out.release_context = data.cast();

    lock(&LIVE_BUFFERS).insert(data as usize, pixels);
}

/// Frees a buffer from `hand_over`
pub fn release(data: *mut u8) {
    let pixels = lock(&LIVE_BUFFERS).remove(&(data as usize));
    drop(pixels);
}
