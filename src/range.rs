use crate::depth::{Bounded, Depth, Depth10, Depth12, Depth16, Depth8};
use crate::ops::CheckedAs;
use num_traits::PrimInt;

pub(crate) trait Range: 'static {
    type Pixel: Bounded + PrimInt;

    const Y_MIN: Self::Pixel;
    const Y_MAX: Self::Pixel;
    const UV_MIN: Self::Pixel;
    const UV_MAX: Self::Pixel;
}

/// Full numeric range (no rescaling needed)
pub(crate) struct Full<D>(pub D);

/// TV/Studio range
pub(crate) struct Limited<D>(pub D);

impl<D: Depth> Range for Full<D> {
    type Pixel = D::Pixel;

    const Y_MIN: D::Pixel = D::Pixel::MIN;
    const Y_MAX: D::Pixel = D::MAX;
    const UV_MIN: D::Pixel = D::Pixel::MIN;
    const UV_MAX: D::Pixel = D::MAX;
}

impl Range for Limited<Depth8> {
    type Pixel = u8;

    const Y_MIN: u8 = 16;
    const Y_MAX: u8 = 235;
    const UV_MIN: u8 = 16;
    const UV_MAX: u8 = 240;
}

impl Range for Limited<Depth10> {
    type Pixel = u16;

    const Y_MIN: u16 = 64;
    const Y_MAX: u16 = 940;
    const UV_MIN: u16 = 64;
    const UV_MAX: u16 = 960;
}

impl Range for Limited<Depth12> {
    type Pixel = u16;

    const Y_MIN: u16 = 256;
    const Y_MAX: u16 = 3760;
    const UV_MIN: u16 = 256;
    const UV_MAX: u16 = 3840;
}

impl Range for Limited<Depth16> {
    type Pixel = u16;

    const Y_MIN: u16 = 4112;
    const Y_MAX: u16 = 60395;
    const UV_MIN: u16 = 4112;
    const UV_MAX: u16 = 61680;
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct RangeScale {
    /// First, multiply by this
    pub mul: f32,
    /// Then subtract this
    pub sub: f32,
}

#[inline(always)]
pub(crate) fn to_floats<F: Range>(multiply: f64) -> (RangeScale, RangeScale) where F::Pixel: Into<f64> {
    let y_min = F::Y_MIN.into();
    let y_max = F::Y_MAX.into();
    let y = RangeScale {
        mul: (multiply / (y_max - y_min)) as f32,
        sub: (multiply * y_min / (y_max - y_min)) as f32,
    };
    let uv_min = F::UV_MIN.into();
    let uv_max = F::UV_MAX.into();
    let uv = RangeScale {
        mul: (multiply / (uv_max - uv_min)) as f32,
        sub: (multiply * (if uv_min == 0. {
            (uv_max/2.).ceil() / uv_max
        } else {
            uv_min / (uv_max - uv_min) + 0.5
        })) as f32,
    };
    (y, uv)
}

pub(crate) trait ToFloat {
    fn to_float_y(self, fmin: Self, fmax: Self) -> f64;
    fn to_float_uv(self, fmin: Self, fmax: Self) -> f64;
}

impl ToFloat for u8 {
    #[inline(always)]
    fn to_float_y(self, fmin: u8, fmax: u8) -> f64 {
        f64::from(self.as_i16() - fmin.as_i16()) / f64::from(fmax - fmin)
    }
    #[inline(always)]
    fn to_float_uv(self, fmin: u8, fmax: u8) -> f64 {
        if fmin == 0 {
            f64::from(self.as_i16() - fmin.as_i16() - 128) / f64::from(fmax)
        } else {
            f64::from(self.as_i16() - fmin.as_i16()) / f64::from(fmax - fmin) - 0.5
        }
    }
}

impl ToFloat for u16 {
    #[inline(always)]
    fn to_float_y(self, fmin: u16, fmax: u16) -> f64 {
        f64::from(self.as_i32() - fmin.as_i32()) / f64::from(fmax - fmin)
    }
    #[inline(always)]
    fn to_float_uv(self, fmin: u16, fmax: u16) -> f64 {
        if fmin == 0 {
            f64::from(self.as_i32() - fmin.as_i32() - (fmax.as_i32() / 2 + 1)) / f64::from(fmax)
        } else {
            f64::from(self.as_i32() - fmin.as_i32()) / f64::from(fmax - fmin) - 0.5
        }
    }
}

#[test]
fn to_uv_zero() {
    assert_eq!(0., 128u8.to_float_uv(0,255));
    assert_eq!(0., 128u8.to_float_uv(16,240));
    assert_eq!(0., (1u16<<15).to_float_uv(0,65535));
    assert_eq!(0., 512u16.to_float_uv(64,960));
    assert_eq!(0., 512u16.to_float_uv(Limited::<Depth10>::UV_MIN,Limited::<Depth10>::UV_MAX));
    assert_eq!(0., 2048u16.to_float_uv(Limited::<Depth12>::UV_MIN,Limited::<Depth12>::UV_MAX));
    assert_eq!(0., (128*256+128).to_float_uv(Limited::<Depth16>::UV_MIN,Limited::<Depth16>::UV_MAX));
}
