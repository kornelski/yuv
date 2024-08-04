use num_traits::PrimInt;

pub trait Bounded {
    const MIN: Self;
}

impl Bounded for u8 {
    const MIN: u8 = 0;
}

impl Bounded for u16 {
    const MIN: u16 = 0;
}

pub trait Depth: 'static {
    type Pixel: PrimInt + Bounded;
    const MAX: Self::Pixel;
}

pub struct Depth8;
pub struct Depth10;
pub struct Depth12;
pub struct Depth16;

impl Depth for Depth8 {
    type Pixel = u8;
    const MAX: u8 = 255;
}

impl Depth for Depth10 {
    type Pixel = u16;
    const MAX: u16 = (1 << 10) - 1;
}

impl Depth for Depth12 {
    type Pixel = u16;
    const MAX: u16 = (1 << 12) - 1;
}

impl Depth for Depth16 {
    type Pixel = u16;
    const MAX: u16 = 65535;
}
