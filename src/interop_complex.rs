//! Optional interoperability with the latest stable `num-complex` crate.

use crate::PrecisionLoss;
use crate::bounded::{DecimalPi, SupportedPrecision};
use num_complex::{Complex32, Complex64};

impl<const D: usize> DecimalPi<D>
where
    (): SupportedPrecision<D>,
{
    /// Converts to a complex binary32 value with zero imaginary part,
    /// explicitly allowing the same precision loss as [`Self::to_f32_lossy`].
    #[must_use]
    pub fn to_complex32_lossy(&self) -> Complex32 {
        Complex32::new(self.to_f32_lossy(), 0.0)
    }

    /// Converts to a complex binary64 value with zero imaginary part,
    /// explicitly allowing the same precision loss as [`Self::to_f64_lossy`].
    #[must_use]
    pub fn to_complex64_lossy(&self) -> Complex64 {
        Complex64::new(self.to_f64_lossy(), 0.0)
    }

    /// Converts to complex binary32 only when all requested decimal places are
    /// guaranteed to survive in the real component.
    pub fn try_to_complex32_preserving_places(&self) -> Result<Complex32, PrecisionLoss> {
        self.try_to_f32_preserving_places()
            .map(|value| Complex32::new(value, 0.0))
    }

    /// Converts to complex binary64 only when all requested decimal places are
    /// guaranteed to survive in the real component.
    pub fn try_to_complex64_preserving_places(&self) -> Result<Complex64, PrecisionLoss> {
        self.try_to_f64_preserving_places()
            .map(|value| Complex64::new(value, 0.0))
    }
}
