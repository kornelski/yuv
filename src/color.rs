/// Chroma subsampling format
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum ChromaSampling {
    /// 4:2:0 = 2x2 pixels of luma per 1 pixel of chroma
    Cs420,
    /// 4:2:2 = Horizontally subsampled
    Cs422,
    /// 4:2:4 = Not subsampled (chroma and luma are 1:1)
    Cs444,
    /// 4:0:0 Only luma (grayscale)
    Monochrome,
}

/// Range of allowed values for pixels
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Range {
    /// Luma is 16-235, Chroma is 16-240
    Limited,
    /// 0-255
    Full,
}

/// Supported Color Primaries
///
/// As defined by “Color primaries” section of ISO/IEC 23091-4/ITU-T H.273
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorPrimaries {
    /// BT.709, sRGB, sYCC. BT.709 is the standard for high definition (HD) television; sRGB is the most common color space used for computer displays.
    BT709 = 1,
    /// BT.470 System M, NTSC (standard definition television in the United States) (historical)
    BT470M = 4,
    /// BT.470 System B, G; BT.601; BT.1358 625; BT.1700 625 PAL and 625 SECAM (historical)
    BT470BG,
    /// BT.601-7 525 (SMPTE 170 M) and SMPTE 240M (historical)
    BT601,
    /// ITU-T H.264
    GenericFilm = 8,
    /// BT.2020; BT.2100. Used for ultra-high definition (4K) High Dynamic Range (HDR) video, these have a very wide color gamut and support 10-bit and 12-bit color component depths.
    BT2020,
    /// CIE 1921 XYZ; SMPTE ST 428 (D-Cinema Distribution Master: Image characteristics). Defines the uncompressed image characteristics for DCDM.
    XYZ,
    /// SMPTE RP 431 (D-Cinema Quality: Reference projector and environment).
    SMPTE431,
    /// SMPTE EG 432-1 (Digital Source Processing: Color Processing for D-Cinema).
    SMPTE432,
    /// EBU Tech. 3213-E
    EBU3213 = 22,
}

/// Gamma correction, essentially.
/// As defined by “Transfer characteristics” section of ISO/IEC 23091-4/ITU-TH.273.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TransferCharacteristics {
    /// BT.709
    BT709 = 1,
    /// BT.470 System M (historical)
    BT470M = 4,
    /// BT.470 System B, G (historical)
    BT470BG,
    /// BT.601-7 525 (SMPTE 170 M)
    BT601,
    /// SMPTE 240 M
    SMPTE240,
    /// Linear
    Linear,
    /// Logarithmic (100:1 range)
    Log100,
    /// Logarithmic ((100 * √10):1 range)
    Log100Sqrt10,
    /// IEC 61966-2-4
    IEC61966,
    /// BT.1361 extended color gamut system (historical)
    BT1361,
    /// sRGB or sYCC
    SRGB,
    /// BT.2020 10-bit systems
    BT2020_10Bit,
    /// BT.2020 12-bit systems
    BT2020_12Bit,
    /// SMPTE ST 2084, ITU BT.2100 PQ
    SMPTE2084,
    /// SMPTE ST 428
    SMPTE428,
    /// BT.2100 HLG (Hybrid Log Gamma), ARIB STD-B67
    HLG,
}

/// Bit depth (8 = 1 byte, >=10 = 2 bytes)
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Depth {
    Depth8 = 8,
    Depth10 = 10,
    Depth12 = 12,
    Depth16 = 16,
}

/// As defined by the “Matrix coefficients” section of ISO/IEC 23091-4/ITU-TH.273.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MatrixCoefficients {
    /// Identity matrix
    Identity = 0,
    /// BT.709 ITU-R Rec. BT.709-5, ITU-R Rec. BT.1361 conventional colour gamut system and extended colour gamut system,
    /// IEC 61966-2-4 xvYCC709,
    /// Society of Motion Picture and Television Engineers RP 177 (1993)
    /// KR = 0.2126; KB = 0.0722
    BT709,
    /// United States Federal Communications Commission Title 47 Code of Federal Regulations (2003) 73.682 (a) (20)
    /// KR = 0.30; KB = 0.11
    FCC = 4,
    /// ITU-RRec.BT.470-6SystemB,G,IEC61966-2-4xvYCC601 (historical)
    /// KR=0.299;KB=0.114
    BT470BG,
    /// BT.601-7 525 (SMPTE 170 M)
    /// KR=0.299;KB=0.114
    BT601,
    /// SMPTE 240 M
    /// KR=0.212;KB=0.087
    SMPTE240,
    /// YCgCo
    YCgCo,
    /// BT.2020 non-constant luminance, BT.2100 YCbCr
    BT2020NCL,
    /// BT.2020 constant luminance
    BT2020CL,
    /// SMPTE ST 2085 YDzDx
    SMPTE2085,
    /// Chromaticity-derived non-constant luminance
    ChromatNCL,
    /// Chromaticity-derived constant luminance
    ChromatCL,
    /// BT.2020 ICtCp
    ICtCp,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChromaSamplePosition {
    /// Horizontally co-located with (0, 0) luma sample, vertically positioned
    /// in the middle between two luma samples.
    Vertical,
    /// Co-located with (0, 0) luma sample.
    Colocated,
}
