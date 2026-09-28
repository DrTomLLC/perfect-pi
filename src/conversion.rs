//! Explicit bounded-decimal to native-float conversions.

use core::fmt;

use crate::bounded::{DecimalPi, Pi, SupportedPrecision};
use crate::native::{PI_F32, PI_F64};

/// Maximum fractional decimal places Perfectπ guarantees an `f32` conversion
/// can preserve for values in the bounded π domain.
pub const F32_GUARANTEED_DECIMAL_PLACES: usize = 6;

/// Maximum fractional decimal places Perfectπ guarantees an `f64` conversion
/// can preserve for values in the bounded π domain.
pub const F64_GUARANTEED_DECIMAL_PLACES: usize = 15;

// Exact IEEE-754 nearest-even encodings independently generated from the
// three finite decimal value classes produced by the bounded rounding API.
// Const-generic matches keep each monomorphization scalar and dead-strip
// unrelated conversion vectors on constrained targets.

#[inline(always)]
fn f32_truncated_bits<const D: usize>() -> Option<u32> {
    match D {
        0 => Some(0x4040_0000),
        1 => Some(0x4046_6666),
        2 => Some(0x4048_f5c3),
        3 => Some(0x4049_0625),
        4 => Some(0x4049_0e56),
        5 => Some(0x4049_0fd0),
        6 => Some(0x4049_0fd8),
        7 => Some(0x4049_0fda),
        _ => None,
    }
}

#[inline(always)]
fn f32_nearest_bits<const D: usize>() -> Option<u32> {
    match D {
        0 => Some(0x4040_0000),
        1 => Some(0x4046_6666),
        2 => Some(0x4048_f5c3),
        3 => Some(0x4049_1687),
        4 => Some(0x4049_0ff9),
        5 => Some(0x4049_0fd0),
        6 => Some(0x4049_0fdc),
        7 => Some(0x4049_0fdb),
        _ => None,
    }
}

#[inline(always)]
fn f32_ceiling_bits<const D: usize>() -> Option<u32> {
    match D {
        0 => Some(0x4080_0000),
        1 => Some(0x404c_cccd),
        2 => Some(0x4049_999a),
        3 => Some(0x4049_1687),
        4 => Some(0x4049_0ff9),
        5 => Some(0x4049_0ff9),
        6 => Some(0x4049_0fdc),
        _ => None,
    }
}

#[inline(always)]
fn f64_truncated_bits<const D: usize>() -> Option<u64> {
    match D {
        0 => Some(0x4008_0000_0000_0000),
        1 => Some(0x4008_cccc_cccc_cccd),
        2 => Some(0x4009_1eb8_51eb_851f),
        3 => Some(0x4009_20c4_9ba5_e354),
        4 => Some(0x4009_21ca_c083_126f),
        5 => Some(0x4009_21f9_f01b_866e),
        6 => Some(0x4009_21fa_fc8b_007a),
        7 => Some(0x4009_21fb_4d12_d84a),
        8 => Some(0x4009_21fb_53c8_d4f1),
        9 => Some(0x4009_21fb_542f_e938),
        10 => Some(0x4009_21fb_5441_1744),
        11 => Some(0x4009_21fb_5443_d6f4),
        12 => Some(0x4009_21fb_5444_261e),
        13 => Some(0x4009_21fb_5444_2c46),
        14 => Some(0x4009_21fb_5444_2d11),
        _ => None,
    }
}

#[inline(always)]
fn f64_nearest_bits<const D: usize>() -> Option<u64> {
    match D {
        0 => Some(0x4008_0000_0000_0000),
        1 => Some(0x4008_cccc_cccc_cccd),
        2 => Some(0x4009_1eb8_51eb_851f),
        3 => Some(0x4009_22d0_e560_4189),
        4 => Some(0x4009_21ff_2e48_e8a7),
        5 => Some(0x4009_21f9_f01b_866e),
        6 => Some(0x4009_21fb_82c2_bd7f),
        7 => Some(0x4009_21fb_5a7e_d197),
        8 => Some(0x4009_21fb_53c8_d4f1),
        9 => Some(0x4009_21fb_5452_4550),
        10 => Some(0x4009_21fb_5444_86e0),
        11 => Some(0x4009_21fb_5444_2eea),
        12 => Some(0x4009_21fb_5444_2eea),
        13 => Some(0x4009_21fb_5444_2d28),
        14 => Some(0x4009_21fb_5444_2d11),
        _ => None,
    }
}

#[inline(always)]
fn f64_ceiling_bits<const D: usize>() -> Option<u64> {
    match D {
        0 => Some(0x4010_0000_0000_0000),
        1 => Some(0x4009_9999_9999_999a),
        2 => Some(0x4009_3333_3333_3333),
        3 => Some(0x4009_22d0_e560_4189),
        4 => Some(0x4009_21ff_2e48_e8a7),
        5 => Some(0x4009_21ff_2e48_e8a7),
        6 => Some(0x4009_21fb_82c2_bd7f),
        7 => Some(0x4009_21fb_5a7e_d197),
        8 => Some(0x4009_21fb_5520_6ddf),
        9 => Some(0x4009_21fb_5452_4550),
        10 => Some(0x4009_21fb_5444_86e0),
        11 => Some(0x4009_21fb_5444_2eea),
        12 => Some(0x4009_21fb_5444_2eea),
        13 => Some(0x4009_21fb_5444_2d28),
        14 => Some(0x4009_21fb_5444_2d28),
        15 => Some(0x4009_21fb_5444_2d1a),
        _ => None,
    }
}

/// Error returned when a checked native-float conversion cannot guarantee
/// preservation of every requested decimal place.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrecisionLoss {
    requested_decimal_places: usize,
    guaranteed_decimal_places: usize,
}

impl PrecisionLoss {
    const fn new(requested_decimal_places: usize, guaranteed_decimal_places: usize) -> Self {
        Self {
            requested_decimal_places,
            guaranteed_decimal_places,
        }
    }

    /// Returns the number of decimal places carried by the source value.
    #[must_use]
    pub const fn requested_decimal_places(&self) -> usize {
        self.requested_decimal_places
    }

    /// Returns the maximum number of decimal places guaranteed by the target.
    #[must_use]
    pub const fn guaranteed_decimal_places(&self) -> usize {
        self.guaranteed_decimal_places
    }
}

impl fmt::Display for PrecisionLoss {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("target float cannot guarantee all requested decimal places")
    }
}

impl core::error::Error for PrecisionLoss {}

impl<const D: usize> DecimalPi<D>
where
    (): SupportedPrecision<D>,
{
    /// Converts the stored canonical decimal π value to the nearest IEEE-754
    /// binary32 value, explicitly allowing decimal-place loss.
    ///
    /// The returned bit pattern is an independently verified round-to-nearest,
    /// ties-to-even conversion of this finite decimal value. This converts the
    /// stored value, not mathematical π.
    #[must_use]
    pub fn to_f32_lossy(&self) -> f32 {
        let bits = if self.is_truncated() {
            f32_truncated_bits::<D>()
        } else if self.is_nearest_even() {
            f32_nearest_bits::<D>()
        } else {
            f32_ceiling_bits::<D>()
        };

        match bits {
            Some(bits) => f32::from_bits(bits),
            None => PI_F32,
        }
    }

    /// Converts the stored canonical decimal π value to the nearest IEEE-754
    /// binary64 value, explicitly allowing decimal-place loss.
    ///
    /// The returned bit pattern is an independently verified round-to-nearest,
    /// ties-to-even conversion of this finite decimal value. This converts the
    /// stored value, not mathematical π.
    #[must_use]
    pub fn to_f64_lossy(&self) -> f64 {
        let bits = if self.is_truncated() {
            f64_truncated_bits::<D>()
        } else if self.is_nearest_even() {
            f64_nearest_bits::<D>()
        } else {
            f64_ceiling_bits::<D>()
        };

        match bits {
            Some(bits) => f64::from_bits(bits),
            None => PI_F64,
        }
    }

    /// Converts to `f32` only when every requested decimal place is
    /// guaranteed to survive the binary conversion.
    ///
    /// Six fractional places is the maximum conservative guarantee across the
    /// canonical bounded π domain. `D=7` already contains a value that cannot
    /// preserve all seven places in binary32.
    pub fn try_to_f32_preserving_places(&self) -> Result<f32, PrecisionLoss> {
        if D <= F32_GUARANTEED_DECIMAL_PLACES {
            Ok(self.to_f32_lossy())
        } else {
            Err(PrecisionLoss::new(D, F32_GUARANTEED_DECIMAL_PLACES))
        }
    }

    /// Converts to `f64` only when every requested decimal place is
    /// guaranteed to survive the binary conversion.
    ///
    /// Fifteen fractional places is the maximum conservative guarantee across
    /// the canonical bounded π domain. `D=16` already contains a value that
    /// cannot preserve all sixteen places in binary64.
    pub fn try_to_f64_preserving_places(&self) -> Result<f64, PrecisionLoss> {
        if D <= F64_GUARANTEED_DECIMAL_PLACES {
            Ok(self.to_f64_lossy())
        } else {
            Err(PrecisionLoss::new(D, F64_GUARANTEED_DECIMAL_PLACES))
        }
    }

    fn is_truncated(&self) -> bool {
        *self == Pi::<D>::truncated()
    }

    fn is_nearest_even(&self) -> bool {
        *self == Pi::<D>::round_nearest_even()
    }
}
