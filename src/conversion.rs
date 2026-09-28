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
// canonical finite decimal values. At D>=8 every canonical bounded value maps
// to PI_F32; at D>=15 every canonical bounded value maps to PI_F64.
const F32_TRUNCATED_BITS: [u32; 8] = [
    0x4040_0000,
    0x4046_6666,
    0x4048_f5c3,
    0x4049_0625,
    0x4049_0e56,
    0x4049_0fd0,
    0x4049_0fd8,
    0x4049_0fda,
];

const F32_ROUNDED_BITS: [u32; 8] = [
    0x4040_0000,
    0x4046_6666,
    0x4048_f5c3,
    0x4049_1687,
    0x4049_0ff9,
    0x4049_0fd0,
    0x4049_0fdc,
    0x4049_0fdb,
];

const F64_TRUNCATED_BITS: [u64; 15] = [
    0x4008_0000_0000_0000,
    0x4008_cccc_cccc_cccd,
    0x4009_1eb8_51eb_851f,
    0x4009_20c4_9ba5_e354,
    0x4009_21ca_c083_126f,
    0x4009_21f9_f01b_866e,
    0x4009_21fa_fc8b_007a,
    0x4009_21fb_4d12_d84a,
    0x4009_21fb_53c8_d4f1,
    0x4009_21fb_542f_e938,
    0x4009_21fb_5441_1744,
    0x4009_21fb_5443_d6f4,
    0x4009_21fb_5444_261e,
    0x4009_21fb_5444_2c46,
    0x4009_21fb_5444_2d11,
];

const F64_ROUNDED_BITS: [u64; 15] = [
    0x4008_0000_0000_0000,
    0x4008_cccc_cccc_cccd,
    0x4009_1eb8_51eb_851f,
    0x4009_22d0_e560_4189,
    0x4009_21ff_2e48_e8a7,
    0x4009_21f9_f01b_866e,
    0x4009_21fb_82c2_bd7f,
    0x4009_21fb_5a7e_d197,
    0x4009_21fb_53c8_d4f1,
    0x4009_21fb_5452_4550,
    0x4009_21fb_5444_86e0,
    0x4009_21fb_5444_2eea,
    0x4009_21fb_5444_2eea,
    0x4009_21fb_5444_2d28,
    0x4009_21fb_5444_2d11,
];

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
        if D >= F32_TRUNCATED_BITS.len() {
            return PI_F32;
        }

        let table = if self.is_truncated() {
            &F32_TRUNCATED_BITS
        } else {
            &F32_ROUNDED_BITS
        };

        match table.get(D) {
            Some(bits) => f32::from_bits(*bits),
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
        if D >= F64_TRUNCATED_BITS.len() {
            return PI_F64;
        }

        let table = if self.is_truncated() {
            &F64_TRUNCATED_BITS
        } else {
            &F64_ROUNDED_BITS
        };

        match table.get(D) {
            Some(bits) => f64::from_bits(*bits),
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
}
