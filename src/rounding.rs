//! Rounding policies shared by bounded and runtime π generation.

/// Decimal rounding policy used when reducing mathematical π to a finite
/// number of places after the decimal point.
///
/// π is positive and irrational. Therefore it is never exactly representable
/// by a finite decimal value and can never land on an exact halfway tie.
/// NearestTiesToEven and NearestTiesAwayFromZero consequently produce the
/// same value for π while retaining their conventional semantics.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RoundingMode {
    /// Discard all digits beyond the requested precision.
    TowardZero,
    /// Increase the retained magnitude whenever discarded digits are nonzero.
    AwayFromZero,
    /// Round toward negative infinity.
    TowardNegativeInfinity,
    /// Round toward positive infinity.
    TowardPositiveInfinity,
    /// Round to the nearest value, with exact halfway ties going to even.
    NearestTiesToEven,
    /// Round to the nearest value, with exact halfway ties away from zero.
    NearestTiesAwayFromZero,
}
