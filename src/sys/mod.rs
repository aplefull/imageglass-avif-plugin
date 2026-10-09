#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
mod bindings;

pub use bindings::*;

unsafe extern "C" {
    /// Rotates clockwise by `mode` degrees (0, 90, 180 or 270), a negative `height` flips the
    /// source upside down first
    pub fn ARGBRotate(
        src_argb: *const u8,
        src_stride_argb: i32,
        dst_argb: *mut u8,
        dst_stride_argb: i32,
        width: i32,
        height: i32,
        mode: i32,
    ) -> i32;

    /// Splits into one plane per channel
    pub fn SplitARGBPlane(
        src_argb: *const u8,
        src_stride_argb: i32,
        dst_r: *mut u8,
        dst_stride_r: i32,
        dst_g: *mut u8,
        dst_stride_g: i32,
        dst_b: *mut u8,
        dst_stride_b: i32,
        dst_a: *mut u8,
        dst_stride_a: i32,
        width: i32,
        height: i32,
    );

    /// Interleaves the planes back
    pub fn MergeARGBPlane(
        src_r: *const u8,
        src_stride_r: i32,
        src_g: *const u8,
        src_stride_g: i32,
        src_b: *const u8,
        src_stride_b: i32,
        src_a: *const u8,
        src_stride_a: i32,
        dst_argb: *mut u8,
        dst_stride_argb: i32,
        width: i32,
        height: i32,
    );

    /// Resizes one 8-bit plane
    pub fn ScalePlane(
        src: *const u8,
        src_stride: i32,
        src_width: i32,
        src_height: i32,
        dst: *mut u8,
        dst_stride: i32,
        dst_width: i32,
        dst_height: i32,
        filtering: i32,
    ) -> i32;

    /// Premultiplies color by alpha
    pub fn ARGBAttenuate(
        src_argb: *const u8,
        src_stride_argb: i32,
        dst_argb: *mut u8,
        dst_stride_argb: i32,
        width: i32,
        height: i32,
    ) -> i32;

    /// Undoes `ARGBAttenuate`
    pub fn ARGBUnattenuate(
        src_argb: *const u8,
        src_stride_argb: i32,
        dst_argb: *mut u8,
        dst_stride_argb: i32,
        width: i32,
        height: i32,
    ) -> i32;
}

pub const FILTER_BOX: i32 = 3;
