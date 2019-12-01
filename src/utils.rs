use std::ffi::{CString, c_void};
use std::os::raw::{c_char, c_int};
use libc::{size_t, c_float};
use crate::sys::webp::{
    self as webp_sys,
    WebPConfig,
    WebPPicture,
};

/// Picture is uninitialized.
pub fn empty_lossless_webp_picture() -> WebPPicture {
    WebPPicture {
        use_argb: 1,

        // YUV input
        colorspace: webp_sys::WEBP_YUV420,
        width: 0,
        height: 0,
        y: std::ptr::null_mut(),
        u: std::ptr::null_mut(),
        v: std::ptr::null_mut(),
        y_stride: 0,
        uv_stride: 0,
        a: std::ptr::null_mut(),
        a_stride: 0,
        pad1: [0, 0],
        
        // ARGB input
        argb: unimplemented!(),
        argb_stride: unimplemented!(),
        pad2: [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ],

        // OUTPUT
        writer: None,
        custom_ptr: std::ptr::null_mut(),
        extra_info_type: 0,
        extra_info: std::ptr::null_mut(),
        
        // STATS AND REPORTS
        stats: std::ptr::null_mut(),
        error_code: webp_sys::VP8_ENC_OK,
        progress_hook: None,
        user_data: std::ptr::null_mut(),
        
        // padding for later use
        pad3: [0, 0, 0],
        
        // Unused for now
        pad4: std::ptr::null_mut(),
        pad5: std::ptr::null_mut(),

        // padding for later use
        pad6: [0, 0, 0, 0, 0, 0, 0, 0],

        // PRIVATE FIELDS
        memory_: std::ptr::null_mut(),
        memory_argb_: std::ptr::null_mut(),
        pad7: [std::ptr::null_mut(), std::ptr::null_mut()],
    }
}

/// Picture is uninitialized.
pub fn empty_lossy_webp_picture() -> WebPPicture {
    let mut picture = empty_lossless_webp_picture();
    picture.use_argb = 0;
    picture
}

