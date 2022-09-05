//! YUV -> RGB converter. See [`RGBConvert::new`]
use crate::Error;
use std::marker::PhantomData;
use crate::color::*;
use crate::depth;
use crate::range;
use crate::YUV;
use rgb::ComponentMap;
use rgb::RGB;

/// Trait for YUV -> RGB conversion implemented by color-space-specific converters. See [`RGBConvert`]
pub trait ToRGB<F = u8, T = u8> {
    /// Convert YUV (YCbCr, etc.) to RGB
    fn to_rgb(&self, px: YUV<F>) -> RGB<T>;
    /// Ignore UV channels, and just convert Y
    fn to_luma(&self, y: F) -> T;
}

/// Enum containing concrete type of converter used.
///
/// Use [`RGBConvert::new`] to create a new instance.
///
/// Variants of this enum are indivdually optimized for specific color spaces. You can either call [`RGBConvert::to_rgb()`] for convenient method, or
/// match on the enum, and use [`ToRGB`] trait with each of the variants to make Rust generate optimized functions for each.
#[derive(Debug, Clone)]
pub enum RGBConvert<T = u8> {
    /// Converter YCbCr color spaces
    Matrix(Matrix<T>),
    /// No conversion
    Copy(CopyGBR<T>),
    /// Scale numbers from 10/12-bit to 16-bit, and/or from studio range to full range. All channels use Y range.
    IdentityScale(IdentityScale<T>),
}

fn coeffs_for_matrix(matrix_coeffs: MatrixCoefficients) -> Option<(f64, f64)> {
    Some(match matrix_coeffs {
        MatrixCoefficients::BT709 => (0.2126, 0.0722),
        MatrixCoefficients::FCC => (0.30, 0.11),
        MatrixCoefficients::BT470BG |
        MatrixCoefficients::BT601 => (0.299, 0.114),
        MatrixCoefficients::SMPTE240 => (0.212, 0.087),
        MatrixCoefficients::YCgCo => (0.25, 0.25),
        _ => return None,
    })
}

impl RGBConvert<u8> {
    /// Use `RGBConvert::<u8>::new()` to call this method, because there's also a `u16` version
    pub fn new(range: Range, matrix: MatrixCoefficients) -> Result<Self, Error> {
        if let Some((kr, kb)) = coeffs_for_matrix(matrix) {
            return Ok(Self::Matrix(Matrix::<u8>::new(kr, kb, range)));
        }
        if matrix == MatrixCoefficients::Identity {
            return Ok(match range {
                Range::Full => Self::Copy(CopyGBR(PhantomData)),
                Range::Limited => Self::IdentityScale(IdentityScale::<u8>::new()),
            });
        }
        Err(Error::UnsupportedMatrixCoefficients)
    }
}

impl RGBConvert<u16> {
    /// Use `RGBConvert::<u16>::new()` to call this method, because there's also a `u8` version
    pub fn new(range: Range, matrix: MatrixCoefficients, depth: Depth) -> Result<Self, Error> {
        if let Some((kr, kb)) = coeffs_for_matrix(matrix) {
            return Ok(Self::Matrix(Matrix::<u16>::new(kr, kb, range, depth)));
        }
        if matrix == MatrixCoefficients::Identity {
            return Ok(match (range, depth) {
                (Range::Full, Depth::Depth16) => Self::Copy(CopyGBR(PhantomData)),
                _ => Self::IdentityScale(IdentityScale::<u16>::new(range, depth)?),
            });
        }
        Err(Error::UnsupportedMatrixCoefficients)
    }
}

impl<T> RGBConvert<T> where Matrix<T>: ToRGB<T, T>, IdentityScale<T>: ToRGB<T, T> {
    /// Convert a single YUV pixel to an RGB pixel.
    ///
    /// This method has a `match` internally, which may or may not be the fastest way to do this (dependin on optimizer).
    /// If you want to have optimal code, use variants of this `enum` individually. They all implement `ToRGB` trait.
    #[inline(always)]
    pub fn to_rgb(&self, px: YUV<T>) -> RGB<T> {
        match self {
            Self::Matrix(c) => c.to_rgb(px),
            Self::Copy(c) => c.to_rgb(px),
            Self::IdentityScale(c) => c.to_rgb(px),
        }
    }

    /// Convert a single Y (Luma) value to a grayscale value.
    #[inline(always)]
    pub fn to_luma(&self, px: T) -> T {
        match self {
            Self::Matrix(c) => c.to_luma(px),
            Self::Copy(c) => c.to_luma(px),
            Self::IdentityScale(c) => c.to_luma(px),
        }
    }
}

impl<T> ToRGB<T,T> for RGBConvert<T> where Matrix<T>: ToRGB<T, T>, IdentityScale<T>: ToRGB<T, T> {
    /// Convert a single YUV pixel to an RGB pixel.
    ///
    /// This method has a `match` internally, which may or may not be the fastest way to do this (dependin on optimizer).
    /// If you want to have optimal code, use variants of this `enum` individually. They all implement `ToRGB` trait.
    #[inline(always)]
    fn to_rgb(&self, px: YUV<T>) -> RGB<T> {
        RGBConvert::to_rgb(self, px)
    }

    /// Convert a single Y (Luma) value to a grayscale value.
    #[inline(always)]
    fn to_luma(&self, y: T) -> T {
        RGBConvert::to_luma(self, y)
    }
}

/// Fast path when no conversion needed for YUV -> GBR
#[derive(Debug, Copy, Clone)]
pub struct CopyGBR<T = u8>(PhantomData<T>);

impl<T> ToRGB<T, T> for CopyGBR<T> {
    #[inline(always)]
    fn to_rgb(&self, px: YUV<T>) -> RGB<T> {
        RGB::new(px.v, px.y, px.u)
    }

    #[inline(always)]
    fn to_luma(&self, y: T) -> T {
        y
    }
}

/// Rescaling bit range for YUV -> GBR
#[derive(Debug, Copy, Clone)]
pub struct IdentityScale<T = u8> {
    min: T,
    range: T,
}

#[inline(always)]
fn rescale16(v: u16, fmin: u16, frange: u16) -> u16 {
    let v = (v as i32 - fmin as i32).max(0) as u32 * 65536;
    (v / frange as u32).min(65535) as u16
}

#[inline(always)]
fn rescale8(v: u8, fmin: u8, frange: u8) -> u8 {
    let v = (v as i16 - fmin as i16).max(0) as u16 * 256;
    (v / frange as u16).min(255) as u8
}

#[inline(always)]
fn new_scale<R: range::Range>() -> IdentityScale<R::Pixel> {
    IdentityScale {
        min: R::Y_MIN,
        range: R::Y_MAX - R::Y_MIN,
    }
}

impl IdentityScale<u8> {
    #[inline(always)]
    fn new() -> Self {
        new_scale::<range::Limited<depth::Depth8>>()
    }
}

impl IdentityScale<u16> {
    #[inline(always)]
    fn new(range: Range, depth: Depth) -> Result<Self, Error> {
        Ok(match (range, depth) {
            (Range::Limited, Depth::Depth10) => new_scale::<range::Limited<depth::Depth10>>(),
            (Range::Limited, Depth::Depth12) => new_scale::<range::Limited<depth::Depth12>>(),
            (Range::Limited, Depth::Depth16) => new_scale::<range::Limited<depth::Depth16>>(),
            (Range::Full, Depth::Depth10) => new_scale::<range::Full<depth::Depth10>>(),
            (Range::Full, Depth::Depth12) => new_scale::<range::Full<depth::Depth12>>(),
            (Range::Full, Depth::Depth16) => new_scale::<range::Full<depth::Depth16>>(),
            (_, Depth::Depth8) => return Err(Error::InvalidDepthRequested),
        })
    }
}

impl ToRGB<u8, u8> for IdentityScale<u8> {
    #[inline(always)]
    fn to_rgb(&self, px: YUV<u8>) -> RGB<u8> {
        RGB {
            g: rescale8(px.y, self.min, self.range),
            b: rescale8(px.u, self.min, self.range),
            r: rescale8(px.v, self.min, self.range),
        }
    }

    #[inline(always)]
    fn to_luma(&self, y: u8) -> u8 {
        rescale8(y, self.min, self.range)
    }
}

impl ToRGB<u16, u16> for IdentityScale<u16> {
    #[inline(always)]
    fn to_rgb(&self, px: YUV<u16>) -> RGB<u16> {
        RGB {
            g: rescale16(px.y, self.min, self.range),
            b: rescale16(px.u, self.min, self.range),
            r: rescale16(px.v, self.min, self.range),
        }
    }

    #[inline(always)]
    fn to_luma(&self, y: u16) -> u16 {
        rescale16(y, self.min, self.range)
    }
}

/// Converter for YCbCr color spaces
#[derive(Debug, Copy, Clone)]
pub struct Matrix<T = u8> {
    y_scale: range::RangeScale,
    uv_scale: range::RangeScale,
    _pixel: PhantomData<T>,

    // matrix coeffs preprocessed
    a: f32, b: f32, c: f32, d: f32,
}

impl<T> Matrix<T> {
    fn new_internal(kr: f64, kb: f64, y_scale: range::RangeScale, uv_scale: range::RangeScale) -> Self {
        let kg = 1. - kr - kb;
        assert!(kr > 0. && kg > 0. && kb > 0.);
        Self {
            a: (2. * (1. - kr)) as f32,
            b: (2. * (1. - kb)*kb/kg) as f32,
            c: (2. * (1. - kr)*kr/kg) as f32,
            d: (2. * (1. - kb)) as f32,
            y_scale,
            uv_scale,
            _pixel: PhantomData,
        }
    }

    /// Input is in its original range, NOT normalized
    /// Returns range or input `RangeScale` (roughly)
    #[inline(always)]
    fn to_rgbf(&self, px: YUV<f32>) -> RGB<f32> {
        let y = px.y * self.y_scale.mul - self.y_scale.sub;
        RGB {
            r: (0_f32).max(y +  px.v * (self.uv_scale.mul * self.a) - (self.uv_scale.sub * self.a)),
            b: (0_f32).max(y +  px.u * (self.uv_scale.mul * self.d) - (self.uv_scale.sub * self.d)),
            g: (0_f32).max(y - (px.u * (self.uv_scale.mul * self.b) - (self.uv_scale.sub * self.b))
                             - (px.v * (self.uv_scale.mul * self.c) - (self.uv_scale.sub * self.c))),
        }
    }
}

impl Matrix<u8> {
    #[inline]
    fn new(kr: f64, kb: f64, yuv_range: Range) -> Self {
        let (y_scale, uv_scale) = match yuv_range {
            Range::Full => range::to_floats::<range::Full<depth::Depth8>>(255.999),
            Range::Limited => range::to_floats::<range::Limited<depth::Depth8>>(255.999),
        };
        Self::new_internal(kr, kb, y_scale, uv_scale)
    }
}

impl<T> ToRGB<T, u8> for Matrix<T> where T: Into<f32> {
    #[inline]
    fn to_rgb(&self, px: YUV<T>) -> RGB<u8> {
        self.to_rgbf(YUV {
            y: px.y.into(),
            u: px.u.into(),
            v: px.v.into(),
        })
        .map(|c| c.min(255.) as u8)
    }

    #[inline]
    fn to_luma(&self, y: T) -> u8 {
        (y.into() * self.y_scale.mul - self.y_scale.sub) as u8
    }
}

impl Matrix<u16> {
    #[inline]
    fn new(kr: f64, kb: f64, yuv_range: Range, depth: Depth) -> Matrix<u16> {
        let (y_scale, uv_scale) = match (yuv_range, depth) {
            (Range::Full, Depth::Depth8) => range::to_floats::<range::Full<depth::Depth8>>(65535.999),
            (Range::Full, Depth::Depth10) => range::to_floats::<range::Full<depth::Depth10>>(65535.999),
            (Range::Full, Depth::Depth12) => range::to_floats::<range::Full<depth::Depth12>>(65535.999),
            (Range::Full, Depth::Depth16) => range::to_floats::<range::Full<depth::Depth16>>(65535.999),
            (Range::Limited, Depth::Depth8) => range::to_floats::<range::Limited<depth::Depth8>>(65535.999),
            (Range::Limited, Depth::Depth10) => range::to_floats::<range::Limited<depth::Depth10>>(65535.999),
            (Range::Limited, Depth::Depth12) => range::to_floats::<range::Limited<depth::Depth12>>(65535.999),
            (Range::Limited, Depth::Depth16) => range::to_floats::<range::Limited<depth::Depth16>>(65535.999),
        };
        Self::new_internal(kr, kb, y_scale, uv_scale)
    }
}

impl<T> ToRGB<T, u16> for Matrix<T> where T: Into<f32> {
    #[inline]
    fn to_rgb(&self, px: YUV<T>) -> RGB<u16> {
        self.to_rgbf(YUV {
            y: px.y.into(),
            u: px.u.into(),
            v: px.v.into(),
        })
        .map(|c| c as u16)
    }

    #[inline]
    fn to_luma(&self, y: T) -> u16 {
        (y.into() * self.y_scale.mul - self.y_scale.sub) as u16
    }
}

#[test]
fn traits_all_the_way_down() {
    let _ = |f: RGBConvert| -> Box<dyn ToRGB<u8, u8>> { match f {
        RGBConvert::IdentityScale(c) => Box::new(c),
        RGBConvert::Copy(c) => Box::new(c),
        RGBConvert::Matrix(c) => Box::new(c),
    }};
}

#[test]
fn matrix_conv() {
    let m = Matrix::<u8>::new(0.2126, 0.0722, Range::Full);
    let px = m.to_rgbf(YUV{y:222.,u:128.,v:128.}).map(|c| c.floor() as u8);
    assert_eq!(RGB::new(222,222,222), px);
    assert_eq!(222u8, m.to_luma(222u8));
    assert_eq!(0u8, m.to_luma(0u8));
    assert_eq!(255u8, m.to_luma(255u8));

    let px = m.to_rgbf(YUV{y:128.,u:40.,v:160.}).map(|c| c.floor() as u8);
    assert_eq!(RGB::new(179,130,0), px);

    let m = Matrix::<u8>::new(0.2126, 0.0722, Range::Limited);
    let px = m.to_rgbf(YUV{y:128.,u:115.,v:90.}).map(|c| c.floor() as u8);
    assert_eq!(RGB::new((16007u16/256) as u8, (39433u16/256) as u8, (26458u16/256) as u8), px);
    assert_eq!(0u8, m.to_luma(16u8));
    assert_eq!(2u8, m.to_luma(18u8));
    assert_eq!(0u8, m.to_luma(0u8));
    assert_eq!(255u8, m.to_luma(240u8));
    assert_eq!(255u8, m.to_luma(255u8));

    let m = Matrix::<u16>::new(0.2126, 0.0722, Range::Limited, Depth::Depth10);
    let px = m.to_rgbf(YUV{y:4.*128.,u:4.*115.,v:4.*90.}).map(|c| c.floor() as u16);
    assert_eq!(RGB::new(16007, 39433, 26458), px);

    let m = Matrix::<u16>::new(0.2126, 0.0722, Range::Limited, Depth::Depth12);
    let px = m.to_rgbf(YUV{y:16.*128.,u:16.*115.,v:16.*90.}).map(|c| c.floor() as u16);
    assert_eq!(RGB::new(16007, 39433, 26458), px);
    assert_eq!(0u16, m.to_luma(0u16));
    assert_eq!(5592u16, m.to_luma(555u16));
}
