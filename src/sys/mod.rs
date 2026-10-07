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
}
