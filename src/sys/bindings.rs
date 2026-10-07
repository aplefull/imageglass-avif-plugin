// bindgen output for libavif v1.4.2

pub const AVIF_VERSION_MAJOR: u32 = 1;
pub const AVIF_VERSION_MINOR: u32 = 4;
pub const AVIF_VERSION_PATCH: u32 = 2;
pub const AVIF_VERSION_DEVEL: u32 = 0;
pub const AVIF_VERSION: u32 = 1040200;
pub const AVIF_TRUE: u32 = 1;
pub const AVIF_FALSE: u32 = 0;
pub const AVIF_DIAGNOSTICS_ERROR_BUFFER_SIZE: u32 = 256;
pub const AVIF_DEFAULT_IMAGE_SIZE_LIMIT: u32 = 268435456;
pub const AVIF_DEFAULT_IMAGE_DIMENSION_LIMIT: u32 = 32768;
pub const AVIF_DEFAULT_IMAGE_COUNT_LIMIT: u32 = 2592000;
pub const AVIF_QUALITY_DEFAULT: i32 = -1;
pub const AVIF_QUALITY_WORST: u32 = 0;
pub const AVIF_QUALITY_BEST: u32 = 100;
pub const AVIF_QUALITY_LOSSLESS: u32 = 100;
pub const AVIF_QUANTIZER_LOSSLESS: u32 = 0;
pub const AVIF_QUANTIZER_BEST_QUALITY: u32 = 0;
pub const AVIF_QUANTIZER_WORST_QUALITY: u32 = 63;
pub const AVIF_PLANE_COUNT_YUV: u32 = 3;
pub const AVIF_SPEED_DEFAULT: i32 = -1;
pub const AVIF_SPEED_SLOWEST: u32 = 0;
pub const AVIF_SPEED_FASTEST: u32 = 10;
pub const AVIF_REPETITION_COUNT_INFINITE: i32 = -1;
pub const AVIF_REPETITION_COUNT_UNKNOWN: i32 = -2;
pub const AVIF_MAX_AV1_LAYER_COUNT: u32 = 4;
pub type avifBool = ::std::os::raw::c_int;
pub const AVIF_PLANES_YUV: avifPlanesFlag = 1;
pub const AVIF_PLANES_A: avifPlanesFlag = 2;
pub const AVIF_PLANES_ALL: avifPlanesFlag = 255;
pub type avifPlanesFlag = ::std::os::raw::c_int;
pub type avifPlanesFlags = u32;
pub const AVIF_CHAN_Y: avifChannelIndex = 0;
pub const AVIF_CHAN_U: avifChannelIndex = 1;
pub const AVIF_CHAN_V: avifChannelIndex = 2;
pub const AVIF_CHAN_A: avifChannelIndex = 3;
pub type avifChannelIndex = ::std::os::raw::c_int;
unsafe extern "C" {
    pub fn avifVersion() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn avifCodecVersions(outBuffer: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn avifLibYUVVersion() -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn avifAlloc(size: usize) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn avifFree(p: *mut ::std::os::raw::c_void);
}
pub const AVIF_RESULT_OK: avifResult = 0;
pub const AVIF_RESULT_UNKNOWN_ERROR: avifResult = 1;
pub const AVIF_RESULT_INVALID_FTYP: avifResult = 2;
pub const AVIF_RESULT_NO_CONTENT: avifResult = 3;
pub const AVIF_RESULT_NO_YUV_FORMAT_SELECTED: avifResult = 4;
pub const AVIF_RESULT_REFORMAT_FAILED: avifResult = 5;
pub const AVIF_RESULT_UNSUPPORTED_DEPTH: avifResult = 6;
pub const AVIF_RESULT_ENCODE_COLOR_FAILED: avifResult = 7;
pub const AVIF_RESULT_ENCODE_ALPHA_FAILED: avifResult = 8;
pub const AVIF_RESULT_BMFF_PARSE_FAILED: avifResult = 9;
pub const AVIF_RESULT_MISSING_IMAGE_ITEM: avifResult = 10;
pub const AVIF_RESULT_DECODE_COLOR_FAILED: avifResult = 11;
pub const AVIF_RESULT_DECODE_ALPHA_FAILED: avifResult = 12;
pub const AVIF_RESULT_COLOR_ALPHA_SIZE_MISMATCH: avifResult = 13;
pub const AVIF_RESULT_ISPE_SIZE_MISMATCH: avifResult = 14;
pub const AVIF_RESULT_NO_CODEC_AVAILABLE: avifResult = 15;
pub const AVIF_RESULT_NO_IMAGES_REMAINING: avifResult = 16;
pub const AVIF_RESULT_INVALID_EXIF_PAYLOAD: avifResult = 17;
pub const AVIF_RESULT_INVALID_IMAGE_GRID: avifResult = 18;
pub const AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION: avifResult = 19;
pub const AVIF_RESULT_TRUNCATED_DATA: avifResult = 20;
pub const AVIF_RESULT_IO_NOT_SET: avifResult = 21;
pub const AVIF_RESULT_IO_ERROR: avifResult = 22;
pub const AVIF_RESULT_WAITING_ON_IO: avifResult = 23;
pub const AVIF_RESULT_INVALID_ARGUMENT: avifResult = 24;
pub const AVIF_RESULT_NOT_IMPLEMENTED: avifResult = 25;
pub const AVIF_RESULT_OUT_OF_MEMORY: avifResult = 26;
pub const AVIF_RESULT_CANNOT_CHANGE_SETTING: avifResult = 27;
pub const AVIF_RESULT_INCOMPATIBLE_IMAGE: avifResult = 28;
pub const AVIF_RESULT_INTERNAL_ERROR: avifResult = 29;
pub const AVIF_RESULT_ENCODE_GAIN_MAP_FAILED: avifResult = 30;
pub const AVIF_RESULT_DECODE_GAIN_MAP_FAILED: avifResult = 31;
pub const AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE: avifResult = 32;
pub const AVIF_RESULT_ENCODE_SAMPLE_TRANSFORM_FAILED: avifResult = 33;
pub const AVIF_RESULT_DECODE_SAMPLE_TRANSFORM_FAILED: avifResult = 34;
pub const AVIF_RESULT_NO_AV1_ITEMS_FOUND: avifResult = 10;
pub type avifResult = ::std::os::raw::c_int;
unsafe extern "C" {
    pub fn avifResultToString(result: avifResult) -> *const ::std::os::raw::c_char;
}
pub const AVIF_HEADER_DEFAULT: avifHeaderFormat = 0;
pub const AVIF_HEADER_FULL: avifHeaderFormat = 0;
pub type avifHeaderFormat = ::std::os::raw::c_int;
pub type avifHeaderFormatFlags = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifROData {
    pub data: *const u8,
    pub size: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifROData"][::std::mem::size_of::<avifROData>() - 16usize];
    ["Alignment of avifROData"][::std::mem::align_of::<avifROData>() - 8usize];
    ["Offset of field: avifROData::data"][::std::mem::offset_of!(avifROData, data) - 0usize];
    ["Offset of field: avifROData::size"][::std::mem::offset_of!(avifROData, size) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifRWData {
    pub data: *mut u8,
    pub size: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifRWData"][::std::mem::size_of::<avifRWData>() - 16usize];
    ["Alignment of avifRWData"][::std::mem::align_of::<avifRWData>() - 8usize];
    ["Offset of field: avifRWData::data"][::std::mem::offset_of!(avifRWData, data) - 0usize];
    ["Offset of field: avifRWData::size"][::std::mem::offset_of!(avifRWData, size) - 8usize];
};
unsafe extern "C" {
    pub fn avifRWDataRealloc(raw: *mut avifRWData, newSize: usize) -> avifResult;
}
unsafe extern "C" {
    pub fn avifRWDataSet(raw: *mut avifRWData, data: *const u8, len: usize) -> avifResult;
}
unsafe extern "C" {
    pub fn avifRWDataFree(raw: *mut avifRWData);
}
unsafe extern "C" {
    pub fn avifGetExifTiffHeaderOffset(
        exif: *const u8,
        exifSize: usize,
        offset: *mut usize,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifGetExifOrientationOffset(
        exif: *const u8,
        exifSize: usize,
        offset: *mut usize,
    ) -> avifResult;
}
pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YUV400: avifPixelFormat = 4;
pub const AVIF_PIXEL_FORMAT_COUNT: avifPixelFormat = 5;
pub type avifPixelFormat = ::std::os::raw::c_int;
unsafe extern "C" {
    pub fn avifPixelFormatToString(format: avifPixelFormat) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifPixelFormatInfo {
    pub monochrome: avifBool,
    pub chromaShiftX: ::std::os::raw::c_int,
    pub chromaShiftY: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifPixelFormatInfo"][::std::mem::size_of::<avifPixelFormatInfo>() - 12usize];
    ["Alignment of avifPixelFormatInfo"][::std::mem::align_of::<avifPixelFormatInfo>() - 4usize];
    ["Offset of field: avifPixelFormatInfo::monochrome"]
        [::std::mem::offset_of!(avifPixelFormatInfo, monochrome) - 0usize];
    ["Offset of field: avifPixelFormatInfo::chromaShiftX"]
        [::std::mem::offset_of!(avifPixelFormatInfo, chromaShiftX) - 4usize];
    ["Offset of field: avifPixelFormatInfo::chromaShiftY"]
        [::std::mem::offset_of!(avifPixelFormatInfo, chromaShiftY) - 8usize];
};
unsafe extern "C" {
    pub fn avifGetPixelFormatInfo(format: avifPixelFormat, info: *mut avifPixelFormatInfo);
}
pub const AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN: avifChromaSamplePosition = 0;
pub const AVIF_CHROMA_SAMPLE_POSITION_VERTICAL: avifChromaSamplePosition = 1;
pub const AVIF_CHROMA_SAMPLE_POSITION_COLOCATED: avifChromaSamplePosition = 2;
pub const AVIF_CHROMA_SAMPLE_POSITION_RESERVED: avifChromaSamplePosition = 3;
pub type avifChromaSamplePosition = ::std::os::raw::c_int;
#[doc = "<- Y  [16..235],  UV  [16..240]  (bit depth 8) */\n/**<- Y  [64..940],  UV  [64..960]  (bit depth 10) */\n/**<- Y [256..3760], UV [256..3840] (bit depth 12)"]
pub const AVIF_RANGE_LIMITED: avifRange = 0;
#[doc = "<- [0..255]  (bit depth 8) */\n/**<- [0..1023] (bit depth 10) */\n/**<- [0..4095] (bit depth 12)"]
pub const AVIF_RANGE_FULL: avifRange = 1;
pub type avifRange = ::std::os::raw::c_int;
pub const AVIF_COLOR_PRIMARIES_UNKNOWN: _bindgen_ty_1 = 0;
pub const AVIF_COLOR_PRIMARIES_BT709: _bindgen_ty_1 = 1;
pub const AVIF_COLOR_PRIMARIES_SRGB: _bindgen_ty_1 = 1;
pub const AVIF_COLOR_PRIMARIES_IEC61966_2_4: _bindgen_ty_1 = 1;
pub const AVIF_COLOR_PRIMARIES_UNSPECIFIED: _bindgen_ty_1 = 2;
pub const AVIF_COLOR_PRIMARIES_BT470M: _bindgen_ty_1 = 4;
pub const AVIF_COLOR_PRIMARIES_BT470BG: _bindgen_ty_1 = 5;
pub const AVIF_COLOR_PRIMARIES_BT601: _bindgen_ty_1 = 6;
pub const AVIF_COLOR_PRIMARIES_SMPTE240: _bindgen_ty_1 = 7;
pub const AVIF_COLOR_PRIMARIES_GENERIC_FILM: _bindgen_ty_1 = 8;
pub const AVIF_COLOR_PRIMARIES_BT2020: _bindgen_ty_1 = 9;
pub const AVIF_COLOR_PRIMARIES_BT2100: _bindgen_ty_1 = 9;
pub const AVIF_COLOR_PRIMARIES_XYZ: _bindgen_ty_1 = 10;
pub const AVIF_COLOR_PRIMARIES_SMPTE431: _bindgen_ty_1 = 11;
pub const AVIF_COLOR_PRIMARIES_SMPTE432: _bindgen_ty_1 = 12;
pub const AVIF_COLOR_PRIMARIES_DCI_P3: _bindgen_ty_1 = 12;
pub const AVIF_COLOR_PRIMARIES_EBU3213: _bindgen_ty_1 = 22;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub type avifColorPrimaries = u16;
unsafe extern "C" {
    pub fn avifColorPrimariesGetValues(acp: avifColorPrimaries, outPrimaries: *mut f32);
}
unsafe extern "C" {
    pub fn avifColorPrimariesFind(
        inPrimaries: *const f32,
        outName: *mut *const ::std::os::raw::c_char,
    ) -> avifColorPrimaries;
}
pub const AVIF_TRANSFER_CHARACTERISTICS_UNKNOWN: _bindgen_ty_2 = 0;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT709: _bindgen_ty_2 = 1;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED: _bindgen_ty_2 = 2;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470M: _bindgen_ty_2 = 4;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470BG: _bindgen_ty_2 = 5;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT601: _bindgen_ty_2 = 6;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE240: _bindgen_ty_2 = 7;
pub const AVIF_TRANSFER_CHARACTERISTICS_LINEAR: _bindgen_ty_2 = 8;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100: _bindgen_ty_2 = 9;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: _bindgen_ty_2 = 10;
pub const AVIF_TRANSFER_CHARACTERISTICS_IEC61966: _bindgen_ty_2 = 11;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT1361: _bindgen_ty_2 = 12;
pub const AVIF_TRANSFER_CHARACTERISTICS_SRGB: _bindgen_ty_2 = 13;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_10BIT: _bindgen_ty_2 = 14;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_12BIT: _bindgen_ty_2 = 15;
pub const AVIF_TRANSFER_CHARACTERISTICS_PQ: _bindgen_ty_2 = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084: _bindgen_ty_2 = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE428: _bindgen_ty_2 = 17;
pub const AVIF_TRANSFER_CHARACTERISTICS_HLG: _bindgen_ty_2 = 18;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
pub type avifTransferCharacteristics = u16;
unsafe extern "C" {
    pub fn avifTransferCharacteristicsGetGamma(
        atc: avifTransferCharacteristics,
        gamma: *mut f32,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifTransferCharacteristicsFindByGamma(gamma: f32) -> avifTransferCharacteristics;
}
pub const AVIF_MATRIX_COEFFICIENTS_IDENTITY: _bindgen_ty_3 = 0;
pub const AVIF_MATRIX_COEFFICIENTS_BT709: _bindgen_ty_3 = 1;
pub const AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED: _bindgen_ty_3 = 2;
pub const AVIF_MATRIX_COEFFICIENTS_FCC: _bindgen_ty_3 = 4;
pub const AVIF_MATRIX_COEFFICIENTS_BT470BG: _bindgen_ty_3 = 5;
pub const AVIF_MATRIX_COEFFICIENTS_BT601: _bindgen_ty_3 = 6;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE240: _bindgen_ty_3 = 7;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO: _bindgen_ty_3 = 8;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_NCL: _bindgen_ty_3 = 9;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_CL: _bindgen_ty_3 = 10;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE2085: _bindgen_ty_3 = 11;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: _bindgen_ty_3 = 12;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: _bindgen_ty_3 = 13;
pub const AVIF_MATRIX_COEFFICIENTS_ICTCP: _bindgen_ty_3 = 14;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RE: _bindgen_ty_3 = 16;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RO: _bindgen_ty_3 = 17;
pub const AVIF_MATRIX_COEFFICIENTS_LAST: _bindgen_ty_3 = 18;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
pub type avifMatrixCoefficients = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifDiagnostics {
    pub error: [::std::os::raw::c_char; 256usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifDiagnostics"][::std::mem::size_of::<avifDiagnostics>() - 256usize];
    ["Alignment of avifDiagnostics"][::std::mem::align_of::<avifDiagnostics>() - 1usize];
    ["Offset of field: avifDiagnostics::error"]
        [::std::mem::offset_of!(avifDiagnostics, error) - 0usize];
};
unsafe extern "C" {
    pub fn avifDiagnosticsClearError(diag: *mut avifDiagnostics);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifFraction {
    pub n: i32,
    pub d: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifFraction"][::std::mem::size_of::<avifFraction>() - 8usize];
    ["Alignment of avifFraction"][::std::mem::align_of::<avifFraction>() - 4usize];
    ["Offset of field: avifFraction::n"][::std::mem::offset_of!(avifFraction, n) - 0usize];
    ["Offset of field: avifFraction::d"][::std::mem::offset_of!(avifFraction, d) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifSignedFraction {
    pub n: i32,
    pub d: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifSignedFraction"][::std::mem::size_of::<avifSignedFraction>() - 8usize];
    ["Alignment of avifSignedFraction"][::std::mem::align_of::<avifSignedFraction>() - 4usize];
    ["Offset of field: avifSignedFraction::n"]
        [::std::mem::offset_of!(avifSignedFraction, n) - 0usize];
    ["Offset of field: avifSignedFraction::d"]
        [::std::mem::offset_of!(avifSignedFraction, d) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifUnsignedFraction {
    pub n: u32,
    pub d: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifUnsignedFraction"][::std::mem::size_of::<avifUnsignedFraction>() - 8usize];
    ["Alignment of avifUnsignedFraction"][::std::mem::align_of::<avifUnsignedFraction>() - 4usize];
    ["Offset of field: avifUnsignedFraction::n"]
        [::std::mem::offset_of!(avifUnsignedFraction, n) - 0usize];
    ["Offset of field: avifUnsignedFraction::d"]
        [::std::mem::offset_of!(avifUnsignedFraction, d) - 4usize];
};
unsafe extern "C" {
    pub fn avifDoubleToSignedFraction(v: f64, fraction: *mut avifSignedFraction) -> avifBool;
}
unsafe extern "C" {
    pub fn avifDoubleToUnsignedFraction(v: f64, fraction: *mut avifUnsignedFraction) -> avifBool;
}
pub const AVIF_TRANSFORM_NONE: avifTransformFlag = 0;
pub const AVIF_TRANSFORM_PASP: avifTransformFlag = 1;
pub const AVIF_TRANSFORM_CLAP: avifTransformFlag = 2;
pub const AVIF_TRANSFORM_IROT: avifTransformFlag = 4;
pub const AVIF_TRANSFORM_IMIR: avifTransformFlag = 8;
pub type avifTransformFlag = ::std::os::raw::c_int;
pub type avifTransformFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifPixelAspectRatioBox {
    pub hSpacing: u32,
    pub vSpacing: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifPixelAspectRatioBox"][::std::mem::size_of::<avifPixelAspectRatioBox>() - 8usize];
    ["Alignment of avifPixelAspectRatioBox"]
        [::std::mem::align_of::<avifPixelAspectRatioBox>() - 4usize];
    ["Offset of field: avifPixelAspectRatioBox::hSpacing"]
        [::std::mem::offset_of!(avifPixelAspectRatioBox, hSpacing) - 0usize];
    ["Offset of field: avifPixelAspectRatioBox::vSpacing"]
        [::std::mem::offset_of!(avifPixelAspectRatioBox, vSpacing) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifCleanApertureBox {
    pub widthN: u32,
    pub widthD: u32,
    pub heightN: u32,
    pub heightD: u32,
    pub horizOffN: u32,
    pub horizOffD: u32,
    pub vertOffN: u32,
    pub vertOffD: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifCleanApertureBox"][::std::mem::size_of::<avifCleanApertureBox>() - 32usize];
    ["Alignment of avifCleanApertureBox"][::std::mem::align_of::<avifCleanApertureBox>() - 4usize];
    ["Offset of field: avifCleanApertureBox::widthN"]
        [::std::mem::offset_of!(avifCleanApertureBox, widthN) - 0usize];
    ["Offset of field: avifCleanApertureBox::widthD"]
        [::std::mem::offset_of!(avifCleanApertureBox, widthD) - 4usize];
    ["Offset of field: avifCleanApertureBox::heightN"]
        [::std::mem::offset_of!(avifCleanApertureBox, heightN) - 8usize];
    ["Offset of field: avifCleanApertureBox::heightD"]
        [::std::mem::offset_of!(avifCleanApertureBox, heightD) - 12usize];
    ["Offset of field: avifCleanApertureBox::horizOffN"]
        [::std::mem::offset_of!(avifCleanApertureBox, horizOffN) - 16usize];
    ["Offset of field: avifCleanApertureBox::horizOffD"]
        [::std::mem::offset_of!(avifCleanApertureBox, horizOffD) - 20usize];
    ["Offset of field: avifCleanApertureBox::vertOffN"]
        [::std::mem::offset_of!(avifCleanApertureBox, vertOffN) - 24usize];
    ["Offset of field: avifCleanApertureBox::vertOffD"]
        [::std::mem::offset_of!(avifCleanApertureBox, vertOffD) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifImageRotation {
    pub angle: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifImageRotation"][::std::mem::size_of::<avifImageRotation>() - 1usize];
    ["Alignment of avifImageRotation"][::std::mem::align_of::<avifImageRotation>() - 1usize];
    ["Offset of field: avifImageRotation::angle"]
        [::std::mem::offset_of!(avifImageRotation, angle) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifImageMirror {
    pub axis: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifImageMirror"][::std::mem::size_of::<avifImageMirror>() - 1usize];
    ["Alignment of avifImageMirror"][::std::mem::align_of::<avifImageMirror>() - 1usize];
    ["Offset of field: avifImageMirror::axis"]
        [::std::mem::offset_of!(avifImageMirror, axis) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifCropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifCropRect"][::std::mem::size_of::<avifCropRect>() - 16usize];
    ["Alignment of avifCropRect"][::std::mem::align_of::<avifCropRect>() - 4usize];
    ["Offset of field: avifCropRect::x"][::std::mem::offset_of!(avifCropRect, x) - 0usize];
    ["Offset of field: avifCropRect::y"][::std::mem::offset_of!(avifCropRect, y) - 4usize];
    ["Offset of field: avifCropRect::width"][::std::mem::offset_of!(avifCropRect, width) - 8usize];
    ["Offset of field: avifCropRect::height"]
        [::std::mem::offset_of!(avifCropRect, height) - 12usize];
};
unsafe extern "C" {
    pub fn avifCropRectFromCleanApertureBox(
        cropRect: *mut avifCropRect,
        clap: *const avifCleanApertureBox,
        imageW: u32,
        imageH: u32,
        diag: *mut avifDiagnostics,
    ) -> avifBool;
}
unsafe extern "C" {
    pub fn avifCleanApertureBoxFromCropRect(
        clap: *mut avifCleanApertureBox,
        cropRect: *const avifCropRect,
        imageW: u32,
        imageH: u32,
        diag: *mut avifDiagnostics,
    ) -> avifBool;
}
unsafe extern "C" {
    pub fn avifCropRectRequiresUpsampling(
        cropRect: *const avifCropRect,
        yuvFormat: avifPixelFormat,
    ) -> avifBool;
}
unsafe extern "C" {
    pub fn avifCropRectConvertCleanApertureBox(
        arg1: *mut avifCropRect,
        arg2: *const avifCleanApertureBox,
        arg3: u32,
        arg4: u32,
        arg5: avifPixelFormat,
        arg6: *mut avifDiagnostics,
    ) -> avifBool;
}
unsafe extern "C" {
    pub fn avifCleanApertureBoxConvertCropRect(
        arg1: *mut avifCleanApertureBox,
        arg2: *const avifCropRect,
        arg3: u32,
        arg4: u32,
        arg5: avifPixelFormat,
        arg6: *mut avifDiagnostics,
    ) -> avifBool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifContentLightLevelInformationBox {
    pub maxCLL: u16,
    pub maxPALL: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifContentLightLevelInformationBox"]
        [::std::mem::size_of::<avifContentLightLevelInformationBox>() - 4usize];
    ["Alignment of avifContentLightLevelInformationBox"]
        [::std::mem::align_of::<avifContentLightLevelInformationBox>() - 2usize];
    ["Offset of field: avifContentLightLevelInformationBox::maxCLL"]
        [::std::mem::offset_of!(avifContentLightLevelInformationBox, maxCLL) - 0usize];
    ["Offset of field: avifContentLightLevelInformationBox::maxPALL"]
        [::std::mem::offset_of!(avifContentLightLevelInformationBox, maxPALL) - 2usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifGainMap {
    pub image: *mut avifImage,
    pub gainMapMin: [avifSignedFraction; 3usize],
    pub gainMapMax: [avifSignedFraction; 3usize],
    pub gainMapGamma: [avifUnsignedFraction; 3usize],
    pub baseOffset: [avifSignedFraction; 3usize],
    pub alternateOffset: [avifSignedFraction; 3usize],
    pub baseHdrHeadroom: avifUnsignedFraction,
    pub alternateHdrHeadroom: avifUnsignedFraction,
    pub useBaseColorSpace: avifBool,
    pub altICC: avifRWData,
    pub altColorPrimaries: avifColorPrimaries,
    pub altTransferCharacteristics: avifTransferCharacteristics,
    pub altMatrixCoefficients: avifMatrixCoefficients,
    pub altYUVRange: avifRange,
    pub altDepth: u32,
    pub altPlaneCount: u32,
    pub altCLLI: avifContentLightLevelInformationBox,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifGainMap"][::std::mem::size_of::<avifGainMap>() - 192usize];
    ["Alignment of avifGainMap"][::std::mem::align_of::<avifGainMap>() - 8usize];
    ["Offset of field: avifGainMap::image"][::std::mem::offset_of!(avifGainMap, image) - 0usize];
    ["Offset of field: avifGainMap::gainMapMin"]
        [::std::mem::offset_of!(avifGainMap, gainMapMin) - 8usize];
    ["Offset of field: avifGainMap::gainMapMax"]
        [::std::mem::offset_of!(avifGainMap, gainMapMax) - 32usize];
    ["Offset of field: avifGainMap::gainMapGamma"]
        [::std::mem::offset_of!(avifGainMap, gainMapGamma) - 56usize];
    ["Offset of field: avifGainMap::baseOffset"]
        [::std::mem::offset_of!(avifGainMap, baseOffset) - 80usize];
    ["Offset of field: avifGainMap::alternateOffset"]
        [::std::mem::offset_of!(avifGainMap, alternateOffset) - 104usize];
    ["Offset of field: avifGainMap::baseHdrHeadroom"]
        [::std::mem::offset_of!(avifGainMap, baseHdrHeadroom) - 128usize];
    ["Offset of field: avifGainMap::alternateHdrHeadroom"]
        [::std::mem::offset_of!(avifGainMap, alternateHdrHeadroom) - 136usize];
    ["Offset of field: avifGainMap::useBaseColorSpace"]
        [::std::mem::offset_of!(avifGainMap, useBaseColorSpace) - 144usize];
    ["Offset of field: avifGainMap::altICC"]
        [::std::mem::offset_of!(avifGainMap, altICC) - 152usize];
    ["Offset of field: avifGainMap::altColorPrimaries"]
        [::std::mem::offset_of!(avifGainMap, altColorPrimaries) - 168usize];
    ["Offset of field: avifGainMap::altTransferCharacteristics"]
        [::std::mem::offset_of!(avifGainMap, altTransferCharacteristics) - 170usize];
    ["Offset of field: avifGainMap::altMatrixCoefficients"]
        [::std::mem::offset_of!(avifGainMap, altMatrixCoefficients) - 172usize];
    ["Offset of field: avifGainMap::altYUVRange"]
        [::std::mem::offset_of!(avifGainMap, altYUVRange) - 176usize];
    ["Offset of field: avifGainMap::altDepth"]
        [::std::mem::offset_of!(avifGainMap, altDepth) - 180usize];
    ["Offset of field: avifGainMap::altPlaneCount"]
        [::std::mem::offset_of!(avifGainMap, altPlaneCount) - 184usize];
    ["Offset of field: avifGainMap::altCLLI"]
        [::std::mem::offset_of!(avifGainMap, altCLLI) - 188usize];
};
unsafe extern "C" {
    pub fn avifGainMapCreate() -> *mut avifGainMap;
}
unsafe extern "C" {
    pub fn avifGainMapDestroy(gainMap: *mut avifGainMap);
}
pub const AVIF_SAMPLE_TRANSFORM_NONE: avifSampleTransformRecipe = 0;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B: avifSampleTransformRecipe = 1;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B: avifSampleTransformRecipe = 2;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B: avifSampleTransformRecipe =
    3;
pub type avifSampleTransformRecipe = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifImageItemProperty {
    pub boxtype: [u8; 4usize],
    pub usertype: [u8; 16usize],
    pub boxPayload: avifRWData,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifImageItemProperty"][::std::mem::size_of::<avifImageItemProperty>() - 40usize];
    ["Alignment of avifImageItemProperty"]
        [::std::mem::align_of::<avifImageItemProperty>() - 8usize];
    ["Offset of field: avifImageItemProperty::boxtype"]
        [::std::mem::offset_of!(avifImageItemProperty, boxtype) - 0usize];
    ["Offset of field: avifImageItemProperty::usertype"]
        [::std::mem::offset_of!(avifImageItemProperty, usertype) - 4usize];
    ["Offset of field: avifImageItemProperty::boxPayload"]
        [::std::mem::offset_of!(avifImageItemProperty, boxPayload) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifImage {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub yuvFormat: avifPixelFormat,
    pub yuvRange: avifRange,
    pub yuvChromaSamplePosition: avifChromaSamplePosition,
    pub yuvPlanes: [*mut u8; 3usize],
    pub yuvRowBytes: [u32; 3usize],
    pub imageOwnsYUVPlanes: avifBool,
    pub alphaPlane: *mut u8,
    pub alphaRowBytes: u32,
    pub imageOwnsAlphaPlane: avifBool,
    pub alphaPremultiplied: avifBool,
    pub icc: avifRWData,
    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,
    pub clli: avifContentLightLevelInformationBox,
    pub transformFlags: avifTransformFlags,
    pub pasp: avifPixelAspectRatioBox,
    pub clap: avifCleanApertureBox,
    pub irot: avifImageRotation,
    pub imir: avifImageMirror,
    pub exif: avifRWData,
    pub xmp: avifRWData,
    pub properties: *mut avifImageItemProperty,
    pub numProperties: usize,
    pub gainMap: *mut avifGainMap,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifImage"][::std::mem::size_of::<avifImage>() - 224usize];
    ["Alignment of avifImage"][::std::mem::align_of::<avifImage>() - 8usize];
    ["Offset of field: avifImage::width"][::std::mem::offset_of!(avifImage, width) - 0usize];
    ["Offset of field: avifImage::height"][::std::mem::offset_of!(avifImage, height) - 4usize];
    ["Offset of field: avifImage::depth"][::std::mem::offset_of!(avifImage, depth) - 8usize];
    ["Offset of field: avifImage::yuvFormat"]
        [::std::mem::offset_of!(avifImage, yuvFormat) - 12usize];
    ["Offset of field: avifImage::yuvRange"][::std::mem::offset_of!(avifImage, yuvRange) - 16usize];
    ["Offset of field: avifImage::yuvChromaSamplePosition"]
        [::std::mem::offset_of!(avifImage, yuvChromaSamplePosition) - 20usize];
    ["Offset of field: avifImage::yuvPlanes"]
        [::std::mem::offset_of!(avifImage, yuvPlanes) - 24usize];
    ["Offset of field: avifImage::yuvRowBytes"]
        [::std::mem::offset_of!(avifImage, yuvRowBytes) - 48usize];
    ["Offset of field: avifImage::imageOwnsYUVPlanes"]
        [::std::mem::offset_of!(avifImage, imageOwnsYUVPlanes) - 60usize];
    ["Offset of field: avifImage::alphaPlane"]
        [::std::mem::offset_of!(avifImage, alphaPlane) - 64usize];
    ["Offset of field: avifImage::alphaRowBytes"]
        [::std::mem::offset_of!(avifImage, alphaRowBytes) - 72usize];
    ["Offset of field: avifImage::imageOwnsAlphaPlane"]
        [::std::mem::offset_of!(avifImage, imageOwnsAlphaPlane) - 76usize];
    ["Offset of field: avifImage::alphaPremultiplied"]
        [::std::mem::offset_of!(avifImage, alphaPremultiplied) - 80usize];
    ["Offset of field: avifImage::icc"][::std::mem::offset_of!(avifImage, icc) - 88usize];
    ["Offset of field: avifImage::colorPrimaries"]
        [::std::mem::offset_of!(avifImage, colorPrimaries) - 104usize];
    ["Offset of field: avifImage::transferCharacteristics"]
        [::std::mem::offset_of!(avifImage, transferCharacteristics) - 106usize];
    ["Offset of field: avifImage::matrixCoefficients"]
        [::std::mem::offset_of!(avifImage, matrixCoefficients) - 108usize];
    ["Offset of field: avifImage::clli"][::std::mem::offset_of!(avifImage, clli) - 110usize];
    ["Offset of field: avifImage::transformFlags"]
        [::std::mem::offset_of!(avifImage, transformFlags) - 116usize];
    ["Offset of field: avifImage::pasp"][::std::mem::offset_of!(avifImage, pasp) - 120usize];
    ["Offset of field: avifImage::clap"][::std::mem::offset_of!(avifImage, clap) - 128usize];
    ["Offset of field: avifImage::irot"][::std::mem::offset_of!(avifImage, irot) - 160usize];
    ["Offset of field: avifImage::imir"][::std::mem::offset_of!(avifImage, imir) - 161usize];
    ["Offset of field: avifImage::exif"][::std::mem::offset_of!(avifImage, exif) - 168usize];
    ["Offset of field: avifImage::xmp"][::std::mem::offset_of!(avifImage, xmp) - 184usize];
    ["Offset of field: avifImage::properties"]
        [::std::mem::offset_of!(avifImage, properties) - 200usize];
    ["Offset of field: avifImage::numProperties"]
        [::std::mem::offset_of!(avifImage, numProperties) - 208usize];
    ["Offset of field: avifImage::gainMap"][::std::mem::offset_of!(avifImage, gainMap) - 216usize];
};
unsafe extern "C" {
    pub fn avifImageCreate(
        width: u32,
        height: u32,
        depth: u32,
        yuvFormat: avifPixelFormat,
    ) -> *mut avifImage;
}
unsafe extern "C" {
    pub fn avifImageCreateEmpty() -> *mut avifImage;
}
unsafe extern "C" {
    pub fn avifImageCopy(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageSetViewRect(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        rect: *const avifCropRect,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageDestroy(image: *mut avifImage);
}
unsafe extern "C" {
    pub fn avifImageSetProfileICC(
        image: *mut avifImage,
        icc: *const u8,
        iccSize: usize,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageSetMetadataExif(
        image: *mut avifImage,
        exif: *const u8,
        exifSize: usize,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageSetMetadataXMP(
        image: *mut avifImage,
        xmp: *const u8,
        xmpSize: usize,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageAllocatePlanes(image: *mut avifImage, planes: avifPlanesFlags) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageFreePlanes(image: *mut avifImage, planes: avifPlanesFlags);
}
unsafe extern "C" {
    pub fn avifImageStealPlanes(
        dstImage: *mut avifImage,
        srcImage: *mut avifImage,
        planes: avifPlanesFlags,
    );
}
unsafe extern "C" {
    pub fn avifImageAddOpaqueProperty(
        image: *mut avifImage,
        boxtype: *const u8,
        data: *const u8,
        dataSize: usize,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageAddUUIDProperty(
        image: *mut avifImage,
        uuid: *const u8,
        data: *const u8,
        dataSize: usize,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageScale(
        image: *mut avifImage,
        dstWidth: u32,
        dstHeight: u32,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
}
pub const AVIF_RGB_FORMAT_RGB: avifRGBFormat = 0;
pub const AVIF_RGB_FORMAT_RGBA: avifRGBFormat = 1;
pub const AVIF_RGB_FORMAT_ARGB: avifRGBFormat = 2;
pub const AVIF_RGB_FORMAT_BGR: avifRGBFormat = 3;
pub const AVIF_RGB_FORMAT_BGRA: avifRGBFormat = 4;
pub const AVIF_RGB_FORMAT_ABGR: avifRGBFormat = 5;
pub const AVIF_RGB_FORMAT_RGB_565: avifRGBFormat = 6;
pub const AVIF_RGB_FORMAT_GRAY: avifRGBFormat = 7;
pub const AVIF_RGB_FORMAT_GRAYA: avifRGBFormat = 8;
pub const AVIF_RGB_FORMAT_AGRAY: avifRGBFormat = 9;
pub const AVIF_RGB_FORMAT_COUNT: avifRGBFormat = 10;
pub type avifRGBFormat = ::std::os::raw::c_int;
unsafe extern "C" {
    pub fn avifRGBFormatChannelCount(format: avifRGBFormat) -> u32;
}
unsafe extern "C" {
    pub fn avifRGBFormatHasAlpha(format: avifRGBFormat) -> avifBool;
}
unsafe extern "C" {
    pub fn avifRGBFormatIsGray(format: avifRGBFormat) -> avifBool;
}
pub const AVIF_CHROMA_UPSAMPLING_AUTOMATIC: avifChromaUpsampling = 0;
pub const AVIF_CHROMA_UPSAMPLING_FASTEST: avifChromaUpsampling = 1;
pub const AVIF_CHROMA_UPSAMPLING_BEST_QUALITY: avifChromaUpsampling = 2;
pub const AVIF_CHROMA_UPSAMPLING_NEAREST: avifChromaUpsampling = 3;
pub const AVIF_CHROMA_UPSAMPLING_BILINEAR: avifChromaUpsampling = 4;
pub type avifChromaUpsampling = ::std::os::raw::c_int;
pub const AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC: avifChromaDownsampling = 0;
pub const AVIF_CHROMA_DOWNSAMPLING_FASTEST: avifChromaDownsampling = 1;
pub const AVIF_CHROMA_DOWNSAMPLING_BEST_QUALITY: avifChromaDownsampling = 2;
pub const AVIF_CHROMA_DOWNSAMPLING_AVERAGE: avifChromaDownsampling = 3;
pub const AVIF_CHROMA_DOWNSAMPLING_SHARP_YUV: avifChromaDownsampling = 4;
pub type avifChromaDownsampling = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifRGBImage {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: avifRGBFormat,
    pub chromaUpsampling: avifChromaUpsampling,
    pub chromaDownsampling: avifChromaDownsampling,
    pub avoidLibYUV: avifBool,
    pub ignoreAlpha: avifBool,
    pub alphaPremultiplied: avifBool,
    pub isFloat: avifBool,
    pub maxThreads: ::std::os::raw::c_int,
    pub pixels: *mut u8,
    pub rowBytes: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifRGBImage"][::std::mem::size_of::<avifRGBImage>() - 64usize];
    ["Alignment of avifRGBImage"][::std::mem::align_of::<avifRGBImage>() - 8usize];
    ["Offset of field: avifRGBImage::width"][::std::mem::offset_of!(avifRGBImage, width) - 0usize];
    ["Offset of field: avifRGBImage::height"]
        [::std::mem::offset_of!(avifRGBImage, height) - 4usize];
    ["Offset of field: avifRGBImage::depth"][::std::mem::offset_of!(avifRGBImage, depth) - 8usize];
    ["Offset of field: avifRGBImage::format"]
        [::std::mem::offset_of!(avifRGBImage, format) - 12usize];
    ["Offset of field: avifRGBImage::chromaUpsampling"]
        [::std::mem::offset_of!(avifRGBImage, chromaUpsampling) - 16usize];
    ["Offset of field: avifRGBImage::chromaDownsampling"]
        [::std::mem::offset_of!(avifRGBImage, chromaDownsampling) - 20usize];
    ["Offset of field: avifRGBImage::avoidLibYUV"]
        [::std::mem::offset_of!(avifRGBImage, avoidLibYUV) - 24usize];
    ["Offset of field: avifRGBImage::ignoreAlpha"]
        [::std::mem::offset_of!(avifRGBImage, ignoreAlpha) - 28usize];
    ["Offset of field: avifRGBImage::alphaPremultiplied"]
        [::std::mem::offset_of!(avifRGBImage, alphaPremultiplied) - 32usize];
    ["Offset of field: avifRGBImage::isFloat"]
        [::std::mem::offset_of!(avifRGBImage, isFloat) - 36usize];
    ["Offset of field: avifRGBImage::maxThreads"]
        [::std::mem::offset_of!(avifRGBImage, maxThreads) - 40usize];
    ["Offset of field: avifRGBImage::pixels"]
        [::std::mem::offset_of!(avifRGBImage, pixels) - 48usize];
    ["Offset of field: avifRGBImage::rowBytes"]
        [::std::mem::offset_of!(avifRGBImage, rowBytes) - 56usize];
};
unsafe extern "C" {
    pub fn avifRGBImageSetDefaults(rgb: *mut avifRGBImage, image: *const avifImage);
}
unsafe extern "C" {
    pub fn avifRGBImagePixelSize(rgb: *const avifRGBImage) -> u32;
}
unsafe extern "C" {
    pub fn avifRGBImageAllocatePixels(rgb: *mut avifRGBImage) -> avifResult;
}
unsafe extern "C" {
    pub fn avifRGBImageFreePixels(rgb: *mut avifRGBImage);
}
unsafe extern "C" {
    pub fn avifImageRGBToYUV(image: *mut avifImage, rgb: *const avifRGBImage) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageYUVToRGB(image: *const avifImage, rgb: *mut avifRGBImage) -> avifResult;
}
unsafe extern "C" {
    pub fn avifRGBImagePremultiplyAlpha(rgb: *mut avifRGBImage) -> avifResult;
}
unsafe extern "C" {
    pub fn avifRGBImageUnpremultiplyAlpha(rgb: *mut avifRGBImage) -> avifResult;
}
unsafe extern "C" {
    pub fn avifFullToLimitedY(depth: u32, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn avifFullToLimitedUV(depth: u32, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn avifLimitedToFullY(depth: u32, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn avifLimitedToFullUV(depth: u32, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
pub const AVIF_CODEC_CHOICE_AUTO: avifCodecChoice = 0;
pub const AVIF_CODEC_CHOICE_AOM: avifCodecChoice = 1;
pub const AVIF_CODEC_CHOICE_DAV1D: avifCodecChoice = 2;
pub const AVIF_CODEC_CHOICE_LIBGAV1: avifCodecChoice = 3;
pub const AVIF_CODEC_CHOICE_RAV1E: avifCodecChoice = 4;
pub const AVIF_CODEC_CHOICE_SVT: avifCodecChoice = 5;
pub const AVIF_CODEC_CHOICE_AVM: avifCodecChoice = 6;
pub type avifCodecChoice = ::std::os::raw::c_int;
pub const AVIF_CODEC_FLAG_CAN_DECODE: avifCodecFlag = 1;
pub const AVIF_CODEC_FLAG_CAN_ENCODE: avifCodecFlag = 2;
pub type avifCodecFlag = ::std::os::raw::c_int;
pub type avifCodecFlags = u32;
unsafe extern "C" {
    pub fn avifCodecName(
        choice: avifCodecChoice,
        requiredFlags: avifCodecFlags,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn avifCodecChoiceFromName(name: *const ::std::os::raw::c_char) -> avifCodecChoice;
}
pub type avifIODestroyFunc = ::std::option::Option<unsafe extern "C" fn(io: *mut avifIO)>;
pub type avifIOReadFunc = ::std::option::Option<
    unsafe extern "C" fn(
        io: *mut avifIO,
        readFlags: u32,
        offset: u64,
        size: usize,
        out: *mut avifROData,
    ) -> avifResult,
>;
pub type avifIOWriteFunc = ::std::option::Option<
    unsafe extern "C" fn(
        io: *mut avifIO,
        writeFlags: u32,
        offset: u64,
        data: *const u8,
        size: usize,
    ) -> avifResult,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifIO {
    pub destroy: avifIODestroyFunc,
    pub read: avifIOReadFunc,
    pub write: avifIOWriteFunc,
    pub sizeHint: u64,
    pub persistent: avifBool,
    pub data: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifIO"][::std::mem::size_of::<avifIO>() - 48usize];
    ["Alignment of avifIO"][::std::mem::align_of::<avifIO>() - 8usize];
    ["Offset of field: avifIO::destroy"][::std::mem::offset_of!(avifIO, destroy) - 0usize];
    ["Offset of field: avifIO::read"][::std::mem::offset_of!(avifIO, read) - 8usize];
    ["Offset of field: avifIO::write"][::std::mem::offset_of!(avifIO, write) - 16usize];
    ["Offset of field: avifIO::sizeHint"][::std::mem::offset_of!(avifIO, sizeHint) - 24usize];
    ["Offset of field: avifIO::persistent"][::std::mem::offset_of!(avifIO, persistent) - 32usize];
    ["Offset of field: avifIO::data"][::std::mem::offset_of!(avifIO, data) - 40usize];
};
unsafe extern "C" {
    pub fn avifIOCreateMemoryReader(data: *const u8, size: usize) -> *mut avifIO;
}
unsafe extern "C" {
    pub fn avifIOCreateFileReader(filename: *const ::std::os::raw::c_char) -> *mut avifIO;
}
unsafe extern "C" {
    pub fn avifIODestroy(io: *mut avifIO);
}
pub const AVIF_STRICT_DISABLED: avifStrictFlag = 0;
pub const AVIF_STRICT_PIXI_REQUIRED: avifStrictFlag = 1;
pub const AVIF_STRICT_CLAP_VALID: avifStrictFlag = 2;
pub const AVIF_STRICT_ALPHA_ISPE_REQUIRED: avifStrictFlag = 4;
pub const AVIF_STRICT_ENABLED: avifStrictFlag = 7;
pub type avifStrictFlag = ::std::os::raw::c_int;
pub type avifStrictFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifIOStats {
    pub colorOBUSize: usize,
    pub alphaOBUSize: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifIOStats"][::std::mem::size_of::<avifIOStats>() - 16usize];
    ["Alignment of avifIOStats"][::std::mem::align_of::<avifIOStats>() - 8usize];
    ["Offset of field: avifIOStats::colorOBUSize"]
        [::std::mem::offset_of!(avifIOStats, colorOBUSize) - 0usize];
    ["Offset of field: avifIOStats::alphaOBUSize"]
        [::std::mem::offset_of!(avifIOStats, alphaOBUSize) - 8usize];
};
#[repr(C)]
#[derive(Debug)]
pub struct avifDecoderData {
    _unused: [u8; 0],
}
pub const AVIF_DECODER_SOURCE_AUTO: avifDecoderSource = 0;
pub const AVIF_DECODER_SOURCE_PRIMARY_ITEM: avifDecoderSource = 1;
pub const AVIF_DECODER_SOURCE_TRACKS: avifDecoderSource = 2;
pub type avifDecoderSource = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifImageTiming {
    pub timescale: u64,
    pub pts: f64,
    pub ptsInTimescales: u64,
    pub duration: f64,
    pub durationInTimescales: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifImageTiming"][::std::mem::size_of::<avifImageTiming>() - 40usize];
    ["Alignment of avifImageTiming"][::std::mem::align_of::<avifImageTiming>() - 8usize];
    ["Offset of field: avifImageTiming::timescale"]
        [::std::mem::offset_of!(avifImageTiming, timescale) - 0usize];
    ["Offset of field: avifImageTiming::pts"]
        [::std::mem::offset_of!(avifImageTiming, pts) - 8usize];
    ["Offset of field: avifImageTiming::ptsInTimescales"]
        [::std::mem::offset_of!(avifImageTiming, ptsInTimescales) - 16usize];
    ["Offset of field: avifImageTiming::duration"]
        [::std::mem::offset_of!(avifImageTiming, duration) - 24usize];
    ["Offset of field: avifImageTiming::durationInTimescales"]
        [::std::mem::offset_of!(avifImageTiming, durationInTimescales) - 32usize];
};
pub const AVIF_PROGRESSIVE_STATE_UNAVAILABLE: avifProgressiveState = 0;
pub const AVIF_PROGRESSIVE_STATE_AVAILABLE: avifProgressiveState = 1;
pub const AVIF_PROGRESSIVE_STATE_ACTIVE: avifProgressiveState = 2;
pub type avifProgressiveState = ::std::os::raw::c_int;
unsafe extern "C" {
    pub fn avifProgressiveStateToString(
        progressiveState: avifProgressiveState,
    ) -> *const ::std::os::raw::c_char;
}
pub const AVIF_IMAGE_CONTENT_NONE: avifImageContentTypeFlag = 0;
pub const AVIF_IMAGE_CONTENT_COLOR_AND_ALPHA: avifImageContentTypeFlag = 3;
pub const AVIF_IMAGE_CONTENT_GAIN_MAP: avifImageContentTypeFlag = 4;
pub const AVIF_IMAGE_CONTENT_ALL: avifImageContentTypeFlag = 7;
pub const AVIF_IMAGE_CONTENT_SAMPLE_TRANSFORMS: avifImageContentTypeFlag = 8;
pub const AVIF_IMAGE_CONTENT_DECODE_DEFAULT: avifImageContentTypeFlag = 3;
pub type avifImageContentTypeFlag = ::std::os::raw::c_int;
pub type avifImageContentTypeFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifDecoder {
    pub codecChoice: avifCodecChoice,
    pub maxThreads: ::std::os::raw::c_int,
    pub requestedSource: avifDecoderSource,
    pub allowProgressive: avifBool,
    pub allowIncremental: avifBool,
    pub ignoreExif: avifBool,
    pub ignoreXMP: avifBool,
    pub imageSizeLimit: u32,
    pub imageDimensionLimit: u32,
    pub imageCountLimit: u32,
    pub strictFlags: avifStrictFlags,
    pub image: *mut avifImage,
    pub imageIndex: ::std::os::raw::c_int,
    pub imageCount: ::std::os::raw::c_int,
    pub progressiveState: avifProgressiveState,
    pub imageTiming: avifImageTiming,
    pub timescale: u64,
    pub duration: f64,
    pub durationInTimescales: u64,
    pub repetitionCount: ::std::os::raw::c_int,
    pub alphaPresent: avifBool,
    pub ioStats: avifIOStats,
    pub diag: avifDiagnostics,
    pub io: *mut avifIO,
    pub data: *mut avifDecoderData,
    pub imageSequenceTrackPresent: avifBool,
    pub imageContentToDecode: avifImageContentTypeFlags,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifDecoder"][::std::mem::size_of::<avifDecoder>() - 440usize];
    ["Alignment of avifDecoder"][::std::mem::align_of::<avifDecoder>() - 8usize];
    ["Offset of field: avifDecoder::codecChoice"]
        [::std::mem::offset_of!(avifDecoder, codecChoice) - 0usize];
    ["Offset of field: avifDecoder::maxThreads"]
        [::std::mem::offset_of!(avifDecoder, maxThreads) - 4usize];
    ["Offset of field: avifDecoder::requestedSource"]
        [::std::mem::offset_of!(avifDecoder, requestedSource) - 8usize];
    ["Offset of field: avifDecoder::allowProgressive"]
        [::std::mem::offset_of!(avifDecoder, allowProgressive) - 12usize];
    ["Offset of field: avifDecoder::allowIncremental"]
        [::std::mem::offset_of!(avifDecoder, allowIncremental) - 16usize];
    ["Offset of field: avifDecoder::ignoreExif"]
        [::std::mem::offset_of!(avifDecoder, ignoreExif) - 20usize];
    ["Offset of field: avifDecoder::ignoreXMP"]
        [::std::mem::offset_of!(avifDecoder, ignoreXMP) - 24usize];
    ["Offset of field: avifDecoder::imageSizeLimit"]
        [::std::mem::offset_of!(avifDecoder, imageSizeLimit) - 28usize];
    ["Offset of field: avifDecoder::imageDimensionLimit"]
        [::std::mem::offset_of!(avifDecoder, imageDimensionLimit) - 32usize];
    ["Offset of field: avifDecoder::imageCountLimit"]
        [::std::mem::offset_of!(avifDecoder, imageCountLimit) - 36usize];
    ["Offset of field: avifDecoder::strictFlags"]
        [::std::mem::offset_of!(avifDecoder, strictFlags) - 40usize];
    ["Offset of field: avifDecoder::image"][::std::mem::offset_of!(avifDecoder, image) - 48usize];
    ["Offset of field: avifDecoder::imageIndex"]
        [::std::mem::offset_of!(avifDecoder, imageIndex) - 56usize];
    ["Offset of field: avifDecoder::imageCount"]
        [::std::mem::offset_of!(avifDecoder, imageCount) - 60usize];
    ["Offset of field: avifDecoder::progressiveState"]
        [::std::mem::offset_of!(avifDecoder, progressiveState) - 64usize];
    ["Offset of field: avifDecoder::imageTiming"]
        [::std::mem::offset_of!(avifDecoder, imageTiming) - 72usize];
    ["Offset of field: avifDecoder::timescale"]
        [::std::mem::offset_of!(avifDecoder, timescale) - 112usize];
    ["Offset of field: avifDecoder::duration"]
        [::std::mem::offset_of!(avifDecoder, duration) - 120usize];
    ["Offset of field: avifDecoder::durationInTimescales"]
        [::std::mem::offset_of!(avifDecoder, durationInTimescales) - 128usize];
    ["Offset of field: avifDecoder::repetitionCount"]
        [::std::mem::offset_of!(avifDecoder, repetitionCount) - 136usize];
    ["Offset of field: avifDecoder::alphaPresent"]
        [::std::mem::offset_of!(avifDecoder, alphaPresent) - 140usize];
    ["Offset of field: avifDecoder::ioStats"]
        [::std::mem::offset_of!(avifDecoder, ioStats) - 144usize];
    ["Offset of field: avifDecoder::diag"][::std::mem::offset_of!(avifDecoder, diag) - 160usize];
    ["Offset of field: avifDecoder::io"][::std::mem::offset_of!(avifDecoder, io) - 416usize];
    ["Offset of field: avifDecoder::data"][::std::mem::offset_of!(avifDecoder, data) - 424usize];
    ["Offset of field: avifDecoder::imageSequenceTrackPresent"]
        [::std::mem::offset_of!(avifDecoder, imageSequenceTrackPresent) - 432usize];
    ["Offset of field: avifDecoder::imageContentToDecode"]
        [::std::mem::offset_of!(avifDecoder, imageContentToDecode) - 436usize];
};
unsafe extern "C" {
    pub fn avifDecoderCreate() -> *mut avifDecoder;
}
unsafe extern "C" {
    pub fn avifDecoderDestroy(decoder: *mut avifDecoder);
}
unsafe extern "C" {
    pub fn avifDecoderRead(decoder: *mut avifDecoder, image: *mut avifImage) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderReadMemory(
        decoder: *mut avifDecoder,
        image: *mut avifImage,
        data: *const u8,
        size: usize,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderReadFile(
        decoder: *mut avifDecoder,
        image: *mut avifImage,
        filename: *const ::std::os::raw::c_char,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderSetSource(decoder: *mut avifDecoder, source: avifDecoderSource)
    -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderSetIO(decoder: *mut avifDecoder, io: *mut avifIO);
}
unsafe extern "C" {
    pub fn avifDecoderSetIOMemory(
        decoder: *mut avifDecoder,
        data: *const u8,
        size: usize,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderSetIOFile(
        decoder: *mut avifDecoder,
        filename: *const ::std::os::raw::c_char,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderParse(decoder: *mut avifDecoder) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderNextImage(decoder: *mut avifDecoder) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderNthImage(decoder: *mut avifDecoder, frameIndex: u32) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderReset(decoder: *mut avifDecoder) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderIsKeyframe(decoder: *const avifDecoder, frameIndex: u32) -> avifBool;
}
unsafe extern "C" {
    pub fn avifDecoderNearestKeyframe(decoder: *const avifDecoder, frameIndex: u32) -> u32;
}
unsafe extern "C" {
    pub fn avifDecoderNthImageTiming(
        decoder: *const avifDecoder,
        frameIndex: u32,
        outTiming: *mut avifImageTiming,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifDecoderDecodedRowCount(decoder: *const avifDecoder) -> u32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifExtent {
    pub offset: u64,
    pub size: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifExtent"][::std::mem::size_of::<avifExtent>() - 16usize];
    ["Alignment of avifExtent"][::std::mem::align_of::<avifExtent>() - 8usize];
    ["Offset of field: avifExtent::offset"][::std::mem::offset_of!(avifExtent, offset) - 0usize];
    ["Offset of field: avifExtent::size"][::std::mem::offset_of!(avifExtent, size) - 8usize];
};
unsafe extern "C" {
    pub fn avifDecoderNthImageMaxExtent(
        decoder: *const avifDecoder,
        frameIndex: u32,
        outExtent: *mut avifExtent,
    ) -> avifResult;
}
#[repr(C)]
#[derive(Debug)]
pub struct avifEncoderData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
pub struct avifCodecSpecificOptions {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifScalingMode {
    pub horizontal: avifFraction,
    pub vertical: avifFraction,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifScalingMode"][::std::mem::size_of::<avifScalingMode>() - 16usize];
    ["Alignment of avifScalingMode"][::std::mem::align_of::<avifScalingMode>() - 4usize];
    ["Offset of field: avifScalingMode::horizontal"]
        [::std::mem::offset_of!(avifScalingMode, horizontal) - 0usize];
    ["Offset of field: avifScalingMode::vertical"]
        [::std::mem::offset_of!(avifScalingMode, vertical) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifEncoder {
    pub codecChoice: avifCodecChoice,
    pub maxThreads: ::std::os::raw::c_int,
    pub speed: ::std::os::raw::c_int,
    pub keyframeInterval: ::std::os::raw::c_int,
    pub timescale: u64,
    pub repetitionCount: ::std::os::raw::c_int,
    pub extraLayerCount: u32,
    pub quality: ::std::os::raw::c_int,
    pub qualityAlpha: ::std::os::raw::c_int,
    pub minQuantizer: ::std::os::raw::c_int,
    pub maxQuantizer: ::std::os::raw::c_int,
    pub minQuantizerAlpha: ::std::os::raw::c_int,
    pub maxQuantizerAlpha: ::std::os::raw::c_int,
    pub tileRowsLog2: ::std::os::raw::c_int,
    pub tileColsLog2: ::std::os::raw::c_int,
    pub autoTiling: avifBool,
    pub scalingMode: avifScalingMode,
    pub ioStats: avifIOStats,
    pub diag: avifDiagnostics,
    pub data: *mut avifEncoderData,
    pub csOptions: *mut avifCodecSpecificOptions,
    pub headerFormat: avifHeaderFormatFlags,
    pub qualityGainMap: ::std::os::raw::c_int,
    pub creationTime: u64,
    pub modificationTime: u64,
    pub sampleTransformRecipe: avifSampleTransformRecipe,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of avifEncoder"][::std::mem::size_of::<avifEncoder>() - 408usize];
    ["Alignment of avifEncoder"][::std::mem::align_of::<avifEncoder>() - 8usize];
    ["Offset of field: avifEncoder::codecChoice"]
        [::std::mem::offset_of!(avifEncoder, codecChoice) - 0usize];
    ["Offset of field: avifEncoder::maxThreads"]
        [::std::mem::offset_of!(avifEncoder, maxThreads) - 4usize];
    ["Offset of field: avifEncoder::speed"][::std::mem::offset_of!(avifEncoder, speed) - 8usize];
    ["Offset of field: avifEncoder::keyframeInterval"]
        [::std::mem::offset_of!(avifEncoder, keyframeInterval) - 12usize];
    ["Offset of field: avifEncoder::timescale"]
        [::std::mem::offset_of!(avifEncoder, timescale) - 16usize];
    ["Offset of field: avifEncoder::repetitionCount"]
        [::std::mem::offset_of!(avifEncoder, repetitionCount) - 24usize];
    ["Offset of field: avifEncoder::extraLayerCount"]
        [::std::mem::offset_of!(avifEncoder, extraLayerCount) - 28usize];
    ["Offset of field: avifEncoder::quality"]
        [::std::mem::offset_of!(avifEncoder, quality) - 32usize];
    ["Offset of field: avifEncoder::qualityAlpha"]
        [::std::mem::offset_of!(avifEncoder, qualityAlpha) - 36usize];
    ["Offset of field: avifEncoder::minQuantizer"]
        [::std::mem::offset_of!(avifEncoder, minQuantizer) - 40usize];
    ["Offset of field: avifEncoder::maxQuantizer"]
        [::std::mem::offset_of!(avifEncoder, maxQuantizer) - 44usize];
    ["Offset of field: avifEncoder::minQuantizerAlpha"]
        [::std::mem::offset_of!(avifEncoder, minQuantizerAlpha) - 48usize];
    ["Offset of field: avifEncoder::maxQuantizerAlpha"]
        [::std::mem::offset_of!(avifEncoder, maxQuantizerAlpha) - 52usize];
    ["Offset of field: avifEncoder::tileRowsLog2"]
        [::std::mem::offset_of!(avifEncoder, tileRowsLog2) - 56usize];
    ["Offset of field: avifEncoder::tileColsLog2"]
        [::std::mem::offset_of!(avifEncoder, tileColsLog2) - 60usize];
    ["Offset of field: avifEncoder::autoTiling"]
        [::std::mem::offset_of!(avifEncoder, autoTiling) - 64usize];
    ["Offset of field: avifEncoder::scalingMode"]
        [::std::mem::offset_of!(avifEncoder, scalingMode) - 68usize];
    ["Offset of field: avifEncoder::ioStats"]
        [::std::mem::offset_of!(avifEncoder, ioStats) - 88usize];
    ["Offset of field: avifEncoder::diag"][::std::mem::offset_of!(avifEncoder, diag) - 104usize];
    ["Offset of field: avifEncoder::data"][::std::mem::offset_of!(avifEncoder, data) - 360usize];
    ["Offset of field: avifEncoder::csOptions"]
        [::std::mem::offset_of!(avifEncoder, csOptions) - 368usize];
    ["Offset of field: avifEncoder::headerFormat"]
        [::std::mem::offset_of!(avifEncoder, headerFormat) - 376usize];
    ["Offset of field: avifEncoder::qualityGainMap"]
        [::std::mem::offset_of!(avifEncoder, qualityGainMap) - 380usize];
    ["Offset of field: avifEncoder::creationTime"]
        [::std::mem::offset_of!(avifEncoder, creationTime) - 384usize];
    ["Offset of field: avifEncoder::modificationTime"]
        [::std::mem::offset_of!(avifEncoder, modificationTime) - 392usize];
    ["Offset of field: avifEncoder::sampleTransformRecipe"]
        [::std::mem::offset_of!(avifEncoder, sampleTransformRecipe) - 400usize];
};
unsafe extern "C" {
    pub fn avifEncoderCreate() -> *mut avifEncoder;
}
unsafe extern "C" {
    pub fn avifEncoderWrite(
        encoder: *mut avifEncoder,
        image: *const avifImage,
        output: *mut avifRWData,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifEncoderDestroy(encoder: *mut avifEncoder);
}
pub const AVIF_ADD_IMAGE_FLAG_NONE: avifAddImageFlag = 0;
pub const AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME: avifAddImageFlag = 1;
pub const AVIF_ADD_IMAGE_FLAG_SINGLE: avifAddImageFlag = 2;
pub type avifAddImageFlag = ::std::os::raw::c_int;
pub type avifAddImageFlags = u32;
unsafe extern "C" {
    pub fn avifEncoderAddImage(
        encoder: *mut avifEncoder,
        image: *const avifImage,
        durationInTimescales: u64,
        addImageFlags: avifAddImageFlags,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifEncoderAddImageGrid(
        encoder: *mut avifEncoder,
        gridCols: u32,
        gridRows: u32,
        cellImages: *const *const avifImage,
        addImageFlags: avifAddImageFlags,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifEncoderFinish(encoder: *mut avifEncoder, output: *mut avifRWData) -> avifResult;
}
unsafe extern "C" {
    pub fn avifEncoderSetCodecSpecificOption(
        encoder: *mut avifEncoder,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifEncoderGetGainMapSizeBytes(encoder: *mut avifEncoder) -> usize;
}
unsafe extern "C" {
    pub fn avifImageUsesU16(image: *const avifImage) -> avifBool;
}
unsafe extern "C" {
    pub fn avifImageIsOpaque(image: *const avifImage) -> avifBool;
}
unsafe extern "C" {
    pub fn avifImagePlane(image: *const avifImage, channel: ::std::os::raw::c_int) -> *mut u8;
}
unsafe extern "C" {
    pub fn avifImagePlaneRowBytes(image: *const avifImage, channel: ::std::os::raw::c_int) -> u32;
}
unsafe extern "C" {
    pub fn avifImagePlaneWidth(image: *const avifImage, channel: ::std::os::raw::c_int) -> u32;
}
unsafe extern "C" {
    pub fn avifImagePlaneHeight(image: *const avifImage, channel: ::std::os::raw::c_int) -> u32;
}
unsafe extern "C" {
    pub fn avifPeekCompatibleFileType(input: *const avifROData) -> avifBool;
}
unsafe extern "C" {
    pub fn avifImageApplyGainMap(
        baseImage: *const avifImage,
        gainMap: *const avifGainMap,
        hdrHeadroom: f32,
        outputColorPrimaries: avifColorPrimaries,
        outputTransferCharacteristics: avifTransferCharacteristics,
        toneMappedImage: *mut avifRGBImage,
        clli: *mut avifContentLightLevelInformationBox,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifRGBImageApplyGainMap(
        baseImage: *const avifRGBImage,
        baseColorPrimaries: avifColorPrimaries,
        baseTransferCharacteristics: avifTransferCharacteristics,
        gainMap: *const avifGainMap,
        hdrHeadroom: f32,
        outputColorPrimaries: avifColorPrimaries,
        outputTransferCharacteristics: avifTransferCharacteristics,
        toneMappedImage: *mut avifRGBImage,
        clli: *mut avifContentLightLevelInformationBox,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifRGBImageComputeGainMap(
        baseRgbImage: *const avifRGBImage,
        baseColorPrimaries: avifColorPrimaries,
        baseTransferCharacteristics: avifTransferCharacteristics,
        altRgbImage: *const avifRGBImage,
        altColorPrimaries: avifColorPrimaries,
        altTransferCharacteristics: avifTransferCharacteristics,
        gainMap: *mut avifGainMap,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
}
unsafe extern "C" {
    pub fn avifImageComputeGainMap(
        baseImage: *const avifImage,
        altImage: *const avifImage,
        gainMap: *mut avifGainMap,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
}
