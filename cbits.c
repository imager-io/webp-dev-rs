// Mostly just workarounds for static C functions that can’t be linked
// directly from the Rust FFI system.
#include <webp/encode.h>
#include <webp/decode.h>
#include <webp/types.h>
#include <imageio/image_enc.h>
#include <imageio/image_dec.h>
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

///////////////////////////////////////////////////////////////////////////////
// WEBP ENCODE
///////////////////////////////////////////////////////////////////////////////

int webp_config_init(WebPConfig* config) {
    return WebPConfigInit(config);
}

int webp_config_preset(WebPConfig* config, WebPPreset preset, float quality) {
    return WebPConfigPreset(config, preset, quality);
}

int webp_picture_init(WebPPicture* picture) {
    return WebPPictureInit(picture);
}
