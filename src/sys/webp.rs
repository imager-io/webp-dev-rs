#![allow(non_upper_case_globals)]
use std::ffi::{CString, c_void};
use std::os::raw::{c_char, c_int};
use libc::{size_t, c_float};

///////////////////////////////////////////////////////////////////////////////
// WEBP STRUCTS
///////////////////////////////////////////////////////////////////////////////
pub type WebPAuxStats = crate::raw::webp::WebPAuxStats;
pub type WebPBitstreamFeatures = crate::raw::webp::WebPBitstreamFeatures;
pub type WebPConfig = crate::raw::webp::WebPConfig;
pub type WebPDecBuffer = crate::raw::webp::WebPDecBuffer;
pub type WebPDecoderConfig = crate::raw::webp::WebPDecoderConfig;
pub type WebPDecoderOptions = crate::raw::webp::WebPDecoderOptions;
pub type WebPIDecoder = crate::raw::webp::WebPIDecoder;
pub type WebPMemoryWriter = crate::raw::webp::WebPMemoryWriter;
pub type WebPPicture = crate::raw::webp::WebPPicture;
pub type WebPRGBABuffer = crate::raw::webp::WebPRGBABuffer;
pub type WebPYUVABuffer = crate::raw::webp::WebPYUVABuffer;

///////////////////////////////////////////////////////////////////////////////
// TYPE-DEFS
///////////////////////////////////////////////////////////////////////////////
pub type VP8StatusCode = crate::raw::webp::VP8StatusCode;
pub type WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE;
pub type WebPEncCSP = crate::raw::webp::WebPEncCSP;
pub type WebPEncodingError = crate::raw::webp::WebPEncodingError;
pub type WebPImageHint = crate::raw::webp::WebPImageHint;
pub type WebPPreset = crate::raw::webp::WebPPreset;
pub type WebPProgressHook = crate::raw::webp::WebPProgressHook;
pub type WebPWriterFunction = crate::raw::webp::WebPWriterFunction;

///////////////////////////////////////////////////////////////////////////////
// WEBP CONSTANTS
///////////////////////////////////////////////////////////////////////////////
pub const WEBP_CSP_MODE_MODE_ARGB: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_ARGB;
#[allow(non_upper_case_globals)]
pub const WEBP_CSP_MODE_MODE_Argb: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_Argb;
pub const WEBP_CSP_MODE_MODE_BGR: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_BGR;
pub const WEBP_CSP_MODE_MODE_BGRA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_BGRA;
pub const WEBP_CSP_MODE_MODE_LAST: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_LAST;
pub const WEBP_CSP_MODE_MODE_RGB: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_RGB;
pub const WEBP_CSP_MODE_MODE_RGBA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_RGBA;
pub const WEBP_CSP_MODE_MODE_RGBA_4444: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_RGBA_4444;
pub const WEBP_CSP_MODE_MODE_RGB_565: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_RGB_565;
pub const WEBP_CSP_MODE_MODE_YUV: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_YUV;
pub const WEBP_CSP_MODE_MODE_YUVA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_YUVA;
pub const WEBP_CSP_MODE_MODE_bgrA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_bgrA;
pub const WEBP_CSP_MODE_MODE_rgbA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_rgbA;
pub const WEBP_CSP_MODE_MODE_rgbA_4444: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_rgbA_4444;
pub const WEBP_DECODER_ABI_VERSION: u32 = crate::raw::webp::WEBP_DECODER_ABI_VERSION;
pub const WEBP_ENCODER_ABI_VERSION: u32 = crate::raw::webp::WEBP_ENCODER_ABI_VERSION;
pub const WEBP_MAX_DIMENSION: u32 = crate::raw::webp::WEBP_MAX_DIMENSION;
pub const WINT_MAX: u32 = crate::raw::webp::WINT_MAX;
pub const WINT_MIN: i32 = crate::raw::webp::WINT_MIN;
pub const WebPEncCSP_WEBP_CSP_ALPHA_BIT: u32 = crate::raw::webp::WebPEncCSP_WEBP_CSP_ALPHA_BIT;
pub const WebPEncCSP_WEBP_CSP_UV_MASK: u32 = crate::raw::webp::WebPEncCSP_WEBP_CSP_UV_MASK;
pub const WebPEncCSP_WEBP_YUV420: u32 = crate::raw::webp::WebPEncCSP_WEBP_YUV420;
pub const WebPEncCSP_WEBP_YUV420A: u32 = crate::raw::webp::WebPEncCSP_WEBP_YUV420A;
pub const WebPEncodingError_VP8_ENC_OK: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_OK;
pub const WebPEncodingError_VP8_ENC_ERROR_OUT_OF_MEMORY: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_OUT_OF_MEMORY;
pub const WebPEncodingError_VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY;
pub const WebPEncodingError_VP8_ENC_ERROR_NULL_PARAMETER: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_NULL_PARAMETER;
pub const WebPEncodingError_VP8_ENC_ERROR_INVALID_CONFIGURATION: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_INVALID_CONFIGURATION;
pub const WebPEncodingError_VP8_ENC_ERROR_BAD_DIMENSION: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_BAD_DIMENSION;
pub const WebPEncodingError_VP8_ENC_ERROR_PARTITION_OVERFLOW: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_PARTITION_OVERFLOW;
pub const WebPEncodingError_VP8_ENC_ERROR_BAD_WRITE: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_BAD_WRITE;
pub const WebPEncodingError_VP8_ENC_ERROR_FILE_TOO_BIG: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_FILE_TOO_BIG;
pub const WebPEncodingError_VP8_ENC_ERROR_USER_ABORT: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_USER_ABORT;
pub const WebPEncodingError_VP8_ENC_ERROR_LAST: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_LAST;
pub const WebPEncodingError_VP8_ENC_ERROR_PARTITION0_OVERFLOW: u32 = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_PARTITION0_OVERFLOW;
pub const WebPImageHint_WEBP_HINT_DEFAULT: u32 = crate::raw::webp::WebPImageHint_WEBP_HINT_DEFAULT;
pub const WebPImageHint_WEBP_HINT_GRAPH: u32 = crate::raw::webp::WebPImageHint_WEBP_HINT_GRAPH;
pub const WebPImageHint_WEBP_HINT_LAST: u32 = crate::raw::webp::WebPImageHint_WEBP_HINT_LAST;
pub const WebPImageHint_WEBP_HINT_PHOTO: u32 = crate::raw::webp::WebPImageHint_WEBP_HINT_PHOTO;
pub const WebPImageHint_WEBP_HINT_PICTURE: u32 = crate::raw::webp::WebPImageHint_WEBP_HINT_PICTURE;
pub const WebPPreset_WEBP_PRESET_DEFAULT: u32 = crate::raw::webp::WebPPreset_WEBP_PRESET_DEFAULT;
pub const WebPPreset_WEBP_PRESET_DRAWING: u32 = crate::raw::webp::WebPPreset_WEBP_PRESET_DRAWING;
pub const WebPPreset_WEBP_PRESET_ICON: u32 = crate::raw::webp::WebPPreset_WEBP_PRESET_ICON;
pub const WebPPreset_WEBP_PRESET_PHOTO: u32 = crate::raw::webp::WebPPreset_WEBP_PRESET_PHOTO;
pub const WebPPreset_WEBP_PRESET_PICTURE: u32 = crate::raw::webp::WebPPreset_WEBP_PRESET_PICTURE;
pub const WebPPreset_WEBP_PRESET_TEXT: u32 = crate::raw::webp::WebPPreset_WEBP_PRESET_TEXT;

///////////////////////////////////////////////////////////////////////////////
// WEBP FUNCTIONS
///////////////////////////////////////////////////////////////////////////////
pub unsafe fn webp_blend_alpha(pic: *mut WebPPicture, background_rgb: u32) {
    crate::raw::webp::WebPBlendAlpha(pic, background_rgb)
}
pub unsafe fn webp_cleanup_transparent_area(picture: *mut WebPPicture) {
    crate::raw::webp::WebPCleanupTransparentArea(picture)
}
pub unsafe fn webp_config_init_internal(
    arg1: *mut WebPConfig,
    arg2: WebPPreset,
    arg3: f32,
    arg4: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPConfigInitInternal(arg1, arg2, arg3, arg4)
}
pub unsafe fn webp_config_lossless_preset(
    config: *mut WebPConfig,
    level: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPConfigLosslessPreset(config, level)
}
pub unsafe fn webp_decode(
    data: *const u8,
    data_size: usize,
    config: *mut WebPDecoderConfig,
) -> VP8StatusCode {
    crate::raw::webp::WebPDecode(data, data_size, config)
}
pub unsafe fn webp_decode_argb(
    data: *const u8,
    data_size: usize,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeARGB(data, data_size, width, height)
}
pub unsafe fn webp_decode_argb_into(
    data: *const u8,
    data_size: usize,
    output_buffer: *mut u8,
    output_buffer_size: usize,
    output_stride: ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeARGBInto(
        data,
        data_size,
        output_buffer,
        output_buffer_size,
        output_stride,
    )
}
pub unsafe fn webp_decode_bgr(
    data: *const u8,
    data_size: usize,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeBGR(
        data,
        data_size,
        width,
        height,
    )
}
pub unsafe fn webp_decode_bgra(
    data: *const u8,
    data_size: usize,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeBGRA(
        data,
        data_size,
        width,
        height,
    )
}
pub unsafe fn webp_decodebgra_into(
    data: *const u8,
    data_size: usize,
    output_buffer: *mut u8,
    output_buffer_size: usize,
    output_stride: ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeBGRAInto(
        data,
        data_size,
        output_buffer,
        output_buffer_size,
        output_stride,
    )
}
pub unsafe fn webp_decode_bgr_into(
    data: *const u8,
    data_size: usize,
    output_buffer: *mut u8,
    output_buffer_size: usize,
    output_stride: ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeBGRInto(
        data,
        data_size,
        output_buffer,
        output_buffer_size,
        output_stride,
    )
}
pub unsafe fn webp_decode_rgb(
    data: *const u8,
    data_size: usize,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeRGB(
        data,
        data_size,
        width,
        height,
    )
}
pub unsafe fn webp_decode_rgba(
    data: *const u8,
    data_size: usize,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeRGBA(
        data,
        data_size,
        width,
        height,
    )
}
pub unsafe fn webp_decode_rgba_into(
    data: *const u8,
    data_size: usize,
    output_buffer: *mut u8,
    output_buffer_size: usize,
    output_stride: ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeRGBAInto(
        data,
        data_size,
        output_buffer,
        output_buffer_size,
        output_stride,
    )
}
pub unsafe fn webp_decode_rgb_into(
    data: *const u8,
    data_size: usize,
    output_buffer: *mut u8,
    output_buffer_size: usize,
    output_stride: ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeRGBInto(
        data,
        data_size,
        output_buffer,
        output_buffer_size,
        output_stride,
    )
}
pub unsafe fn webp_decode_yuv(
    data: *const u8,
    data_size: usize,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
    u: *mut *mut u8,
    v: *mut *mut u8,
    stride: *mut ::std::os::raw::c_int,
    uv_stride: *mut ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeYUV(
        data,
        data_size,
        width,
        height,
        u,
        v,
        stride,
        uv_stride,
    )
}
pub unsafe fn webp_decode_yuv_into(
    data: *const u8,
    data_size: usize,
    luma: *mut u8,
    luma_size: usize,
    luma_stride: ::std::os::raw::c_int,
    u: *mut u8,
    u_size: usize,
    u_stride: ::std::os::raw::c_int,
    v: *mut u8,
    v_size: usize,
    v_stride: ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPDecodeYUVInto(
        data,
        data_size,
        luma,
        luma_size,
        luma_stride,
        u,
        u_size,
        u_stride,
        v,
        v_size,
        v_stride,
    )
}
pub unsafe fn webp_encode(
    config: *const WebPConfig,
    picture: *mut WebPPicture,
) -> c_int {
    crate::raw::webp::WebPEncode(config, picture)
}
pub unsafe fn webp_encode_bgr(
    bgr: *const u8,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    stride: ::std::os::raw::c_int,
    quality_factor: f32,
    output: *mut *mut u8,
) -> usize {
    crate::raw::webp::WebPEncodeBGR(
        bgr,
        width,
        height,
        stride,
        quality_factor,
        output,
    )
}
pub unsafe fn webp_encode_bgra(
    bgra: *const u8,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    stride: ::std::os::raw::c_int,
    quality_factor: f32,
    output: *mut *mut u8,
) -> usize {
    crate::raw::webp::WebPEncodeBGRA(
        bgra,
        width,
        height,
        stride,
        quality_factor,
        output,
    )
}
pub unsafe fn webp_encode_lossless_bgr(
    bgr: *const u8,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    stride: ::std::os::raw::c_int,
    output: *mut *mut u8,
) -> usize {
    crate::raw::webp::WebPEncodeLosslessBGR(
        bgr,
        width,
        height,
        stride,
        output,
    )
}
pub unsafe fn webp_encode_lossless_bgra(
    rgba: *const u8,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    stride: ::std::os::raw::c_int,
    output: *mut *mut u8,
) -> usize {
    crate::raw::webp::WebPEncodeLosslessBGRA(
        rgba,
        width,
        height,
        stride,
        output,
    )
}
pub unsafe fn webp_encode_lossless_rgb(
    rgb: *const u8,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    stride: ::std::os::raw::c_int,
    output: *mut *mut u8,
) -> usize {
    crate::raw::webp::WebPEncodeLosslessRGB(
        rgb,
        width,
        height,
        stride,
        output,
    )
}
pub unsafe fn webp_encode_lossless_rgba(
    rgba: *const u8,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    stride: ::std::os::raw::c_int,
    output: *mut *mut u8,
) -> usize {
    crate::raw::webp::WebPEncodeLosslessRGBA(
        rgba,
        width,
        height,
        stride,
        output,
    )
}
pub unsafe fn webp_encode_rgb(
    rgb: *const u8,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    stride: ::std::os::raw::c_int,
    quality_factor: f32,
    output: *mut *mut u8,
) -> usize {
    crate::raw::webp::WebPEncodeRGB(
        rgb,
        width,
        height,
        stride,
        quality_factor,
        output,
    )
}
pub unsafe fn webp_encode_rgba(
    rgba: *const u8,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    stride: ::std::os::raw::c_int,
    quality_factor: f32,
    output: *mut *mut u8,
) -> usize {
    crate::raw::webp::WebPEncodeRGBA(
        rgba,
        width,
        height,
        stride,
        quality_factor,
        output,
    )
}
pub unsafe fn webp_free(ptr: *mut ::std::os::raw::c_void) {
    crate::raw::webp::WebPFree(ptr)
}
pub unsafe fn webp_free_dec_buffer(buffer: *mut WebPDecBuffer) {
    crate::raw::webp::WebPFreeDecBuffer(buffer)
}
pub unsafe fn webp_get_decoder_version() -> c_int {
    crate::raw::webp::WebPGetDecoderVersion()
}
pub unsafe fn webp_get_encoder_version() -> c_int {
    crate::raw::webp::WebPGetEncoderVersion()
}
pub unsafe fn webp_get_features_internal(
    arg1: *const u8,
    arg2: usize,
    arg3: *mut WebPBitstreamFeatures,
    arg4: ::std::os::raw::c_int,
) -> VP8StatusCode {
    crate::raw::webp::WebPGetFeaturesInternal(
        arg1,
        arg2,
        arg3,
        arg4,
    )
}
pub unsafe fn webp_get_info(
    data: *const u8,
    data_size: usize,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPGetInfo(
        data,
        data_size,
        width,
        height,
    )
}
pub unsafe fn webp_iappend(
    idec: *mut WebPIDecoder,
    data: *const u8,
    data_size: usize
) -> VP8StatusCode {
    crate::raw::webp::WebPIAppend(
        idec,
        data,
        data_size,
    )
}
pub unsafe fn webp_idec_get_rgb(
    idec: *const WebPIDecoder,
    last_y: *mut ::std::os::raw::c_int,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
    stride: *mut ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPIDecGetRGB(
        idec,
        last_y,
        width,
        height,
        stride,
    )
}
pub unsafe fn webp_idec_get_yuva(
    idec: *const WebPIDecoder,
    last_y: *mut ::std::os::raw::c_int,
    u: *mut *mut u8,
    v: *mut *mut u8,
    a: *mut *mut u8,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
    stride: *mut ::std::os::raw::c_int,
    uv_stride: *mut ::std::os::raw::c_int,
    a_stride: *mut ::std::os::raw::c_int,
) -> *mut u8 {
    crate::raw::webp::WebPIDecGetYUVA(
        idec,
        last_y,
        u,
        v,
        a,
        width,
        height,
        stride,
        uv_stride,
        a_stride,
    )
}
pub unsafe fn webp_idecode(
    data: *const u8,
    data_size: usize,
    config: *mut WebPDecoderConfig,
) -> *mut WebPIDecoder {
    crate::raw::webp::WebPIDecode(
        data,
        data_size,
        config,
    )
}
pub unsafe fn webp_idecoded_area(
    idec: *const WebPIDecoder,
    left: *mut ::std::os::raw::c_int,
    top: *mut ::std::os::raw::c_int,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> *const WebPDecBuffer {
    crate::raw::webp::WebPIDecodedArea(
        idec,
        left,
        top,
        width,
        height,
    )
}
pub unsafe fn webp_idelete(idec: *mut WebPIDecoder) {
    crate::raw::webp::WebPIDelete(idec)
}
pub unsafe fn webp_inew_decoder(output_buffer: *mut WebPDecBuffer) -> *mut WebPIDecoder {
    crate::raw::webp::WebPINewDecoder(output_buffer)
}
pub unsafe fn webp_inew_rgb(
    csp: WebPCSPMode,
    output_buffer: *mut u8,
    output_buffer_size: usize,
    output_stride: ::std::os::raw::c_int,
) -> *mut WebPIDecoder {
    crate::raw::webp::WebPINewRGB(
        csp,
        output_buffer,
        output_buffer_size,
        output_stride,
    )
}
pub unsafe fn webp_inew_yuv(
    luma: *mut u8,
    luma_size: usize,
    luma_stride: ::std::os::raw::c_int,
    u: *mut u8,
    u_size: usize,
    u_stride: ::std::os::raw::c_int,
    v: *mut u8,
    v_size: usize,
    v_stride: ::std::os::raw::c_int,
) -> *mut WebPIDecoder {
    crate::raw::webp::WebPINewYUV(
        luma,
        luma_size,
        luma_stride,
        u,
        u_size,
        u_stride,
        v,
        v_size,
        v_stride,
    )
}
pub unsafe fn webp_inew_yuva(
    luma: *mut u8,
    luma_size: usize,
    luma_stride: ::std::os::raw::c_int,
    u: *mut u8,
    u_size: usize,
    u_stride: ::std::os::raw::c_int,
    v: *mut u8,
    v_size: usize,
    v_stride: ::std::os::raw::c_int,
    a: *mut u8,
    a_size: usize,
    a_stride: ::std::os::raw::c_int,
) -> *mut WebPIDecoder {
    crate::raw::webp::WebPINewYUVA(
        luma,
        luma_size,
        luma_stride,
        u,
        u_size,
        u_stride,
        v,
        v_size,
        v_stride,
        a,
        a_size,
        a_stride,
    )
}
pub unsafe fn webp_iupdate(
    idec: *mut WebPIDecoder,
    data: *const u8,
    data_size: usize,
) -> VP8StatusCode {
    crate::raw::webp::WebPIUpdate(idec, data, data_size)
}
pub unsafe fn webp_init_dec_buffer_internal(
    arg1: *mut WebPDecBuffer,
    arg2: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPInitDecBufferInternal(arg1, arg2)
}
pub unsafe fn webp_init_decoder_config_internal(
    arg1: *mut WebPDecoderConfig,
    arg2: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPInitDecoderConfigInternal(arg1, arg2)
}
pub unsafe fn webp_memory_write(
    data: *const u8,
    data_size: usize,
    picture: *const WebPPicture,
) -> c_int {
    crate::raw::webp::WebPMemoryWrite(data, data_size, picture)
}
pub unsafe fn webp_memory_writer_clear(writer: *mut WebPMemoryWriter) {
    crate::raw::webp::WebPMemoryWriterClear(writer)
}
pub unsafe fn webp_memory_writer_init(writer: *mut WebPMemoryWriter) {
    crate::raw::webp::WebPMemoryWriterInit(writer)
}
pub unsafe fn webp_picture_argb_to_yuva(
    picture: *mut WebPPicture,
    arg1: WebPEncCSP,
) -> c_int {
    crate::raw::webp::WebPPictureARGBToYUVA(picture, arg1)
}
pub unsafe fn webp_picture_argb_to_yuva_dithered(
    picture: *mut WebPPicture,
    colorspace: WebPEncCSP,
    dithering: f32,
) -> c_int {
    crate::raw::webp::WebPPictureARGBToYUVADithered(
        picture,
        colorspace,
        dithering,
    )
}
pub unsafe fn webp_picture_alloc(picture: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureAlloc(picture)
}
pub unsafe fn webp_picture_copy(src: *const WebPPicture, dst: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureCopy(src, dst)
}
pub unsafe fn webp_picture_crop(
    picture: *mut WebPPicture,
    left: ::std::os::raw::c_int,
    top: ::std::os::raw::c_int,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureCrop(
        picture,
        left,
        top,
        width,
        height,
    )
}
pub unsafe fn webp_picture_distortion(
    src: *const WebPPicture,
    ref_: *const WebPPicture,
    metric_type: ::std::os::raw::c_int,
    result: *mut f32,
) -> c_int {
    crate::raw::webp::WebPPictureDistortion(
        src,
        ref_,
        metric_type,
        result,
    )
}
pub unsafe fn webp_picture_free(picture: *mut WebPPicture) {
    crate::raw::webp::WebPPictureFree(picture)
}
pub unsafe fn webp_picture_has_transparency(picture: *const WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureHasTransparency(picture)
}
pub unsafe fn webp_picture_import_bgr(
    picture: *mut WebPPicture,
    bgr: *const u8,
    bgr_stride: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureImportBGR(
        picture,
        bgr,
        bgr_stride,
    )
}
pub unsafe fn webp_picture_import_bgra(
    picture: *mut WebPPicture,
    bgra: *const u8,
    bgra_stride: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureImportBGRA(
        picture,
        bgra,
        bgra_stride,
    )
}
pub unsafe fn webp_picture_import_bgrx(
    picture: *mut WebPPicture,
    bgrx: *const u8,
    bgrx_stride: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureImportBGRX(
        picture,
        bgrx,
        bgrx_stride,
    )
}
pub unsafe fn webp_picture_import_rgb(
    picture: *mut WebPPicture,
    rgb: *const u8,
    rgb_stride: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureImportRGB(
        picture,
        rgb,
        rgb_stride,
    )
}
pub unsafe fn webp_picture_import_rgba(
    picture: *mut WebPPicture,
    rgba: *const u8,
    rgba_stride: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureImportRGBA(
        picture,
        rgba,
        rgba_stride,
    )
}
pub unsafe fn webp_picture_import_rgbx(
    picture: *mut WebPPicture,
    rgbx: *const u8,
    rgbx_stride: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureImportRGBX(
        picture,
        rgbx,
        rgbx_stride,
    )
}
pub unsafe fn webp_picture_init_internal(
    arg1: *mut WebPPicture,
    arg2: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureInitInternal(arg1, arg2)
}
pub unsafe fn webp_picture_is_view(picture: *const WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureIsView(picture)
}
pub unsafe fn webp_picture_rescale(
    pic: *mut WebPPicture,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureRescale(pic, width, height)
}
pub unsafe fn webp_picture_sharp_argb_to_yuva(picture: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureSharpARGBToYUVA(picture)
}
pub unsafe fn webp_picture_smart_argb_to_yuva(picture: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureSmartARGBToYUVA(picture)
}
pub unsafe fn webp_picture_view(
    src: *const WebPPicture,
    left: ::std::os::raw::c_int,
    top: ::std::os::raw::c_int,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    dst: *mut WebPPicture,
) -> c_int {
    crate::raw::webp::WebPPictureView(
        src,
        left,
        top,
        width,
        height,
        dst,
    )
}
pub unsafe fn webp_picture_yuva_to_argb(picture: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureYUVAToARGB(picture)
}
pub unsafe fn webp_plane_distortion(
    src: *const u8,
    src_stride: usize,
    ref_: *const u8,
    ref_stride: usize,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    x_step: usize,
    type_: ::std::os::raw::c_int,
    distortion: *mut f32,
    result: *mut f32,
) -> c_int {
    crate::raw::webp::WebPPlaneDistortion(
        src,
        src_stride,
        ref_,
        ref_stride,
        width,
        height,
        x_step,
        type_,
        distortion,
        result,
    )
}
pub unsafe fn webp_validate_config(config: *const WebPConfig) -> c_int {
    crate::raw::webp::WebPValidateConfig(config)
}

#[link(name = "cbits")]
extern "C" {
    pub fn webp_config_init(config: *mut WebPConfig) -> c_int;
    pub fn webp_config_preset(
        config: *mut WebPConfig,
        preset: WebPPreset,
        quality: c_float,
    ) -> c_int;
    pub fn webp_picture_init(picture: *mut WebPPicture) -> c_int;
}


mod test {
    use super::*;

    #[test]
    fn test_create_webp_config() {
        let mut config: WebPConfig = unsafe {std::mem::zeroed()};
        unsafe {
            assert!(webp_config_init(&mut config) != 0);
            // assert!(webp_validate_config(&mut config) != 0);
        };
    }

    #[test]
    fn test_create_webp_picture() {
        let mut picture: WebPPicture = unsafe {std::mem::zeroed()};
        unsafe {
            webp_picture_init(&mut picture);
            // assert!(webp_config_init(&mut config) != 0);
            // assert!(webp_validate_config(&mut config) != 0);
        };
    }
}
