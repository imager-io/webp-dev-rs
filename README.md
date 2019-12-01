# About
Rust bindings to `libwebp`, `libwebpdemux`, `imageio`, workarounds for static/inline C functions and some other miscellaneous stuff.

## Modules `raw` & `sys`
Raw, unaltered symbols are under the `raw` module. Also most of the `raw` symbols are re-exported under the `sys` module, with more idiomatic rust naming conventions. E.g. the `WebPValidateConfig` function is renamed to `webp_validate_config`.

## Other
Includes a higher-level `utils` module for some common use cases.

For the purposes of being self contained, utils from `imageio` may be phased out in the future. 

Also working on getting C API comments converted to rust doccomments for exposed functions.