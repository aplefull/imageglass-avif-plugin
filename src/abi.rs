use std::ffi::c_void;

pub const IG_PLUGIN_ABI_VERSION: i32 = 1_002_000;
pub const IG_PLUGIN_ABI_MAJOR: i32 = 1;

// IGStatus
pub const IG_STATUS_OK: i32 = 0;
pub const IG_STATUS_CANCELED: i32 = 2;
pub const IG_STATUS_INVALID_ARG: i32 = 3;
pub const IG_STATUS_DECODE_FAILED: i32 = 4;
pub const IG_STATUS_OUT_OF_MEMORY: i32 = 5;
pub const IG_STATUS_INTERNAL: i32 = 6;
pub const IG_STATUS_IO_ERROR: i32 = 8;

// IGPixelFormat
pub const IG_PIXEL_FORMAT_BGRA8_UNORM: i32 = 1;

// IGColorSpace
pub const IG_COLOR_SPACE_SRGB: i32 = 1;
pub const IG_COLOR_SPACE_DISPLAY_P3: i32 = 3;
pub const IG_COLOR_SPACE_REC2020: i32 = 5;

// IGHdrTransferFn
pub const IG_HDR_TRANSFER_FN_NONE: i32 = 0;

// log levels for IGHostCoreApi.Log
pub const LOG_WARN: i32 = 3;
pub const LOG_ERROR: i32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IGStringRef {
    pub data: *const u16,
    pub length: i32,
}

impl IGStringRef {
    /// Copies the slice into a Rust string, replacing invalid UTF-16.
    ///
    /// `data` must point at `length` readable code units, or be null.
    pub unsafe fn to_string_lossy(&self) -> String {
        if self.data.is_null() || self.length <= 0 {
            return String::new();
        }
        let units = unsafe { std::slice::from_raw_parts(self.data, self.length as usize) };
        String::from_utf16_lossy(units)
    }
}

#[repr(C)]
pub struct IGPixelBuffer {
    pub data: *mut u8,
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub pixel_format: i32,
    pub release_context: *mut c_void,
}

#[repr(C)]
pub struct IGImageInfo {
    pub width: i32,
    pub height: i32,
    pub pixel_format: i32,
    pub has_alpha: i32,
    pub hdr_transfer_fn: i32,
    pub color_space: i32,
    pub orientation: i32,
    pub frame_count: i32,
    pub file_size_bytes: i64,
    pub icc_profile_data: *const u8,
    pub icc_profile_size: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IGAnimationFrameInfo {
    pub duration_ms: i32,
    pub has_alpha: i32,
}

#[repr(C)]
pub struct IGAnimationInfo {
    pub frame_count: i32,
    pub loop_count: i32,
    pub frames: *mut IGAnimationFrameInfo,
}

#[repr(C)]
pub struct IGCodecCapability {
    pub struct_size: i32,
    pub codec_id: IGStringRef,
    pub codec_name: IGStringRef,
    pub metadata_priority: i32,
    pub decode_priority: i32,
    pub supports_metadata: i32,
    pub supports_color_profiles: i32,
    pub supports_static_raster_decoding: i32,
    pub supports_animation_decoding: i32,
    pub decode_extension_count: i32,
    pub decode_extensions: *const IGStringRef,
    pub supports_static_raster_encoding: i32,
    pub supports_multi_frame_encoding: i32,
    pub encode_priority: i32,
    pub encode_extension_count: i32,
    pub encode_extensions: *const IGStringRef,
}

#[repr(C)]
pub struct IGPluginInfo {
    pub plugin_id: IGStringRef,
    pub name: IGStringRef,
    pub version: IGStringRef,
    pub abi_version: i32,
    pub codec_count: i32,
}

#[repr(C)]
pub struct IGHostCoreApi {
    pub log: Option<unsafe extern "C" fn(level: i32, message: IGStringRef)>,
    pub alloc: Option<unsafe extern "C" fn(size: usize) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(ptr: *mut c_void)>,
    pub is_cancellation_requested: Option<unsafe extern "C" fn(cancellation: *mut c_void) -> i32>,
    pub get_config_directory: Option<unsafe extern "C" fn(buffer: *mut u16, length: i32) -> i32>,
}

#[repr(C)]
pub struct IGHostApi {
    pub struct_size: i32,
    pub abi_version: i32,
    pub core: *const IGHostCoreApi,
}

pub type IGEncodeOptions = c_void;
pub type IGMultiFrameEncodeInfo = c_void;
pub type IGEncodeFrameInfo = c_void;

#[repr(C)]
pub struct IGCodecApi {
    pub struct_size: i32,
    pub get_capability: Option<unsafe extern "C" fn(out: *mut *mut IGCodecCapability) -> i32>,
    pub can_handle_extension: Option<unsafe extern "C" fn(ext: IGStringRef) -> i32>,
    pub can_handle_signature: Option<unsafe extern "C" fn(sig: *const u8, len: i32) -> i32>,
    pub load_metadata: Option<
        unsafe extern "C" fn(path: IGStringRef, out: *mut IGImageInfo, cancel: *mut c_void) -> i32,
    >,
    pub decode_static_raster: Option<
        unsafe extern "C" fn(
            path: IGStringRef,
            frame: i32,
            out: *mut IGPixelBuffer,
            cancel: *mut c_void,
        ) -> i32,
    >,
    pub free_pixel_buffer: Option<unsafe extern "C" fn(buf: *mut IGPixelBuffer)>,
    pub get_animation_info: Option<
        unsafe extern "C" fn(
            path: IGStringRef,
            out: *mut IGAnimationInfo,
            cancel: *mut c_void,
        ) -> i32,
    >,
    pub free_animation_info: Option<unsafe extern "C" fn(info: *mut IGAnimationInfo)>,
    pub decode_animation_frame: Option<
        unsafe extern "C" fn(
            path: IGStringRef,
            frame: i32,
            out: *mut IGPixelBuffer,
            cancel: *mut c_void,
        ) -> i32,
    >,
    pub encode_static_raster: Option<
        unsafe extern "C" fn(
            dest: IGStringRef,
            src: *const IGPixelBuffer,
            options: *const IGEncodeOptions,
            cancel: *mut c_void,
        ) -> i32,
    >,
    pub begin_encode_multi_frame: Option<
        unsafe extern "C" fn(
            dest: IGStringRef,
            info: *const IGMultiFrameEncodeInfo,
            options: *const IGEncodeOptions,
            out_session: *mut *mut c_void,
            cancel: *mut c_void,
        ) -> i32,
    >,
    pub encode_frame: Option<
        unsafe extern "C" fn(
            session: *mut c_void,
            frame: *const IGPixelBuffer,
            info: *const IGEncodeFrameInfo,
            cancel: *mut c_void,
        ) -> i32,
    >,
    pub end_encode_multi_frame:
        Option<unsafe extern "C" fn(session: *mut c_void, commit: i32, cancel: *mut c_void) -> i32>,
    pub decode_static_raster_scaled: Option<
        unsafe extern "C" fn(
            path: IGStringRef,
            frame: i32,
            max_width: i32,
            max_height: i32,
            out: *mut IGPixelBuffer,
            cancel: *mut c_void,
        ) -> i32,
    >,
}

#[repr(C)]
pub struct IGPluginApi {
    pub struct_size: i32,
    pub abi_version: i32,
    pub info: IGPluginInfo,
    pub get_codec: Option<unsafe extern "C" fn(index: i32, out: *mut *mut IGCodecApi) -> i32>,
    pub initialize: Option<unsafe extern "C" fn() -> i32>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub self_test: Option<unsafe extern "C" fn() -> i32>,
}
