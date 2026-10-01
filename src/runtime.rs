//! Optional runtime generation of π beyond the bounded constant tier.

use core::cmp::Ordering;
use core::fmt;
use num_bigint::BigInt;

use crate::RoundingMode;

const MAX_GUARD_PLACES: u32 = 64;
const CHUDNOVSKY_A: u64 = 13_591_409;
const CHUDNOVSKY_B: u64 = 545_140_134;
const CHUDNOVSKY_C3_OVER_24: u64 = 10_939_058_860_032_000;
const CHUDNOVSKY_C_MULTIPLIER: u32 = 426_880;
const CHUDNOVSKY_SQRT_RADICAND: u32 = 10_005;
const CHUDNOVSKY_TERM_DIGITS: usize = 14;
const CHUDNOVSKY_TERM_MARGIN: usize = 8;

/// Error returned by runtime π generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimePiError {
    /// The caller-provided output buffer cannot hold the requested decimal text.
    BufferTooSmall {
        /// Number of output bytes required.
        required: usize,
        /// Number of output bytes supplied by the caller.
        provided: usize,
    },
    /// The requested precision cannot be represented by this API.
    PrecisionTooLarge,
    /// A caller-selected resource limit rejected the requested precision.
    PrecisionLimitExceeded {
        /// Requested places after the decimal point.
        requested: usize,
        /// Maximum places permitted by the caller.
        limit: usize,
    },
    /// An internal arithmetic or formatting invariant was not satisfied.
    InternalInvariant,
}

/// Returns the ASCII byte count required for runtime-generated π.
pub fn runtime_pi_ascii_len(decimal_places: usize) -> Result<usize, RuntimePiError> {
    if decimal_places == 0 {
        Ok(1)
    } else {
        decimal_places
            .checked_add(2)
            .ok_or(RuntimePiError::PrecisionTooLarge)
    }
}

/// Generates π truncated to exactly decimal_places places after the decimal point.
///
/// Equivalent to generate_pi_ascii_with_rounding with TowardZero.
pub fn generate_pi_ascii(
    decimal_places: usize,
    output: &mut [u8],
) -> Result<usize, RuntimePiError> {
    generate_pi_ascii_with_rounding(decimal_places, RoundingMode::TowardZero, output)
}

/// Generates π rounded to decimal_places places using nearest, ties-to-even.
///
/// Equivalent to generate_pi_ascii_with_rounding with NearestTiesToEven.
pub fn generate_pi_ascii_round_nearest_even(
    decimal_places: usize,
    output: &mut [u8],
) -> Result<usize, RuntimePiError> {
    generate_pi_ascii_with_rounding(decimal_places, RoundingMode::NearestTiesToEven, output)
}

/// Generates π using an explicit decimal rounding policy.
///
/// This optional variable-cost path uses arbitrary-precision integer arithmetic
/// and never routes through binary floating point. Callers forwarding untrusted
/// precision should prefer generate_pi_ascii_with_limit.
pub fn generate_pi_ascii_with_rounding(
    decimal_places: usize,
    rounding: RoundingMode,
    output: &mut [u8],
) -> Result<usize, RuntimePiError> {
    let required = runtime_pi_ascii_len(decimal_places)?;
    if output.len() < required {
        return Err(RuntimePiError::BufferTooSmall {
            required,
            provided: output.len(),
        });
    }

    let scaled = scaled_pi_for_rounding(decimal_places, rounding)?;
    write_scaled_pi(decimal_places, &scaled, output, required)
}

/// Generates π while enforcing a caller-selected maximum precision.
///
/// The limit check happens before generation or output mutation.
pub fn generate_pi_ascii_with_limit(
    decimal_places: usize,
    max_decimal_places: usize,
    rounding: RoundingMode,
    output: &mut [u8],
) -> Result<usize, RuntimePiError> {
    enforce_precision_limit(decimal_places, max_decimal_places)?;
    generate_pi_ascii_with_rounding(decimal_places, rounding, output)
}

impl fmt::Display for RuntimePiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferTooSmall { .. } => formatter.write_str("output buffer is too small"),
            Self::PrecisionTooLarge => formatter.write_str("requested precision is too large"),
            Self::PrecisionLimitExceeded { .. } => {
                formatter.write_str("requested precision exceeds the configured limit")
            }
            Self::InternalInvariant => {
                formatter.write_str("runtime pi generation invariant failed")
            }
        }
    }
}

impl core::error::Error for RuntimePiError {}

fn enforce_precision_limit(
    decimal_places: usize,
    max_decimal_places: usize,
) -> Result<(), RuntimePiError> {
    if decimal_places > max_decimal_places {
        Err(RuntimePiError::PrecisionLimitExceeded {
            requested: decimal_places,
            limit: max_decimal_places,
        })
    } else {
        Ok(())
    }
}

fn scaled_pi_for_rounding(
    decimal_places: usize,
    rounding: RoundingMode,
) -> Result<BigInt, RuntimePiError> {
    match rounding {
        RoundingMode::TowardZero | RoundingMode::TowardNegativeInfinity => {
            certified_scaled_pi(decimal_places)
        }
        RoundingMode::AwayFromZero | RoundingMode::TowardPositiveInfinity => {
            let mut value = certified_scaled_pi(decimal_places)?;
            value += 1_u8;
            Ok(value)
        }
        RoundingMode::NearestTiesToEven | RoundingMode::NearestTiesAwayFromZero => {
            let extended_places = decimal_places
                .checked_add(1)
                .ok_or(RuntimePiError::PrecisionTooLarge)?;
            let extended = certified_scaled_pi(extended_places)?;
            let next_digit = &extended % 10_u8;
            let mut rounded = &extended / 10_u8;
            if next_digit >= BigInt::from(5_u8) {
                rounded += 1_u8;
            }
            Ok(rounded)
        }
    }
}

/// Returns floor(π * 10^decimal_places) after proving that the exact value lies
/// inside one integer bucket.
///
/// Production generation uses the Chudnovsky series with binary splitting.
/// Consecutive partial sums of its alternating, monotonically decreasing series
/// bracket the exact reciprocal-π series. Integer square-root bounds bracket
/// sqrt(10005). Combining those rational intervals produces lower and upper
/// bounds for π itself; generation succeeds only when both bounds have the same
/// requested-scale floor.
fn certified_scaled_pi(decimal_places: usize) -> Result<BigInt, RuntimePiError> {
    let mut guard_places = 16_u32;

    loop {
        let guard_usize =
            usize::try_from(guard_places).map_err(|_| RuntimePiError::PrecisionTooLarge)?;
        let working_places = decimal_places
            .checked_add(guard_usize)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let exponent =
            u32::try_from(working_places).map_err(|_| RuntimePiError::PrecisionTooLarge)?;

        let terms = working_places
            .checked_div(CHUDNOVSKY_TERM_DIGITS)
            .and_then(|value| value.checked_add(CHUDNOVSKY_TERM_MARGIN))
            .ok_or(RuntimePiError::PrecisionTooLarge)?;

        let split = chudnovsky_binary_split(0, terms)?;
        let next_term_end = terms
            .checked_add(1)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let next_leaf = chudnovsky_binary_split(terms, next_term_end)?;

        let next_q = &split.q * &next_leaf.q;
        let next_t = &split.t * &next_leaf.q + &split.p * &next_leaf.t;

        let comparison = (&split.t * &next_q).cmp(&(&next_t * &split.q));
        let ((series_lower_t, series_lower_q), (series_upper_t, series_upper_q)) =
            match comparison {
                Ordering::Less | Ordering::Equal => {
                    ((&split.t, &split.q), (&next_t, &next_q))
                }
                Ordering::Greater => ((&next_t, &next_q), (&split.t, &split.q)),
            };

        if series_lower_t <= &BigInt::default() || series_upper_t <= &BigInt::default() {
            return Err(RuntimePiError::InternalInvariant);
        }

        let decimal_scale = BigInt::from(10_u8).pow(exponent);
        let sqrt_floor = scaled_sqrt_10005_floor(&decimal_scale);
        let c_lower = &sqrt_floor * CHUDNOVSKY_C_MULTIPLIER;
        let c_upper = (&sqrt_floor + 1_u8) * CHUDNOVSKY_C_MULTIPLIER;
        let guard_scale = BigInt::from(10_u8).pow(guard_places);

        let lower_numerator = c_lower * series_upper_q;
        let lower_denominator = &guard_scale * series_upper_t;
        let upper_numerator = c_upper * series_lower_q;
        let upper_denominator = &guard_scale * series_lower_t;

        let lower_floor = lower_numerator / lower_denominator;
        let upper_floor = upper_numerator / upper_denominator;

        if lower_floor == upper_floor {
            return Ok(lower_floor);
        }

        guard_places = next_guard_places(guard_places)?;
    }
}

struct BinarySplit {
    p: BigInt,
    q: BigInt,
    t: BigInt,
}

fn chudnovsky_binary_split(start: usize, end: usize) -> Result<BinarySplit, RuntimePiError> {
    if start >= end {
        return Err(RuntimePiError::InternalInvariant);
    }

    if end - start == 1 {
        return Ok(chudnovsky_leaf(start));
    }

    let midpoint = start
        .checked_add((end - start) / 2)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    let left = chudnovsky_binary_split(start, midpoint)?;
    let right = chudnovsky_binary_split(midpoint, end)?;

    let p = &left.p * &right.p;
    let q = &left.q * &right.q;
    let t = &left.t * &right.q + &left.p * &right.t;

    Ok(BinarySplit { p, q, t })
}

fn chudnovsky_leaf(index: usize) -> BinarySplit {
    if index == 0 {
        return BinarySplit {
            p: BigInt::from(1_u8),
            q: BigInt::from(1_u8),
            t: BigInt::from(CHUDNOVSKY_A),
        };
    }

    let index_big = BigInt::from(index);
    let six_index = &index_big * 6_u8;
    let p = (&six_index - 5_u8)
        * (&index_big * 2_u8 - 1_u8)
        * (&six_index - 1_u8);
    let q = &index_big
        * &index_big
        * &index_big
        * BigInt::from(CHUDNOVSKY_C3_OVER_24);
    let linear = BigInt::from(CHUDNOVSKY_A) + &index_big * CHUDNOVSKY_B;
    let mut t = &p * linear;

    if !index.is_multiple_of(2) {
        t = -t;
    }

    BinarySplit { p, q, t }
}

fn scaled_sqrt_10005_floor(scale: &BigInt) -> BigInt {
    let radicand = BigInt::from(CHUDNOVSKY_SQRT_RADICAND) * scale * scale;
    let mut estimate = scale * 101_u8;

    loop {
        let quotient = &radicand / &estimate;
        let next = (&estimate + quotient) / 2_u8;
        if next >= estimate {
            return estimate;
        }
        estimate = next;
    }
}

fn next_guard_places(current: u32) -> Result<u32, RuntimePiError> {
    let next = current
        .checked_add(8)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    if next > MAX_GUARD_PLACES {
        Err(RuntimePiError::InternalInvariant)
    } else {
        Ok(next)
    }
}

#[cfg(test)]
fn certified_scaled_pi_machin(decimal_places: usize) -> Result<BigInt, RuntimePiError> {
    let mut guard_places = 8_u32;
    loop {
        let guard_usize =
            usize::try_from(guard_places).map_err(|_| RuntimePiError::PrecisionTooLarge)?;
        let working_places = decimal_places
            .checked_add(guard_usize)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let exponent =
            u32::try_from(working_places).map_err(|_| RuntimePiError::PrecisionTooLarge)?;

        let max_terms = working_places
            .checked_add(8)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let scale = BigInt::from(10_u8).pow(exponent);
        let (lower, upper) = machin_pi_bounds(&scale, max_terms)?;
        let guard_scale = BigInt::from(10_u8).pow(guard_places);
        let lower_requested = &lower / &guard_scale;
        let upper_requested = &upper / &guard_scale;

        if lower_requested == upper_requested {
            return Ok(lower_requested);
        }

        guard_places = next_guard_places(guard_places)?;
    }
}

#[cfg(test)]
fn machin_pi_bounds(
    scale: &BigInt,
    max_terms: usize,
) -> Result<(BigInt, BigInt), RuntimePiError> {
    let (atan5_lower, atan5_upper) = arctan_reciprocal_bounds(scale, 5, max_terms)?;
    let (atan239_lower, atan239_upper) = arctan_reciprocal_bounds(scale, 239, max_terms)?;

    let lower = atan5_lower * 16_u8 - atan239_upper * 4_u8;
    let upper = atan5_upper * 16_u8 - atan239_lower * 4_u8;
    Ok((lower, upper))
}

#[cfg(test)]
fn arctan_reciprocal_bounds(
    scale: &BigInt,
    inverse: u32,
    max_terms: usize,
) -> Result<(BigInt, BigInt), RuntimePiError> {
    let inverse_big = BigInt::from(inverse);
    let inverse_squared = inverse
        .checked_mul(inverse)
        .map(BigInt::from)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let one = BigInt::from(1_u8);
    let zero = BigInt::default();
    let mut power = inverse_big;
    let mut lower = BigInt::default();
    let mut upper = BigInt::default();
    let mut term_index = 0_usize;

    loop {
        if term_index >= max_terms {
            return Err(RuntimePiError::InternalInvariant);
        }

        let odd = term_index
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let denominator = &power * BigInt::from(odd);
        let floor_term = scale / denominator;

        if floor_term == zero {
            if term_index.is_multiple_of(2) {
                upper += &one;
            } else {
                lower -= &one;
            }
            return Ok((lower, upper));
        }

        if term_index.is_multiple_of(2) {
            lower += &floor_term;
            upper += &floor_term;
            upper += &one;
        } else {
            lower -= &floor_term;
            lower -= &one;
            upper -= &floor_term;
        }

        power *= &inverse_squared;
        term_index = term_index
            .checked_add(1)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
    }
}

fn write_scaled_pi(
    decimal_places: usize,
    scaled: &BigInt,
    output: &mut [u8],
    required: usize,
) -> Result<usize, RuntimePiError> {
    let digits = scaled.to_str_radix(10);
    let expected_digits = decimal_places
        .checked_add(1)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    if digits.len() != expected_digits {
        return Err(RuntimePiError::InternalInvariant);
    }

    let target = output
        .get_mut(..required)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let mut slots = target.iter_mut();
    let mut digit_bytes = digits.as_bytes().iter().copied();

    let first_digit = digit_bytes
        .next()
        .ok_or(RuntimePiError::InternalInvariant)?;
    let first_slot = slots.next().ok_or(RuntimePiError::InternalInvariant)?;
    *first_slot = first_digit;

    if decimal_places != 0 {
        let decimal_slot = slots.next().ok_or(RuntimePiError::InternalInvariant)?;
        *decimal_slot = b'.';

        for digit in digit_bytes {
            let slot = slots.next().ok_or(RuntimePiError::InternalInvariant)?;
            *slot = digit;
        }
    }

    Ok(required)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_progression_accepts_limit_and_rejects_beyond_it() {
        assert_eq!(next_guard_places(48), Ok(56));
        assert_eq!(next_guard_places(56), Ok(64));
        assert_eq!(
            next_guard_places(64),
            Err(RuntimePiError::InternalInvariant)
        );
        assert_eq!(
            next_guard_places(u32::MAX),
            Err(RuntimePiError::PrecisionTooLarge)
        );
    }

    #[test]
    fn chudnovsky_matches_independent_machin_reference() {
        for decimal_places in [0_usize, 1, 2, 6, 15, 28, 40, 100, 1_000] {
            assert_eq!(
                certified_scaled_pi(decimal_places),
                certified_scaled_pi_machin(decimal_places),
                "mismatch at {decimal_places} decimal places"
            );
        }
    }

    #[test]
    fn scaled_integer_sqrt_is_a_floor_bound() {
        let scale = BigInt::from(10_u8).pow(80_u32);
        let radicand = BigInt::from(CHUDNOVSKY_SQRT_RADICAND) * &scale * &scale;
        let root = scaled_sqrt_10005_floor(&scale);
        let next = &root + 1_u8;

        assert!(&root * &root <= radicand);
        assert!(&next * &next > radicand);
    }

    #[test]
    fn arctan_bounds_cover_both_tail_directions() {
        assert_eq!(
            arctan_reciprocal_bounds(&BigInt::from(1_u8), 5, 16),
            Ok((BigInt::from(0_u8), BigInt::from(1_u8)))
        );
        assert_eq!(
            arctan_reciprocal_bounds(&BigInt::from(5_u8), 5, 16),
            Ok((BigInt::from(0_u8), BigInt::from(2_u8)))
        );
        assert_eq!(
            arctan_reciprocal_bounds(&BigInt::from(375_u16), 5, 16),
            Ok((BigInt::from(73_u8), BigInt::from(76_u8)))
        );
    }

    #[test]
    fn arctan_iteration_limit_rejects_nonconvergence() {
        assert_eq!(
            arctan_reciprocal_bounds(&BigInt::from(1_u8), 5, 0),
            Err(RuntimePiError::InternalInvariant)
        );
    }

    #[test]
    fn scaled_writer_defensive_paths_return_errors() {
        let mut empty = [];
        assert_eq!(
            write_scaled_pi(0, &BigInt::from(3_u8), &mut empty, 0),
            Err(RuntimePiError::InternalInvariant)
        );

        let mut one = [0_u8; 1];
        assert_eq!(
            write_scaled_pi(0, &BigInt::from(31_u8), &mut one, 1),
            Err(RuntimePiError::InternalInvariant)
        );
        assert_eq!(
            write_scaled_pi(1, &BigInt::from(31_u8), &mut one, 1),
            Err(RuntimePiError::InternalInvariant)
        );

        let mut two = [0_u8; 2];
        assert_eq!(
            write_scaled_pi(2, &BigInt::from(314_u16), &mut two, 2),
            Err(RuntimePiError::InternalInvariant)
        );
    }
}
