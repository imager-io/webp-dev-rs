use std::ffi::{CString, c_void};
use std::os::raw::{c_char, c_int};
use libc::{size_t, c_float};
use image::{DynamicImage, GenericImage, GenericImageView};
// use crate::sys::webp::WebPPicture;

// pub fn image_to_webp_picture(img: DynamicImage) -> WebPPicture {
//     unimplemented!()
// }