#[cfg(debug_assertions)]
use std::convert::TryInto;

pub trait CheckedAs {
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_i8(self) -> i8;
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_u8(self) -> u8;
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_u16(self) -> u16;
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_i16(self) -> i16;
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_i32(self) -> i32;
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_u32(self) -> u32;
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_i64(self) -> i64;
}

impl CheckedAs for i8 {
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        assert!(self >= 0);
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self as _
    }
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self.into()
    }
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self.into()
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self as _
    }
    #[inline(always)]
    fn as_i64(self) -> i64 {
        self.into()
    }
}

impl CheckedAs for u8 {
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self as _
    }
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self
    }
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self.into()
    }
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self.into()
    }
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self.into()
    }
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self.into()
    }
    #[inline(always)]
    fn as_i64(self) -> i64 {
        self.into()
    }
}

impl CheckedAs for u16 {
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        assert!(self <= 255, "{}", self);
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self as _
    }
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self as _
    }
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self.into()
    }
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self.into()
    }
    #[inline(always)]
    fn as_i64(self) -> i64 {
        self.into()
    }
}

impl CheckedAs for i16 {
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        assert!(self >= 0, "{}", self);
        assert!(self <= 255, "{}", self);
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        assert!(self >= 0);
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self as _
    }
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self
    }
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self.into()
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self as _
    }
    #[inline(always)]
    fn as_i64(self) -> i64 {
        self.into()
    }
}

impl CheckedAs for u32 {
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        assert!(self <= 65535, "{}", self);
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self as _
    }
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self
    }
    #[inline(always)]
    fn as_i64(self) -> i64 {
        self.into()
    }
}

impl CheckedAs for i32 {
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        assert!(self >= 0);
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        assert!(self >= 0);
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self as _
    }
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self as _
    }
    #[inline(always)]
    fn as_i64(self) -> i64 {
        self.into()
    }
}

impl CheckedAs for i64 {
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i8(self) -> i8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u8(self) -> u8 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u16(self) -> u16 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i16(self) -> i16 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_i32(self) -> i32 {
        self as _
    }
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn as_u32(self) -> u32 {
        assert!(self >= 0);
        self.try_into().unwrap()
    }
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self as _
    }
    #[inline(always)]
    fn as_i64(self) -> i64 {
        self
    }
}
