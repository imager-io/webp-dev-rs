#![allow(non_upper_case_globals)]
use std::ffi::{CString, c_void};
use std::os::raw::{c_char, c_int};
use libc::{size_t, c_float};

///////////////////////////////////////////////////////////////////////////////
// WEBP STRUCTS
///////////////////////////////////////////////////////////////////////////////

/// Structure for storing auxiliary statistics.
pub type WebPAuxStats = crate::raw::webp::WebPAuxStats;

/// Features gathered from the bitstream.
pub type WebPBitstreamFeatures = crate::raw::webp::WebPBitstreamFeatures;

/// Compression parameters.
pub type WebPConfig = crate::raw::webp::WebPConfig;

/// WebPDecBuffer: Generic structure for describing the output sample buffer.
pub type WebPDecBuffer = crate::raw::webp::WebPDecBuffer;

/// Main object storing the configuration for advanced decoding.
pub type WebPDecoderConfig = crate::raw::webp::WebPDecoderConfig;

/// Decoding options
pub type WebPDecoderOptions = crate::raw::webp::WebPDecoderOptions;

pub type WebPIDecoder = crate::raw::webp::WebPIDecoder;

/// WebPMemoryWrite: a special WebPWriterFunction that writes to memory using
/// the following WebPMemoryWriter object (to be set as a custom_ptr).
pub type WebPMemoryWriter = crate::raw::webp::WebPMemoryWriter;

/// Main exchange structure (input samples, output bytes, statistics)
pub type WebPPicture = crate::raw::webp::WebPPicture;

/// WebPDecBuffer: Generic structure for describing the output sample buffer.
pub type WebPRGBABuffer = crate::raw::webp::WebPRGBABuffer;

/// WebPDecBuffer: Generic structure for describing the output sample buffer.
pub type WebPYUVABuffer = crate::raw::webp::WebPYUVABuffer;


///////////////////////////////////////////////////////////////////////////////
// TYPE-DEFS
///////////////////////////////////////////////////////////////////////////////

/// Enumeration of the status codes.
pub type VP8StatusCode = crate::raw::webp::VP8StatusCode;

/// Colorspaces.
/// 
/// Note: the naming describes the byte-ordering of packed samples in memory.
/// For instance, MODE_BGRA relates to samples ordered as B,G,R,A,B,G,R,A,...
/// Non-capital names (e.g.:MODE_Argb) relates to pre-multiplied RGB channels.
/// RGBA-4444 and RGB-565 colorspaces are represented by following byte-order:
/// RGBA-4444: [r3 r2 r1 r0 g3 g2 g1 g0], [b3 b2 b1 b0 a3 a2 a1 a0], ...
/// RGB-565: [r4 r3 r2 r1 r0 g5 g4 g3], [g2 g1 g0 b4 b3 b2 b1 b0], ...
/// In the case WEBP_SWAP_16BITS_CSP is defined, the bytes are swapped for
/// these two modes:
/// RGBA-4444: [b3 b2 b1 b0 a3 a2 a1 a0], [r3 r2 r1 r0 g3 g2 g1 g0], ...
/// RGB-565: [g2 g1 g0 b4 b3 b2 b1 b0], [r4 r3 r2 r1 r0 g5 g4 g3], ...
pub type WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE;

/// Color spaces.
pub type WebPEncCSP = crate::raw::webp::WebPEncCSP;

/// Encoding error conditions.
pub type WebPEncodingError = crate::raw::webp::WebPEncodingError;

/// Image characteristics hint for the underlying encoder.
pub type WebPImageHint = crate::raw::webp::WebPImageHint;

/// Enumerate some predefined settings for WebPConfig, depending on the type
/// of source picture.
/// 
/// These presets are used when calling WebPConfigPreset().
pub type WebPPreset = crate::raw::webp::WebPPreset;

/// Progress hook, called from time to time to report progress.
/// 
/// It can return
/// false to request an abort of the encoding process, or true otherwise if
/// everything is OK.
pub type WebPProgressHook = crate::raw::webp::WebPProgressHook;

/// Signature for output function.
/// 
/// Should return true if writing was successful.
/// data/data_size is the segment of data to write, and 'picture' is for
/// reference (and so one can make use of picture->custom_ptr).
pub type WebPWriterFunction = crate::raw::webp::WebPWriterFunction;

///////////////////////////////////////////////////////////////////////////////
// WEBP CONSTANTS
///////////////////////////////////////////////////////////////////////////////

pub const MODE_ARGB: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_ARGB;
pub const MODE_Argb: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_Argb;
pub const MODE_BGR: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_BGR;
pub const MODE_BGRA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_BGRA;
pub const MODE_LAST: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_LAST;
pub const MODE_RGB: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_RGB;
pub const MODE_RGBA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_RGBA;
pub const MODE_RGBA_4444: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_RGBA_4444;
pub const MODE_RGB_565: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_RGB_565;
pub const MODE_YUV: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_YUV;
pub const MODE_YUVA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_YUVA;
pub const MODE_bgrA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_bgrA;
pub const MODE_rgbA: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_rgbA;
pub const MODE_rgbA_4444: WebPCSPMode = crate::raw::webp::WEBP_CSP_MODE_MODE_rgbA_4444;

pub const WEBP_DECODER_ABI_VERSION: u32 = crate::raw::webp::WEBP_DECODER_ABI_VERSION;
pub const WEBP_ENCODER_ABI_VERSION: u32 = crate::raw::webp::WEBP_ENCODER_ABI_VERSION;

pub const WEBP_MAX_DIMENSION: u32 = crate::raw::webp::WEBP_MAX_DIMENSION;

pub const WEBP_CSP_ALPHA_BIT: WebPEncCSP = crate::raw::webp::WebPEncCSP_WEBP_CSP_ALPHA_BIT;
pub const WEBP_CSP_UV_MASK: WebPEncCSP = crate::raw::webp::WebPEncCSP_WEBP_CSP_UV_MASK;
pub const WEBP_YUV420: WebPEncCSP = crate::raw::webp::WebPEncCSP_WEBP_YUV420;
pub const WEBP_YUV420A: WebPEncCSP = crate::raw::webp::WebPEncCSP_WEBP_YUV420A;

pub const VP8_ENC_OK: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_OK;
pub const VP8_ENC_ERROR_OUT_OF_MEMORY: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_OUT_OF_MEMORY;
pub const VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY;
pub const VP8_ENC_ERROR_NULL_PARAMETER: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_NULL_PARAMETER;
pub const VP8_ENC_ERROR_INVALID_CONFIGURATION: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_INVALID_CONFIGURATION;
pub const VP8_ENC_ERROR_BAD_DIMENSION: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_BAD_DIMENSION;
pub const VP8_ENC_ERROR_PARTITION_OVERFLOW: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_PARTITION_OVERFLOW;
pub const VP8_ENC_ERROR_BAD_WRITE: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_BAD_WRITE;
pub const VP8_ENC_ERROR_FILE_TOO_BIG: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_FILE_TOO_BIG;
pub const VP8_ENC_ERROR_USER_ABORT: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_USER_ABORT;
pub const VP8_ENC_ERROR_LAST: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_LAST;
pub const VP8_ENC_ERROR_PARTITION0_OVERFLOW: WebPEncodingError = crate::raw::webp::WebPEncodingError_VP8_ENC_ERROR_PARTITION0_OVERFLOW;

pub const WEBP_HINT_DEFAULT: WebPImageHint = crate::raw::webp::WebPImageHint_WEBP_HINT_DEFAULT;
pub const WEBP_HINT_GRAPH: WebPImageHint = crate::raw::webp::WebPImageHint_WEBP_HINT_GRAPH;
pub const WEBP_HINT_LAST: WebPImageHint = crate::raw::webp::WebPImageHint_WEBP_HINT_LAST;
pub const WEBP_HINT_PHOTO: WebPImageHint = crate::raw::webp::WebPImageHint_WEBP_HINT_PHOTO;
pub const WEBP_HINT_PICTURE: WebPImageHint = crate::raw::webp::WebPImageHint_WEBP_HINT_PICTURE;

pub const WEBP_PRESET_DEFAULT: WebPPreset = crate::raw::webp::WebPPreset_WEBP_PRESET_DEFAULT;
pub const WEBP_PRESET_DRAWING: WebPPreset = crate::raw::webp::WebPPreset_WEBP_PRESET_DRAWING;
pub const WEBP_PRESET_ICON: WebPPreset = crate::raw::webp::WebPPreset_WEBP_PRESET_ICON;
pub const WEBP_PRESET_PHOTO: WebPPreset = crate::raw::webp::WebPPreset_WEBP_PRESET_PHOTO;
pub const WEBP_PRESET_PICTURE: WebPPreset = crate::raw::webp::WebPPreset_WEBP_PRESET_PICTURE;
pub const WEBP_PRESET_TEXT: WebPPreset = crate::raw::webp::WebPPreset_WEBP_PRESET_TEXT;

///////////////////////////////////////////////////////////////////////////////
// WEBP FUNCTIONS
///////////////////////////////////////////////////////////////////////////////


// Remove the transparency information (if present) by blending the color with
// the background color 'background_rgb' (specified as 24bit RGB triplet).
// After this call, all alpha values are reset to 0xff.
pub unsafe fn webp_blend_alpha(pic: *mut WebPPicture, background_rgb: u32) {
    crate::raw::webp::WebPBlendAlpha(pic, background_rgb)
}

/// Helper function: given a width x height plane of RGBA or YUV(A) samples
/// clean-up or smoothen the YUV or RGB samples under fully transparent area,
/// to help compressibility (no guarantee, though).
pub unsafe fn webp_cleanup_transparent_area(picture: *mut WebPPicture) {
    crate::raw::webp::WebPCleanupTransparentArea(picture)
}

/// Internal, version-checked, entry point
pub unsafe fn webp_config_init_internal(
    arg1: *mut WebPConfig,
    arg2: WebPPreset,
    arg3: f32,
    arg4: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPConfigInitInternal(arg1, arg2, arg3, arg4)
}

/// Activate the lossless compression mode with the desired efficiency level
/// between 0 (fastest, lowest compression) and 9 (slower, best compression).
/// 
/// A good default level is '6', providing a fair tradeoff between compression
/// speed and final compressed size.
/// This function will overwrite several fields from config: 'method', 'quality'
/// and 'lossless'. Returns false in case of parameter error.
pub unsafe fn webp_config_lossless_preset(
    config: *mut WebPConfig,
    level: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPConfigLosslessPreset(config, level)
}

/// Non-incremental version. This version decodes the full data at once, taking
/// 'config' into account.
/// 
/// Returns decoding status (which should be VP8_STATUS_OK
/// if the decoding was successful). Note that 'config' cannot be NULL.
pub unsafe fn webp_decode(
    data: *const u8,
    data_size: usize,
    config: *mut WebPDecoderConfig,
) -> VP8StatusCode {
    crate::raw::webp::WebPDecode(data, data_size, config)
}

/// Same as WebPDecodeRGBA, but returning A, R, G, B, A, R, G, B... ordered data.
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

/// Same as WebPDecodeRGB, but returning B, G, R, B, G, R... ordered data.
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

/// Same as WebPDecodeRGBA, but returning B, G, R, A, B, G, R, A... ordered data.
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

/// Same as WebPDecodeRGBA, but returning R, G, B, R, G, B... ordered data.
/// If the bitstream contains transparency, it is ignored.
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

/// Decodes WebP images pointed to by 'data' and returns RGBA samples, along
/// with the dimensions in *width and *height.
/// 
/// The ordering of samples in
/// memory is R, G, B, A, R, G, B, A... in scan order (endian-independent).
/// The returned pointer should be deleted calling WebPFree().
/// Returns NULL in case of error.
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

/// These five functions are variants of the above ones, that decode the image
/// directly into a pre-allocated buffer 'output_buffer'.
/// 
/// The maximum storage
/// available in this buffer is indicated by 'output_buffer_size'. If this
/// storage is not sufficient (or an error occurred), NULL is returned.
/// Otherwise, output_buffer is returned, for convenience.
/// The parameter 'output_stride' specifies the distance (in bytes)
/// between scanlines. Hence, output_buffer_size is expected to be at least
/// output_stride x picture-height.
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

/// RGB and BGR variants. Here too the transparency information, if present,
/// will be dropped and ignored.
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

/// Decode WebP images pointed to by 'data' to Y'UV format(*).
/// 
/// The pointer
/// returned is the Y samples buffer. Upon return, *u and *v will point to
/// the U and V chroma data. These U and V buffers need NOT be passed to
/// WebPFree(), unlike the returned Y luma one. The dimension of the U and V
/// planes are both (*width + 1) / 2 and (*height + 1)/ 2.
/// Upon return, the Y buffer has a stride returned as '*stride', while U and V
/// have a common stride returned as '*uv_stride'.
/// Return NULL in case of error.
/// (*) Also named Y'CbCr. See: http://en.wikipedia.org/wiki/YCbCr
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

/// WebPDecodeYUVInto() is a variant of WebPDecodeYUV() that operates directly
/// into pre-allocated luma/chroma plane buffers.
/// 
/// This function requires the
/// strides to be passed: one for the luma plane and one for each of the
/// chroma ones. The size of each plane buffer is passed as 'luma_size',
/// 'u_size' and 'v_size' respectively.
/// Pointer to the luma plane ('*luma') is returned or NULL if an error occurred
/// during decoding (or because some buffers were found to be too small).
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

/// Main call
/// 
/// Main encoding call, after config and picture have been initialized.
/// 'picture' must be less than 16384x16384 in dimension (cf WEBP_MAX_DIMENSION),
/// and the 'config' object must be a valid one.
/// Returns false in case of error, true otherwise.
/// In case of error, picture->error_code is updated accordingly.
/// 'picture' can hold the source samples in both YUV(A) or ARGB input, depending
/// on the value of 'picture->use_argb'. It is highly recommended to use
/// the former for lossy encoding, and the latter for lossless encoding
/// (when config.lossless is true). Automatic conversion from one format to
/// another is provided but they both incur some loss.
pub unsafe fn webp_encode(
    config: *const WebPConfig,
    picture: *mut WebPPicture,
) -> c_int {
    crate::raw::webp::WebPEncode(config, picture)
}

/// One-stop-shop call! No questions asked:
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

/// One-stop-shop call! No questions asked:
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

/// One-stop-shop call! No questions asked:
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

/// One-stop-shop call! No questions asked:
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

/// One-stop-shop call! No questions asked:
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

/// One-stop-shop call! No questions asked:
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

/// One-stop-shop call! No questions asked:
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

/// One-stop-shop call! No questions asked:
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

/// Releases memory returned by the WebPDecode*() functions (from decode.h).
pub unsafe fn webp_free(ptr: *mut ::std::os::raw::c_void) {
    crate::raw::webp::WebPFree(ptr)
}

/// Free any memory associated with the buffer. Must always be called last.
/// 
/// Note: doesn't free the 'buffer' structure itself.
pub unsafe fn webp_free_dec_buffer(buffer: *mut WebPDecBuffer) {
    crate::raw::webp::WebPFreeDecBuffer(buffer)
}

/// Return the decoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision.
/// 
/// E.g: v2.5.7 is 0x020507.
pub unsafe fn webp_get_decoder_version() -> c_int {
    crate::raw::webp::WebPGetDecoderVersion()
}

/// Return the encoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision.
/// 
/// E.g: v2.5.7 is 0x020507.
pub unsafe fn webp_get_encoder_version() -> c_int {
    crate::raw::webp::WebPGetEncoderVersion()
}

/// Internal, version-checked, entry point.
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

/// Retrieve basic header information: width, height.
/// 
/// This function will also validate the header, returning true on success,
/// false otherwise. '*width' and '*height' are only valid on successful return.
/// Pointers 'width' and 'height' can be passed NULL if deemed irrelevant.
/// Note: The following chunk sequences (before the raw VP8/VP8L data) are
/// considered valid by this function:
/// RIFF + VP8(L)
/// RIFF + VP8X + (optional chunks) + VP8(L)
/// ALPH + VP8 <-- Not a valid WebP format: only allowed for internal purpose.
/// VP8(L)     <-- Not a valid WebP format: only allowed for internal purpose.
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

/// Copies and decodes the next available data.
/// 
/// Returns VP8_STATUS_OK when
/// the image is successfully decoded. Returns VP8_STATUS_SUSPENDED when more
/// data is expected. Returns error in other cases.
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

/// Returns the RGB/A image decoded so far.
/// 
/// Returns NULL if output params
/// are not initialized yet. The RGB/A output type corresponds to the colorspace
/// specified during call to WebPINewDecoder() or WebPINewRGB().
/// *last_y is the index of last decoded row in raster scan order. Some pointers
/// (*last_y, *width etc.) can be NULL if corresponding information is not
/// needed. The values in these pointers are only valid on successful (non-NULL)
/// return.
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

/// Same as above function to get a YUVA image.
/// 
/// Returns pointer to the luma
/// plane or NULL in case of error. If there is no alpha information
/// the alpha pointer '*a' will be returned NULL.
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

/// Instantiate a new incremental decoder object with the requested
/// configuration.
/// 
/// The bitstream can be passed using 'data' and 'data_size'
/// parameter, in which case the features will be parsed and stored into
/// config->input. Otherwise, 'data' can be NULL and no parsing will occur.
/// Note that 'config' can be NULL too, in which case a default configuration
/// is used. If 'config' is not NULL, it must outlive the WebPIDecoder object
/// as some references to its fields will be used. No internal copy of 'config'
/// is made.
/// The return WebPIDecoder object must always be deleted calling WebPIDelete().
/// Returns NULL in case of error (and config->status will then reflect
/// the error condition, if available).
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

/// Generic call to retrieve information about the displayable area.
/// 
/// If non NULL, the left/right/width/height pointers are filled with the visible
/// rectangular area so far.
/// Returns NULL in case the incremental decoder object is in an invalid state.
/// Otherwise returns the pointer to the internal representation. This structure
/// is read-only, tied to WebPIDecoder's lifespan and should not be modified.
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

/// Deletes the WebPIDecoder object and associated memory.
/// 
/// Must always be called if WebPINewDecoder, WebPINewRGB or WebPINewYUV
/// succeeded.
pub unsafe fn webp_idelete(idec: *mut WebPIDecoder) {
    crate::raw::webp::WebPIDelete(idec)
}

/// Creates a new incremental decoder with the supplied buffer parameter.
/// 
/// This output_buffer can be passed NULL, in which case a default output buffer
/// is used (with MODE_RGB). Otherwise, an internal reference to 'output_buffer'
/// is kept, which means that the lifespan of 'output_buffer' must be larger than
/// that of the returned WebPIDecoder object.
/// The supplied 'output_buffer' content MUST NOT be changed between calls to
/// WebPIAppend() or WebPIUpdate() unless 'output_buffer.is_external_memory' is
/// not set to 0. In such a case, it is allowed to modify the pointers, size and
/// stride of output_buffer.u.RGBA or output_buffer.u.YUVA, provided they remain
/// within valid bounds.
/// All other fields of WebPDecBuffer MUST remain constant between calls.
/// Returns NULL if the allocation failed.
pub unsafe fn webp_inew_decoder(output_buffer: *mut WebPDecBuffer) -> *mut WebPIDecoder {
    crate::raw::webp::WebPINewDecoder(output_buffer)
}

/// This function allocates and initializes an incremental-decoder object, which
/// will output the RGB/A samples specified by 'csp' into a preallocated
/// buffer 'output_buffer'.
/// 
/// The size of this buffer is at least
/// 'output_buffer_size' and the stride (distance in bytes between two scanlines)
/// is specified by 'output_stride'.
/// Additionally, output_buffer can be passed NULL in which case the output
/// buffer will be allocated automatically when the decoding starts. The
/// colorspace 'csp' is taken into account for allocating this buffer. All other
/// parameters are ignored.
/// Returns NULL if the allocation failed, or if some parameters are invalid.
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

/// Deprecated version of the above, without the alpha plane.
/// 
/// Kept for backward compatibility.
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

/// This function allocates and initializes an incremental-decoder object, which
/// will output the raw luma/chroma samples into a preallocated planes if
/// supplied.
/// 
/// The luma plane is specified by its pointer 'luma', its size
/// 'luma_size' and its stride 'luma_stride'. Similarly, the chroma-u plane
/// is specified by the 'u', 'u_size' and 'u_stride' parameters, and the chroma-v
/// plane by 'v' and 'v_size'. And same for the alpha-plane. The 'a' pointer
/// can be pass NULL in case one is not interested in the transparency plane.
/// Conversely, 'luma' can be passed NULL if no preallocated planes are supplied.
/// In this case, the output buffer will be automatically allocated (using
/// MODE_YUVA) when decoding starts. All parameters are then ignored.
/// Returns NULL if the allocation failed or if a parameter is invalid.
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

/// A variant of the above function to be used when data buffer contains
/// partial data from the beginning.
/// 
/// In this case data buffer is not copied
/// to the internal memory.
/// Note that the value of the 'data' pointer can change between calls to
/// WebPIUpdate, for instance when the data buffer is resized to fit larger data.
pub unsafe fn webp_iupdate(
    idec: *mut WebPIDecoder,
    data: *const u8,
    data_size: usize,
) -> VP8StatusCode {
    crate::raw::webp::WebPIUpdate(idec, data, data_size)
}

/// Internal, version-checked, entry point
pub unsafe fn webp_init_dec_buffer_internal(
    arg1: *mut WebPDecBuffer,
    arg2: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPInitDecBufferInternal(arg1, arg2)
}

/// Internal, version-checked, entry point
pub unsafe fn webp_init_decoder_config_internal(
    arg1: *mut WebPDecoderConfig,
    arg2: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPInitDecoderConfigInternal(arg1, arg2)
}

/// WebPMemoryWrite: a special WebPWriterFunction that writes to memory using
/// the following WebPMemoryWriter object (to be set as a custom_ptr).
pub unsafe fn webp_memory_write(
    data: *const u8,
    data_size: usize,
    picture: *const WebPPicture,
) -> c_int {
    crate::raw::webp::WebPMemoryWrite(data, data_size, picture)
}

/// The following must be called to deallocate writer->mem memory.
/// 
/// The 'writer'
/// object itself is not deallocated.
pub unsafe fn webp_memory_writer_clear(writer: *mut WebPMemoryWriter) {
    crate::raw::webp::WebPMemoryWriterClear(writer)
}

/// The following must be called first before any use.
pub unsafe fn webp_memory_writer_init(writer: *mut WebPMemoryWriter) {
    crate::raw::webp::WebPMemoryWriterInit(writer)
}

/// Converts picture->argb data to the YUV420A format.
/// 
/// The 'colorspace'
/// parameter is deprecated and should be equal to WEBP_YUV420.
/// Upon return, picture->use_argb is set to false. The presence of real
/// non-opaque transparent values is detected, and 'colorspace' will be
/// adjusted accordingly. Note that this method is lossy.
/// Returns false in case of error.
pub unsafe fn webp_picture_argb_to_yuva(
    picture: *mut WebPPicture,
    arg1: WebPEncCSP,
) -> c_int {
    crate::raw::webp::WebPPictureARGBToYUVA(picture, arg1)
}

/// Same as WebPPictureARGBToYUVA(), but the conversion is done using
/// pseudo-random dithering with a strength 'dithering' between
/// 0.0 (no dithering) and 1.0 (maximum dithering).
/// 
/// This is useful for photographic picture.
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


/// Convenience allocation / deallocation based on picture->width/height
/// 
/// Allocate y/u/v buffers as per colorspace/width/height specification.
/// Note! This function will free the previous buffer if needed.
/// Returns false in case of memory error.
pub unsafe fn webp_picture_alloc(picture: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureAlloc(picture)
}

/// Copy the pixels of *src into *dst, using WebPPictureAlloc.
/// 
/// Upon return, *dst
/// will fully own the copied pixels (this is not a view). The 'dst' picture need
/// not be initialized as its content is overwritten.
/// Returns false in case of memory allocation error.
pub unsafe fn webp_picture_copy(src: *const WebPPicture, dst: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureCopy(src, dst)
}

/// self-crops a picture to the rectangle defined by top/left/width/height.
/// 
/// Returns false in case of memory allocation error, or if the rectangle is
/// outside of the source picture.
/// The rectangle for the view is defined by the top-left corner pixel
/// coordinates (left, top) as well as its width and height. This rectangle
/// must be fully be comprised inside the 'src' source picture. If the source
/// picture uses the YUV420 colorspace, the top and left coordinates will be
/// snapped to even values.
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

/// Compute PSNR, SSIM or LSIM distortion metric between two pictures.
/// 
/// Results
/// are in dB, stored in result[] in the B/G/R/A/All order. The distortion is
/// always performed using ARGB samples. Hence if the input is YUV(A), the
/// picture will be internally converted to ARGB (just for the measurement).
/// Warning: this function is rather CPU-intensive.
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

/// Release the memory allocated by WebPPictureAlloc() or WebPPictureImport*().
/// 
/// Note that this function does _not_ free the memory used by the 'picture'
/// object itself.
/// Besides memory (which is reclaimed) all other fields of 'picture' are
/// preserved.
pub unsafe fn webp_picture_free(picture: *mut WebPPicture) {
    crate::raw::webp::WebPPictureFree(picture)
}

/// Scan the picture 'picture' for the presence of non fully opaque alpha values.
/// 
/// Returns true in such case. Otherwise returns false (indicating that the
/// alpha plane can be ignored altogether e.g.).
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

/// Colorspace conversion function to import RGB samples.
/// 
/// Previous buffer will be free'd, if any.
/// *rgb buffer should have a size of at least height * rgb_stride.
/// Returns false in case of memory error.
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

/// Same as `webp_picture_import_rgb`, but for RGBA buffer.
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

/// Imports the RGB direct from the 32-bit format
/// input buffer ignoring the alpha channel.
/// 
/// Avoids needing to copy the data
/// to a temporary 24-bit RGB buffer to import the RGB only.
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

/// Internal, version-checked, entry point
pub unsafe fn webp_picture_init_internal(
    arg1: *mut WebPPicture,
    arg2: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureInitInternal(arg1, arg2)
}

/// Returns true if the 'picture' is actually a view and therefore does
/// not own the memory for pixels.
pub unsafe fn webp_picture_is_view(picture: *const WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureIsView(picture)
}

/// Rescale a picture to new dimension width x height.
/// 
/// If either 'width' or 'height' (but not both) is 0 the corresponding
/// dimension will be calculated preserving the aspect ratio.
/// No gamma correction is applied.
/// Returns false in case of error (invalid parameter or insufficient memory).
pub unsafe fn webp_picture_rescale(
    pic: *mut WebPPicture,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
) -> c_int {
    crate::raw::webp::WebPPictureRescale(pic, width, height)
}

/// Performs 'sharp' RGBA->YUVA420 downsampling and colorspace conversion.
/// 
/// Downsampling is handled with extra care in case of color clipping. This
/// method is roughly 2x slower than WebPPictureARGBToYUVA() but produces better
/// and sharper YUV representation.
/// Returns false in case of error.
pub unsafe fn webp_picture_sharp_argb_to_yuva(picture: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureSharpARGBToYUVA(picture)
}

/// kept for backward compatibility
pub unsafe fn webp_picture_smart_argb_to_yuva(picture: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureSmartARGBToYUVA(picture)
}

/// Extracts a view from 'src' picture into 'dst'.
/// 
/// The rectangle for the view
/// is defined by the top-left corner pixel coordinates (left, top) as well
/// as its width and height. This rectangle must be fully be comprised inside
/// the 'src' source picture. If the source picture uses the YUV420 colorspace,
/// the top and left coordinates will be snapped to even values.
/// Picture 'src' must out-live 'dst' picture. Self-extraction of view is allowed
/// ('src' equal to 'dst') as a mean of fast-cropping (but note that doing so,
/// the original dimension will be lost). Picture 'dst' need not be initialized
/// with WebPPictureInit() if it is different from 'src', since its content will
/// be overwritten.
/// Returns false in case of memory allocation error or invalid parameters.
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

/// Converts picture->yuv to picture->argb and sets picture->use_argb to true.
/// 
/// The input format must be YUV_420 or YUV_420A. The conversion from YUV420 to
/// ARGB incurs a small loss too.
/// Note that the use of this colorspace is discouraged if one has access to the
/// raw ARGB samples, since using YUV420 is comparatively lossy.
/// Returns false in case of error.
pub unsafe fn webp_picture_yuva_to_argb(picture: *mut WebPPicture) -> c_int {
    crate::raw::webp::WebPPictureYUVAToARGB(picture)
}

/// Compute the single distortion for packed planes of samples.
/// 
/// 'src' will be compared to 'ref', and the raw distortion stored into
/// '*distortion'. The refined metric (log(MSE), log(1 - ssim),...' will be
/// stored in '*result'.
/// 'x_step' is the horizontal stride (in bytes) between samples.
/// 'src/ref_stride' is the byte distance between rows.
/// Returns false in case of error (bad parameter, memory allocation error, ...).
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

/// Returns true if 'config' is non-NULL and all configuration parameters are
/// within their valid ranges.
pub unsafe fn webp_validate_config(config: *const WebPConfig) -> c_int {
    crate::raw::webp::WebPValidateConfig(config)
}

#[link(name = "cbits")]
extern "C" {
    /// Should always be called, to initialize a fresh WebPConfig structure before
    /// modification.
    /// 
    /// Returns false in case of version mismatch. WebPConfigInit()
    /// must have succeeded before using the 'config' object.
    /// Note that the default values are lossless=0 and quality=75.
    pub fn webp_config_init(config: *mut WebPConfig) -> c_int;

    /// This function will initialize the configuration according to a predefined
    /// set of parameters (referred to by 'preset') and a given quality factor.
    /// 
    /// This function can be called as a replacement to WebPConfigInit(). Will
    /// return false in case of error.
    pub fn webp_config_preset(
        config: *mut WebPConfig,
        preset: WebPPreset,
        quality: c_float,
    ) -> c_int;

    /// Should always be called, to initialize the structure.
    /// 
    /// Returns false in case
    /// of version mismatch. WebPPictureInit() must have succeeded before using the
    /// 'picture' object.
    /// Note that, by default, use_argb is false and colorspace is WEBP_YUV420.
    pub fn webp_picture_init(picture: *mut WebPPicture) -> c_int;
}


mod test {
    use super::*;

    #[test]
    fn test_create_webp_config() {
        let mut config: WebPConfig = unsafe {std::mem::zeroed()};
        unsafe {
            assert!(webp_config_init(&mut config) != 0);
            assert!(webp_validate_config(&mut config) != 0);
        };
    }

    #[test]
    fn test_create_webp_picture() {
        let mut picture: WebPPicture = unsafe {std::mem::zeroed()};
        unsafe {
            webp_picture_init(&mut picture);
        };
    }
}
