//! Optional interoperability with the latest stable `rust_decimal` crate.

use core::fmt;

use crate::bounded::{DecimalPi, SupportedPrecision};

/// Maximum number of decimal places representable by `rust_decimal::Decimal`.
pub const RUST_DECIMAL_MAX_PLACES: usize = 28;

/// Failure while converting a bounded decimal π value to `rust_decimal`.
#[derive(Debug)]
pub enum RustDecimalInteropError {
    /// The source carries more decimal places than an exact Decimal can retain.
    TooManyDecimalPlaces {
        /// Source precision.
        requested: usize,
        /// Destination maximum scale.
        maximum: usize,
    },
    /// The finite coefficient could not be constructed safely.
    CoefficientOverflow,
    /// The destination crate rejected the checked coefficient/scale pair.
    Destination(rust_decimal::Error),
}

impl fmt::Display for RustDecimalInteropError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyDecimalPlaces { .. } => formatter
                .write_str("rust_decimal cannot exactly retain all requested decimal places"),
            Self::CoefficientOverflow => formatter.write_str("decimal coefficient overflow"),
            Self::Destination(error) => error.fmt(formatter),
        }
    }
}

impl core::error::Error for RustDecimalInteropError {}

impl<const D: usize> DecimalPi<D>
where
    (): SupportedPrecision<D>,
{
    /// Converts exactly to `rust_decimal::Decimal` when `D <= 28`.
    ///
    /// No binary floating-point conversion is involved.
    pub fn try_to_rust_decimal_exact(
        &self,
    ) -> Result<rust_decimal::Decimal, RustDecimalInteropError> {
        if D > RUST_DECIMAL_MAX_PLACES {
            return Err(RustDecimalInteropError::TooManyDecimalPlaces {
                requested: D,
                maximum: RUST_DECIMAL_MAX_PLACES,
            });
        }

        decimal_from_source(self, D, false)
    }

    /// Converts to `rust_decimal::Decimal`, rounding to at most 28 decimal
    /// places using round-to-nearest, ties-to-even when the source is wider.
    ///
    /// For `D <= 28` this is exact. For every supported Perfectπ value with
    /// `D > 28`, the first discarded source digit is greater than five, so
    /// the finite-domain nearest-even result always increments the 28-place
    /// coefficient by exactly one.
    pub fn to_rust_decimal_nearest_even(
        &self,
    ) -> Result<rust_decimal::Decimal, RustDecimalInteropError> {
        let scale = core::cmp::min(D, RUST_DECIMAL_MAX_PLACES);
        decimal_from_source(self, scale, D > RUST_DECIMAL_MAX_PLACES)
    }
}

fn decimal_from_source<const D: usize>(
    value: &DecimalPi<D>,
    scale: usize,
    round_up: bool,
) -> Result<rust_decimal::Decimal, RustDecimalInteropError> {
    let mut coefficient = u128::from(value.integer_part());
    let digits = value.fractional_digits();
    let mut index = 0;

    while index < scale {
        coefficient = coefficient
            .checked_mul(10)
            .ok_or(RustDecimalInteropError::CoefficientOverflow)?;
        let digit = match digits.get(index) {
            Some(digit) => u128::from(*digit),
            None => return Err(RustDecimalInteropError::CoefficientOverflow),
        };
        coefficient = coefficient
            .checked_add(digit)
            .ok_or(RustDecimalInteropError::CoefficientOverflow)?;
        index = index.saturating_add(1);
    }

    if round_up {
        coefficient = coefficient
            .checked_add(1)
            .ok_or(RustDecimalInteropError::CoefficientOverflow)?;
    }

    let signed =
        i128::try_from(coefficient).map_err(|_| RustDecimalInteropError::CoefficientOverflow)?;
    let scale_u32 =
        u32::try_from(scale).map_err(|_| RustDecimalInteropError::CoefficientOverflow)?;

    rust_decimal::Decimal::try_from_i128_with_scale(signed, scale_u32)
        .map_err(RustDecimalInteropError::Destination)
}
