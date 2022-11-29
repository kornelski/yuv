//! This crate contains two things:
//!
//! 1. [`color`] `enum`s that can be used to describe color spaces in image and video formats, as defined in ISO/IEC 23091-4/ITU-T H.273
//! 2. Routines to [`convert`] between YUV family of color spaces and RGB.

#![cfg_attr(feature="no_std", no_std, feature(error_in_core))]

/// Enums describing color characteristics (color space, gamma, range)
///
/// The numbers should be compatible with ISO/IEC 23091-4/ITU-T H.273
pub mod color;

pub mod convert;

mod error;
pub use error::Error;

/// These are internal
mod depth;
mod ops;
mod range;

/// A generic 3-component pixel, which is usually luma + chroma
///
/// Color space of these values is described outside of this struct by enums in the [`color`] module.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct YUV<T> {
    pub y: T,
    pub u: T,
    pub v: T,
}

/// An RGB pixel (from the [`rgb`] crate)
pub use rgb::RGB;
/// An RGBA pixel (from the [`rgb`] crate)
#[doc(hidden)]
pub use rgb::RGBA;
