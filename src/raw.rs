#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod webp {
    include!(concat!(env!("OUT_DIR"), "/bindings_webp.rs"));
}

pub mod imageio {
    include!(concat!(env!("OUT_DIR"), "/bindings_imageio.rs"));
}