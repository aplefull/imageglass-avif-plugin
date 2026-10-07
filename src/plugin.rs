use crate::abi::*;
use crate::{avif, host};
use std::cell::RefCell;
use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::OnceLock;

const PLUGIN_ID: &str = "Plugin_AvifCodec";
const PLUGIN_NAME: &str = "AVIF Codec";
const CODEC_ID: &str = "plugin.avif.codec";
const CODEC_NAME: &str = "AVIF (libavif + dav1d)";
const DECODE_EXTENSIONS: [&str; 2] = [".avif", ".avifs"];

const PRIORITY: i32 = 200;

static PLUGIN_API: OnceLock<usize> = OnceLock::new();
static CODEC_API: OnceLock<usize> = OnceLock::new();
static CAPABILITY: OnceLock<usize> = OnceLock::new();

thread_local! {
    static ICC_COPY: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// A UTF-16 string that lives for the rest of the process
fn leak_utf16(text: &str) -> IGStringRef {
    let units: &'static [u16] =
        Box::leak(text.encode_utf16().collect::<Vec<_>>().into_boxed_slice());
    IGStringRef {
        data: units.as_ptr(),
        length: units.len() as i32,
    }
}

/// Runs an entry point body, a panic must never unwind into the host, it would abort ImageGlass
fn guard(name: &str, body: impl FnOnce() -> i32) -> i32 {
    catch_unwind(AssertUnwindSafe(body)).unwrap_or_else(|_| {
        host::log(LOG_ERROR, &format!("AVIF: {name} panicked"));
        IG_STATUS_INTERNAL
    })
}

fn into_status(result: Result<(), i32>) -> i32 {
    result.err().unwrap_or(IG_STATUS_OK)
}

/// Entry point, this is what host is looking for
#[unsafe(no_mangle)]
pub extern "C" fn ig_plugin_get_api(
    host_abi_version: i32,
    host_api: *const IGHostApi,
) -> *const IGPluginApi {
    let result = catch_unwind(|| {
        if host_abi_version / 1_000_000 != IG_PLUGIN_ABI_MAJOR || host_api.is_null() {
            return std::ptr::null();
        }

        host::set_host_api(host_api);
        *PLUGIN_API.get_or_init(|| build_plugin_api() as usize) as *const IGPluginApi
    });

    result.unwrap_or(std::ptr::null())
}

fn build_plugin_api() -> *mut IGPluginApi {
    let api = IGPluginApi {
        struct_size: size_of::<IGPluginApi>() as i32,
        abi_version: IG_PLUGIN_ABI_VERSION,
        info: IGPluginInfo {
            plugin_id: leak_utf16(PLUGIN_ID),
            name: leak_utf16(PLUGIN_NAME),
            version: leak_utf16(env!("CARGO_PKG_VERSION")),
            abi_version: IG_PLUGIN_ABI_VERSION,
            codec_count: 1,
        },
        get_codec: Some(get_codec),
        initialize: None,
        shutdown: Some(shutdown),
        self_test: None,
    };

    Box::into_raw(Box::new(api))
}

fn build_codec_api() -> *mut IGCodecApi {
    let api = IGCodecApi {
        struct_size: size_of::<IGCodecApi>() as i32,
        get_capability: Some(get_capability),
        can_handle_extension: Some(can_handle_extension),
        can_handle_signature: Some(can_handle_signature),
        load_metadata: Some(load_metadata),
        decode_static_raster: Some(decode_static_raster),
        free_pixel_buffer: Some(free_pixel_buffer),
        get_animation_info: Some(get_animation_info),
        free_animation_info: Some(free_animation_info),
        decode_animation_frame: Some(decode_animation_frame),

        encode_static_raster: None,
        begin_encode_multi_frame: None,
        encode_frame: None,
        end_encode_multi_frame: None,
        decode_static_raster_scaled: None,
    };

    Box::into_raw(Box::new(api))
}

fn build_capability() -> *mut IGCodecCapability {
    let extensions: &'static [IGStringRef] = Box::leak(
        DECODE_EXTENSIONS
            .iter()
            .map(|e| leak_utf16(e))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );

    let capability = IGCodecCapability {
        struct_size: size_of::<IGCodecCapability>() as i32,
        codec_id: leak_utf16(CODEC_ID),
        codec_name: leak_utf16(CODEC_NAME),
        metadata_priority: PRIORITY,
        decode_priority: PRIORITY,
        supports_metadata: 1,
        supports_color_profiles: 1,
        supports_static_raster_decoding: 1,
        supports_animation_decoding: 1,
        decode_extension_count: extensions.len() as i32,
        decode_extensions: extensions.as_ptr(),
        supports_static_raster_encoding: 0,
        supports_multi_frame_encoding: 0,
        encode_priority: 0,
        encode_extension_count: 0,
        encode_extensions: std::ptr::null(),
    };

    Box::into_raw(Box::new(capability))
}

unsafe extern "C" fn get_codec(index: i32, out: *mut *mut IGCodecApi) -> i32 {
    guard("GetCodec", || {
        if out.is_null() {
            return IG_STATUS_INVALID_ARG;
        }

        // ACTUALLY_SAFE: checked non-null above
        unsafe {
            if index != 0 {
                *out = std::ptr::null_mut();
                return IG_STATUS_INVALID_ARG;
            }
            *out = *CODEC_API.get_or_init(|| build_codec_api() as usize) as *mut IGCodecApi;
        }
        IG_STATUS_OK
    })
}

unsafe extern "C" fn shutdown() {
    let _ = catch_unwind(avif::clear_cache);
}

unsafe extern "C" fn get_capability(out: *mut *mut IGCodecCapability) -> i32 {
    guard("GetCapability", || {
        if out.is_null() {
            return IG_STATUS_INVALID_ARG;
        }

        // ACTUALLY_SAFE: checked non-null above
        unsafe {
            *out = *CAPABILITY.get_or_init(|| build_capability() as usize) as *mut IGCodecCapability
        };
        IG_STATUS_OK
    })
}

unsafe extern "C" fn can_handle_extension(ext: IGStringRef) -> i32 {
    guard("CanHandleExtension", || {
        // ACTUALLY_SAFE: the host passes a valid slice for the duration of the call
        let ext = unsafe { ext.to_string_lossy() }.to_ascii_lowercase();
        DECODE_EXTENSIONS.contains(&ext.as_str()) as i32
    })
}

unsafe extern "C" fn can_handle_signature(sig: *const u8, len: i32) -> i32 {
    guard("CanHandleSignature", || {
        if sig.is_null() || len <= 0 {
            return 0;
        }

        // ACTUALLY_SAFE: the host passes `len` readable bytes for the duration of the call
        let header = libavif_sys::avifROData {
            data: sig,
            size: len as usize,
        };
        (unsafe { libavif_sys::avifPeekCompatibleFileType(&header) } != 0) as i32
    })
}

unsafe extern "C" fn load_metadata(
    path: IGStringRef,
    out: *mut IGImageInfo,
    cancel: *mut c_void,
) -> i32 {
    guard("LoadMetadata", || {
        if out.is_null() {
            return IG_STATUS_INVALID_ARG;
        }

        if host::is_canceled(cancel) {
            return IG_STATUS_CANCELED;
        }

        // ACTUALLY_SAFE: the host passes a valid path slice and output struct for the call
        let (path, out) = unsafe { (path.to_string_lossy(), &mut *out) };

        into_status(avif::with_file(&path, |file| {
            let meta = file.metadata();
            let icc = ICC_COPY.with_borrow_mut(|copy| {
                copy.clear();
                copy.extend_from_slice(file.icc_profile());
                (copy.as_ptr(), copy.len())
            });

            *out = IGImageInfo {
                width: meta.width as i32,
                height: meta.height as i32,
                pixel_format: IG_PIXEL_FORMAT_BGRA8_UNORM,
                has_alpha: meta.has_alpha as i32,
                hdr_transfer_fn: IG_HDR_TRANSFER_FN_NONE,
                color_space: meta.color_space,
                // rotation and mirroring are already applied
                orientation: 1,
                frame_count: meta.frame_count as i32,
                file_size_bytes: meta.file_size as i64,
                icc_profile_data: if icc.1 > 0 { icc.0 } else { std::ptr::null() },
                icc_profile_size: icc.1 as i32,
            };
            Ok(())
        }))
    })
}

fn decode_into(
    name: &str,
    path: IGStringRef,
    frame_index: i32,
    out: *mut IGPixelBuffer,
    cancel: *mut c_void,
) -> i32 {
    guard(name, || {
        if out.is_null() || frame_index < 0 {
            return IG_STATUS_INVALID_ARG;
        }

        if host::is_canceled(cancel) {
            return IG_STATUS_CANCELED;
        }

        // ACTUALLY_SAFE: the host passes a valid path slice and output struct for the call
        let (path, out) = unsafe { (path.to_string_lossy(), &mut *out) };

        into_status(avif::with_file(&path, |file| {
            let frame = file.decode_frame(frame_index as u32)?;
            avif::hand_over(frame, out);
            Ok(())
        }))
    })
}

unsafe extern "C" fn decode_static_raster(
    path: IGStringRef,
    frame: i32,
    out: *mut IGPixelBuffer,
    cancel: *mut c_void,
) -> i32 {
    decode_into("DecodeStaticRaster", path, frame, out, cancel)
}

unsafe extern "C" fn decode_animation_frame(
    path: IGStringRef,
    frame: i32,
    out: *mut IGPixelBuffer,
    cancel: *mut c_void,
) -> i32 {
    decode_into("DecodeAnimationFrame", path, frame, out, cancel)
}

unsafe extern "C" fn free_pixel_buffer(buf: *mut IGPixelBuffer) {
    let _ = catch_unwind(|| {
        // ACTUALLY_SAFE: the host passes back a buffer this plugin filled, or null
        let Some(buf) = (unsafe { buf.as_mut() }) else {
            return;
        };
        if buf.data.is_null() {
            return;
        }

        avif::release(buf.data);
        buf.data = std::ptr::null_mut();
        buf.release_context = std::ptr::null_mut();
    });
}

unsafe extern "C" fn get_animation_info(
    path: IGStringRef,
    out: *mut IGAnimationInfo,
    cancel: *mut c_void,
) -> i32 {
    guard("GetAnimationInfo", || {
        if out.is_null() {
            return IG_STATUS_INVALID_ARG;
        }

        if host::is_canceled(cancel) {
            return IG_STATUS_CANCELED;
        }

        // ACTUALLY_SAFE: the host passes a valid path slice and output struct for the call
        let (path, out) = unsafe { (path.to_string_lossy(), &mut *out) };

        into_status(avif::with_file(&path, |file| {
            let (durations, loop_count) = file.animation_timing();
            let has_alpha = file.metadata().has_alpha as i32;

            let frames: Box<[IGAnimationFrameInfo]> = durations
                .iter()
                .map(|&duration_ms| IGAnimationFrameInfo {
                    duration_ms,
                    has_alpha,
                })
                .collect();

            out.frame_count = frames.len() as i32;
            out.loop_count = loop_count;
            out.frames = Box::into_raw(frames).cast();
            Ok(())
        }))
    })
}

unsafe extern "C" fn free_animation_info(info: *mut IGAnimationInfo) {
    let _ = catch_unwind(|| {
        // ACTUALLY_SAFE: the host passes back the struct GetAnimationInfo filled, or null
        let Some(info) = (unsafe { info.as_mut() }) else {
            return;
        };
        if info.frames.is_null() || info.frame_count <= 0 {
            return;
        }

        // ACTUALLY_SAFE: allocated in get_animation_info as a boxed slice of frame_count items
        unsafe {
            let slice = std::ptr::slice_from_raw_parts_mut(info.frames, info.frame_count as usize);
            drop(Box::from_raw(slice));
        }
        info.frames = std::ptr::null_mut();
        info.frame_count = 0;
    });
}
