use core::fmt;

#[cfg(not(feature = "no_std"))]
use std::error;
#[cfg(feature = "no_std")]
use core::error;

/// This library doesn't support all combinations of color spaces
#[derive(Debug, Copy, Clone)]
pub enum Error {
    UnsupportedTransferCharacteristics,
    UnsupportedMatrixCoefficients,
    InvalidDepthRequested,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedTransferCharacteristics => "Unsupported color space (transfer characteristics)",
            Self::UnsupportedMatrixCoefficients => "Unsupported color space (matrix coefficients)",
            Self::InvalidDepthRequested => "16-bit converter was asked to convert 8-bit color",
        })
    }
}
