# YUV formats

This crate implements conversions from YUV/YCbCr formats to RGB. It also contains enums/constants for describing color spaces common in video formats.

Currently it's in an early stage, implementing subset needed for decoding AV1/AVIF.

## Implemented

 * [x] Matrix conversion for BT601, BT709, FCC, BT470BG, SMPTE240
 * [ ] YCgCo (untested)
 * [x] Identity pass-through (GBR)
 * [x] Studio and full range colors
 * [x] 8-bit, and 10/12 to 16-bit conversions

## Planned

 * [ ] Gamma conversion
 * [ ] Color primaries conversion
 * [ ] Chroma upsampling
 * [ ] RGB to YUV conversion
 * [ ] YUV pixel convenience functions, like in [`rgb`](//lib.rs/rgb)
 * [ ] 8-to-16-bit expanding conversion
 * [ ] HDR to SDR conversion?
 * [ ] Maybe HDR support?
