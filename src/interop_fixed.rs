//! Optional interoperability with the latest stable `fixed` crate.

use core::fmt;

use crate::BufferTooSmall;
use crate::bounded::{DecimalPi, SupportedPrecision};
use fixed::traits::Fixed;

/// Failure while converting a bounded decimal π value to a `fixed` type.
#[derive(Debug)]
pub enum FixedInteropError {
    /// The internal caller-provided formatting buffer was unexpectedly too small.
    Buffer(BufferTooSmall),
    /// The destination fixed-point type could not parse/represent the value.
    Parse(fixed::ParseFixedError),
}

impl fmt::Display for FixedInteropError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Buffer(error) => error.fmt(formatter),
            Self::Parse(error) => error.fmt(formatter),
        }
    }
}

impl core::error::Error for FixedInteropError {}

impl<const D: usize> DecimalPi<D>
where
    (): SupportedPrecision<D>,
{
    /// Converts to any `fixed::traits::Fixed` destination using the
    /// destination crate's decimal parser.
    ///
    /// The `fixed` crate specifies round-to-nearest, ties-to-even semantics
    /// for decimal parsing. The method name makes that rounding contract
    /// explicit; it does not route through `f32` or `f64`.
    pub fn to_fixed_nearest_even<F>(&self) -> Result<F, FixedInteropError>
    where
        F: Fixed,
    {
        let mut buffer = [0_u8; 42];
        let written = self
            .write_ascii(&mut buffer)
            .map_err(FixedInteropError::Buffer)?;

        F::from_ascii(&buffer[..written]).map_err(FixedInteropError::Parse)
    }
}
