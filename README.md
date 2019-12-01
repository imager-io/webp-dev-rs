# About
Rust bindings to `libwebp`, `libwebpdemux`, `imageio`, workarounds for static/inline C functions and some other miscellaneous stuff.

**Much of the API doc comments from the C header files have been included with their rust FFI counterparts.**

## Modules `raw` & `sys`
Raw, unaltered symbols are under the `raw` module. Also most of the `raw` symbols are re-exported under the `sys` module, with more idiomatic rust naming conventions. E.g. the `WebPValidateConfig` function is renamed to `webp_validate_config`.
