//! Optional runtime generation of π beyond the bounded constant tier.

use core::{cmp::Ordering, fmt};

#[cfg(test)]
use alloc::vec;
#[cfg(test)]
use alloc::vec::Vec;
#[cfg(test)]
use num_bigint::BigInt;

use crate::RoundingMode;
use crate::runtime_arith::{Nat, SignedNat};
use crate::runtime_precomputed::{PRECOMPUTED_DECIMAL_PLACES, PRECOMPUTED_PI_ASCII};

const MAX_GUARD_PLACES: u32 = 64;
const CHUDNOVSKY_A: u64 = 13_591_409;
const CHUDNOVSKY_B: u64 = 545_140_134;
const CHUDNOVSKY_C3_OVER_24: u64 = 10_939_058_860_032_000;
const CHUDNOVSKY_C_MULTIPLIER: u64 = 426_880;
const CHUDNOVSKY_SQRT_RADICAND: u64 = 10_005;
const CHUDNOVSKY_TERM_DIGITS: usize = 14;
const CHUDNOVSKY_TERM_MARGIN: usize = 8;
#[cfg(all(
    feature = "parallel-runtime",
    not(target_os = "none"),
    not(target_family = "wasm")
))]
const PARALLEL_SPLIT_MIN_TERMS: usize = 512;
#[cfg(all(
    feature = "parallel-runtime",
    not(target_os = "none"),
    not(target_family = "wasm")
))]
const PARALLEL_THREAD_STACK_BYTES: usize = 256 * 1_024;
#[cfg(test)]
const PELL_FUNDAMENTAL_P: u64 = 4_001;
#[cfg(test)]
const PELL_FUNDAMENTAL_Q: u64 = 40;
#[cfg(test)]
const PELL_DECIMAL_SAFETY_DIVISOR: usize = 7;
#[cfg(test)]
const PELL_EXPONENT_MARGIN: usize = 8;
const RECIPROCAL_SQRT_SEED_DIGITS: usize = 18;
const RECIPROCAL_SQRT_SEED: u64 = 9_997_500_937_109_545;
const RECIPROCAL_SQRT_GUARD_DIGITS: usize = 36;
const RECIPROCAL_DIVISION_SEED_DIGITS: usize = 18;
const RECIPROCAL_DIVISION_GUARD_DIGITS: usize = 54;
const MAX_CERTIFICATION_ADJUSTMENTS: u8 = 4;
const MAX_RECIPROCAL_SQRT_REFINEMENTS: u8 = 2;

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
/// This optional variable-cost path uses Perfectπ's specialized arbitrary-
/// precision integer engine and never routes through binary floating point.
/// Callers forwarding untrusted precision should prefer
/// generate_pi_ascii_with_limit.
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

    if uses_precomputed_prefix(decimal_places) {
        return write_precomputed_pi(decimal_places, rounding, output, required);
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

fn uses_precomputed_prefix(decimal_places: usize) -> bool {
    decimal_places <= PRECOMPUTED_DECIMAL_PLACES
}

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

fn write_precomputed_pi(
    decimal_places: usize,
    rounding: RoundingMode,
    output: &mut [u8],
    required: usize,
) -> Result<usize, RuntimePiError> {
    let source = PRECOMPUTED_PI_ASCII
        .get(..required)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let target = output
        .get_mut(..required)
        .ok_or(RuntimePiError::InternalInvariant)?;
    target.copy_from_slice(source);

    let increment = match rounding {
        RoundingMode::TowardZero | RoundingMode::TowardNegativeInfinity => false,
        RoundingMode::AwayFromZero | RoundingMode::TowardPositiveInfinity => true,
        RoundingMode::NearestTiesToEven | RoundingMode::NearestTiesAwayFromZero => {
            // π is irrational, so finite decimal rounding can never land on an
            // exact half-way tie. The next decimal digit therefore decides.
            let next_index = if decimal_places == 0 { 2 } else { required };
            matches!(PRECOMPUTED_PI_ASCII.get(next_index), Some(b'5'..=b'9'))
        }
    };

    if increment {
        increment_ascii_decimal(target)?;
    }

    Ok(required)
}

fn increment_ascii_decimal(text: &mut [u8]) -> Result<(), RuntimePiError> {
    let mut index = text.len();
    while index != 0 {
        index -= 1;
        let digit = text
            .get_mut(index)
            .ok_or(RuntimePiError::InternalInvariant)?;

        match *digit {
            b'.' => {}
            b'0'..=b'8' => {
                *digit += 1;
                return Ok(());
            }
            b'9' => *digit = b'0',
            _ => return Err(RuntimePiError::InternalInvariant),
        }
    }

    Err(RuntimePiError::InternalInvariant)
}

fn scaled_pi_for_rounding(
    decimal_places: usize,
    rounding: RoundingMode,
) -> Result<Nat, RuntimePiError> {
    match rounding {
        RoundingMode::TowardZero | RoundingMode::TowardNegativeInfinity => {
            certified_scaled_pi(decimal_places)
        }
        RoundingMode::AwayFromZero | RoundingMode::TowardPositiveInfinity => {
            Ok(certified_scaled_pi(decimal_places)?.add_small(1))
        }
        RoundingMode::NearestTiesToEven | RoundingMode::NearestTiesAwayFromZero => {
            let extended_places = decimal_places
                .checked_add(1)
                .ok_or(RuntimePiError::PrecisionTooLarge)?;
            let extended = certified_scaled_pi(extended_places)?;
            let next_digit = extended
                .mod_small(10)
                .ok_or(RuntimePiError::InternalInvariant)?;
            let (mut rounded, _) = extended
                .div_small(10)
                .ok_or(RuntimePiError::InternalInvariant)?;
            if next_digit >= 5 {
                rounded = rounded.add_small(1);
            }
            Ok(rounded)
        }
    }
}

/// Returns floor(π * 10^decimal_places) only after the Chudnovsky interval
/// proves that the exact value lies inside one integer bucket.
fn certified_scaled_pi(decimal_places: usize) -> Result<Nat, RuntimePiError> {
    let mut guard_places = 16_u32;

    loop {
        let guard_usize =
            usize::try_from(guard_places).map_err(|_| RuntimePiError::PrecisionTooLarge)?;
        let working_places = decimal_places
            .checked_add(guard_usize)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;

        let terms = working_places
            .checked_div(CHUDNOVSKY_TERM_DIGITS)
            .and_then(|value| value.checked_add(CHUDNOVSKY_TERM_MARGIN))
            .ok_or(RuntimePiError::PrecisionTooLarge)?;

        let (split, c_lower, c_upper) = split_and_constant_bounds(terms, working_places)?;
        let next_term_end = terms
            .checked_add(1)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let next_leaf = chudnovsky_binary_split(terms, next_term_end)?;
        let extended = combine_binary_splits(&split, &next_leaf);
        let bounds = alternating_series_bounds(&split, &extended, terms)?;

        let lower_numerator = c_lower.mul(bounds.upper_q);
        let lower_denominator = bounds.upper_t.mul_pow10(guard_usize);
        let upper_numerator = c_upper.mul(bounds.lower_q);
        let upper_denominator = bounds.lower_t.mul_pow10(guard_usize);

        let lower_floor = quotient_floor(&lower_numerator, &lower_denominator)?;
        let next_floor = lower_floor.add_small(1);

        let next_scaled = next_floor.mul(&upper_denominator);
        if upper_bound_is_below_next_integer(&next_scaled, &upper_numerator) {
            return Ok(lower_floor);
        }

        guard_places = next_guard_places(guard_places)?;
    }
}

struct BinarySplit {
    p: Nat,
    q: Nat,
    t: SignedNat,
}

fn split_and_constant_bounds_sequential(
    terms: usize,
    working_places: usize,
) -> Result<(BinarySplit, Nat, Nat), RuntimePiError> {
    let split = chudnovsky_binary_split(0, terms)?;
    let (c_lower, c_upper) = scaled_chudnovsky_constant_bounds(working_places)?;
    Ok((split, c_lower, c_upper))
}

#[cfg(all(
    feature = "parallel-runtime",
    not(target_os = "none"),
    not(target_family = "wasm")
))]
fn split_and_constant_bounds_parallel(
    terms: usize,
    working_places: usize,
) -> Result<(BinarySplit, Nat, Nat), RuntimePiError> {
    let midpoint = terms / 2;

    std::thread::scope(|scope| {
        let left_worker = std::thread::Builder::new()
            .name("perfect-pi-left".into())
            .stack_size(PARALLEL_THREAD_STACK_BYTES)
            .spawn_scoped(scope, move || chudnovsky_binary_split(0, midpoint));
        let right_worker = std::thread::Builder::new()
            .name("perfect-pi-right".into())
            .stack_size(PARALLEL_THREAD_STACK_BYTES)
            .spawn_scoped(scope, move || chudnovsky_binary_split(midpoint, terms));

        let constant_bounds = scaled_chudnovsky_constant_bounds(working_places);

        let left = match left_worker {
            Ok(handle) => match handle.join() {
                Ok(result) => result,
                Err(_) => chudnovsky_binary_split(0, midpoint),
            },
            Err(_) => chudnovsky_binary_split(0, midpoint),
        }?;
        let right = match right_worker {
            Ok(handle) => match handle.join() {
                Ok(result) => result,
                Err(_) => chudnovsky_binary_split(midpoint, terms),
            },
            Err(_) => chudnovsky_binary_split(midpoint, terms),
        }?;
        let (c_lower, c_upper) = constant_bounds?;
        let split = combine_binary_splits(&left, &right);

        Ok((split, c_lower, c_upper))
    })
}

#[cfg(all(
    feature = "parallel-runtime",
    not(target_os = "none"),
    not(target_family = "wasm")
))]
fn should_parallelize_split(terms: usize) -> bool {
    terms >= PARALLEL_SPLIT_MIN_TERMS
}

fn split_and_constant_bounds(
    terms: usize,
    working_places: usize,
) -> Result<(BinarySplit, Nat, Nat), RuntimePiError> {
    #[cfg(all(
        feature = "parallel-runtime",
        not(target_os = "none"),
        not(target_family = "wasm")
    ))]
    if should_parallelize_split(terms) {
        return split_and_constant_bounds_parallel(terms, working_places);
    }

    split_and_constant_bounds_sequential(terms, working_places)
}

fn chudnovsky_binary_split(start: usize, end: usize) -> Result<BinarySplit, RuntimePiError> {
    if start >= end {
        return Err(RuntimePiError::InternalInvariant);
    }

    if end - start == 1 {
        return chudnovsky_leaf(start);
    }

    let midpoint = start
        .checked_add((end - start) / 2)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    let left = chudnovsky_binary_split(start, midpoint)?;
    let right = chudnovsky_binary_split(midpoint, end)?;

    Ok(combine_binary_splits(&left, &right))
}

fn combine_binary_splits(left: &BinarySplit, right: &BinarySplit) -> BinarySplit {
    let p = left.p.mul(&right.p);
    let q = left.q.mul(&right.q);
    let left_t = left.t.mul_nat(&right.q);
    let right_t = right.t.mul_nat(&left.p);
    let t = left_t.add(&right_t);
    BinarySplit { p, q, t }
}

struct SeriesBounds<'a> {
    lower_t: &'a Nat,
    lower_q: &'a Nat,
    upper_t: &'a Nat,
    upper_q: &'a Nat,
}

fn alternating_series_bounds<'a>(
    partial: &'a BinarySplit,
    extended: &'a BinarySplit,
    term_count: usize,
) -> Result<SeriesBounds<'a>, RuntimePiError> {
    if !partial.t.is_positive() || !extended.t.is_positive() {
        return Err(RuntimePiError::InternalInvariant);
    }

    if term_count.is_multiple_of(2) {
        Ok(SeriesBounds {
            lower_t: partial.t.magnitude(),
            lower_q: &partial.q,
            upper_t: extended.t.magnitude(),
            upper_q: &extended.q,
        })
    } else {
        Ok(SeriesBounds {
            lower_t: extended.t.magnitude(),
            lower_q: &extended.q,
            upper_t: partial.t.magnitude(),
            upper_q: &partial.q,
        })
    }
}

fn upper_bound_is_below_next_integer(next_scaled: &Nat, upper_numerator: &Nat) -> bool {
    matches!(next_scaled.cmp(upper_numerator), Ordering::Greater)
}

fn gcd_u128(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn chudnovsky_leaf(index: usize) -> Result<BinarySplit, RuntimePiError> {
    if index == 0 {
        return Ok(BinarySplit {
            p: Nat::from_u64(1),
            q: Nat::from_u64(1),
            t: SignedNat::from_magnitude(Nat::from_u64(CHUDNOVSKY_A), false),
        });
    }

    let index_u64 = u64::try_from(index).map_err(|_| RuntimePiError::PrecisionTooLarge)?;
    let six_index = index_u64
        .checked_mul(6)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    let two_index = index_u64
        .checked_mul(2)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;

    let p_factor_1 = six_index
        .checked_sub(5)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let p_factor_2 = two_index
        .checked_sub(1)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let p_factor_3 = six_index
        .checked_sub(1)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let linear = index_u64
        .checked_mul(CHUDNOVSKY_B)
        .and_then(|value| value.checked_add(CHUDNOVSKY_A))
        .ok_or(RuntimePiError::PrecisionTooLarge)?;

    let scalar_p = u128::from(p_factor_1)
        .checked_mul(u128::from(p_factor_2))
        .and_then(|value| value.checked_mul(u128::from(p_factor_3)));
    let scalar_q = u128::from(index_u64)
        .checked_mul(u128::from(index_u64))
        .and_then(|value| value.checked_mul(u128::from(index_u64)))
        .and_then(|value| value.checked_mul(u128::from(CHUDNOVSKY_C3_OVER_24)));

    if let (Some(p_value), Some(q_value)) = (scalar_p, scalar_q)
        && let Some(t_value) = p_value.checked_mul(u128::from(linear))
    {
        let common = gcd_u128(p_value, q_value);
        if common == 0 {
            return Err(RuntimePiError::InternalInvariant);
        }

        let p = Nat::from_u128(p_value / common);
        let q = Nat::from_u128(q_value / common);
        let t =
            SignedNat::from_magnitude(Nat::from_u128(t_value / common), !index.is_multiple_of(2));
        return Ok(BinarySplit { p, q, t });
    }

    let p = Nat::from_u64(p_factor_1)
        .mul(&Nat::from_u64(p_factor_2))
        .mul(&Nat::from_u64(p_factor_3));
    let index_nat = Nat::from_u64(index_u64);
    let q = index_nat
        .mul(&index_nat)
        .mul(&index_nat)
        .mul_small(CHUDNOVSKY_C3_OVER_24);
    let t = SignedNat::from_magnitude(p.mul_small(linear), !index.is_multiple_of(2));

    Ok(BinarySplit { p, q, t })
}

fn scaled_chudnovsky_constant_bounds(scale_places: usize) -> Result<(Nat, Nat), RuntimePiError> {
    let sqrt_floor = scaled_sqrt_10005_floor(scale_places)?;
    let lower = sqrt_floor.mul_small(CHUDNOVSKY_C_MULTIPLIER);
    let upper = sqrt_floor.add_small(1).mul_small(CHUDNOVSKY_C_MULTIPLIER);
    Ok((lower, upper))
}

#[cfg(test)]
struct PellPair {
    p: Nat,
    q: Nat,
}

#[cfg(test)]
fn multiply_pell_pairs(left: &PellPair, right: &PellPair) -> PellPair {
    let p = left
        .p
        .mul(&right.p)
        .add(&left.q.mul(&right.q).mul_small(CHUDNOVSKY_SQRT_RADICAND));
    let q = left.p.mul(&right.q).add(&right.p.mul(&left.q));
    PellPair { p, q }
}

#[cfg(test)]
fn pell_power(mut exponent: usize) -> PellPair {
    let mut accumulator = PellPair {
        p: Nat::from_u64(1),
        q: Nat::zero(),
    };
    let mut base = PellPair {
        p: Nat::from_u64(PELL_FUNDAMENTAL_P),
        q: Nat::from_u64(PELL_FUNDAMENTAL_Q),
    };

    while exponent != 0 {
        if exponent & 1 == 1 {
            accumulator = multiply_pell_pairs(&accumulator, &base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = multiply_pell_pairs(&base, &base);
        }
    }

    accumulator
}

#[cfg(test)]
fn scaled_sqrt_10005_floor_pell(scale_places: usize) -> Result<Nat, RuntimePiError> {
    let mut pell_exponent = scale_places
        .checked_div(PELL_DECIMAL_SAFETY_DIVISOR)
        .and_then(|value| value.checked_add(PELL_EXPONENT_MARGIN))
        .ok_or(RuntimePiError::PrecisionTooLarge)?;

    loop {
        let pell = pell_power(pell_exponent);

        // Pell +1 gives D*q/p < sqrt(D) < p/q.  The lower rational provides
        // a candidate floor; the upper rational proves that sqrt(D) is still
        // below the next integer at the requested decimal scale.
        let lower_numerator = pell
            .q
            .mul_small(CHUDNOVSKY_SQRT_RADICAND)
            .mul_pow10(scale_places);
        let lower_floor = quotient_floor(&lower_numerator, &pell.p)?;
        let upper_numerator = pell.p.mul_pow10(scale_places);
        let next_floor = lower_floor.add_small(1);

        if next_floor.mul(&pell.q).cmp(&upper_numerator) == Ordering::Greater {
            return Ok(lower_floor);
        }

        pell_exponent = pell_exponent
            .checked_add(PELL_EXPONENT_MARGIN)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
    }
}

fn refine_reciprocal_sqrt(reciprocal: &Nat, precision: usize) -> Result<Nat, RuntimePiError> {
    let square = reciprocal.square();
    let twice_precision = precision
        .checked_mul(2)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    let three_scale_squared = Nat::pow10(twice_precision).mul_small(3);
    let scaled_term = square.mul_small(CHUDNOVSKY_SQRT_RADICAND);
    let correction = three_scale_squared
        .checked_sub(&scaled_term)
        .ok_or(RuntimePiError::InternalInvariant)?;

    let product = reciprocal.mul(&correction);
    let shifted = product
        .div_pow10_floor(twice_precision)
        .ok_or(RuntimePiError::InternalInvariant)?;
    shifted
        .div_small(2)
        .map(|(value, _)| value)
        .ok_or(RuntimePiError::InternalInvariant)
}

fn scaled_sqrt_10005_floor(scale_places: usize) -> Result<Nat, RuntimePiError> {
    let target_digits = scale_places
        .checked_add(RECIPROCAL_SQRT_GUARD_DIGITS)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    let mut precision = RECIPROCAL_SQRT_SEED_DIGITS.min(target_digits);
    let mut reciprocal = Nat::from_u64(RECIPROCAL_SQRT_SEED);

    loop {
        let remaining = target_digits
            .checked_sub(precision)
            .ok_or(RuntimePiError::InternalInvariant)?;
        if remaining == 0 {
            break;
        }

        let doubled = precision
            .checked_mul(2)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let next_precision = doubled.min(target_digits);
        let growth = next_precision
            .checked_sub(precision)
            .ok_or(RuntimePiError::InternalInvariant)?;
        reciprocal = reciprocal.mul_pow10(growth);
        reciprocal = refine_reciprocal_sqrt(&reciprocal, next_precision)?;
        precision = next_precision;
    }

    let drop_digits = target_digits
        .checked_sub(scale_places)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let twice_places = scale_places
        .checked_mul(2)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    let radicand = Nat::pow10(twice_places).mul_small(CHUDNOVSKY_SQRT_RADICAND);
    let mut refinements = 0_u8;

    loop {
        let mut candidate = reciprocal
            .mul_small(CHUDNOVSKY_SQRT_RADICAND)
            .div_pow10_floor(drop_digits)
            .ok_or(RuntimePiError::InternalInvariant)?;
        let mut adjustments = 0_u8;

        loop {
            let square = candidate.square();
            if matches!(square.cmp(&radicand), Ordering::Greater) {
                if certification_adjustment_limit_reached(adjustments) {
                    break;
                }
                candidate = candidate
                    .checked_sub_small(1)
                    .ok_or(RuntimePiError::InternalInvariant)?;
                adjustments = adjustments.saturating_add(1);
                continue;
            }

            let twice_candidate = candidate.mul_small(2);
            let next_square = square.add(&twice_candidate).add_small(1);
            if !matches!(next_square.cmp(&radicand), Ordering::Greater) {
                if certification_adjustment_limit_reached(adjustments) {
                    break;
                }
                candidate = candidate.add_small(1);
                adjustments = adjustments.saturating_add(1);
                continue;
            }

            return Ok(candidate);
        }

        if reciprocal_sqrt_refinement_limit_reached(refinements) {
            return Err(RuntimePiError::InternalInvariant);
        }

        reciprocal = refine_reciprocal_sqrt(&reciprocal, target_digits)?;
        refinements = refinements.saturating_add(1);
    }
}

fn quotient_floor(numerator: &Nat, denominator: &Nat) -> Result<Nat, RuntimePiError> {
    if denominator.is_zero() {
        return Err(RuntimePiError::InternalInvariant);
    }
    match numerator.cmp(denominator) {
        Ordering::Less => return Ok(Nat::zero()),
        Ordering::Equal => return Ok(Nat::from_u64(1)),
        Ordering::Greater => {}
    }

    let numerator_digits = numerator.decimal_digits();
    let denominator_digits = denominator.decimal_digits();
    let quotient_digits = numerator_digits
        .checked_sub(denominator_digits)
        .and_then(|value| value.checked_add(1))
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    let target_precision = quotient_digits
        .checked_add(RECIPROCAL_DIVISION_GUARD_DIGITS)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;

    let mut precision = RECIPROCAL_DIVISION_SEED_DIGITS;
    let leading = normalized_denominator(denominator, denominator_digits, precision)?;
    let leading_u64 = leading.to_u64().ok_or(RuntimePiError::InternalInvariant)?;
    if leading_u64 == 0 {
        return Err(RuntimePiError::InternalInvariant);
    }

    let seed_scale = 10_u128.pow((RECIPROCAL_DIVISION_SEED_DIGITS * 2) as u32);
    let seed = seed_scale / u128::from(leading_u64);
    let mut reciprocal = Nat::from_u128(seed);

    loop {
        let remaining = target_precision
            .checked_sub(precision)
            .ok_or(RuntimePiError::InternalInvariant)?;
        if remaining == 0 {
            break;
        }

        let doubled = precision
            .checked_mul(2)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let next_precision = doubled.min(target_precision);
        let growth = next_precision
            .checked_sub(precision)
            .ok_or(RuntimePiError::InternalInvariant)?;
        reciprocal = reciprocal.mul_pow10(growth);

        let normalized = normalized_denominator(denominator, denominator_digits, next_precision)?;
        let scaled_product = normalized
            .mul(&reciprocal)
            .div_pow10_floor(next_precision)
            .ok_or(RuntimePiError::InternalInvariant)?;
        let twice_scale = Nat::pow10(next_precision).mul_small(2);
        let correction = twice_scale
            .checked_sub(&scaled_product)
            .ok_or(RuntimePiError::InternalInvariant)?;
        reciprocal = reciprocal
            .mul(&correction)
            .div_pow10_floor(next_precision)
            .ok_or(RuntimePiError::InternalInvariant)?;
        precision = next_precision;
    }

    let normalized_numerator =
        normalized_numerator(numerator, denominator_digits, target_precision)?;
    let product = normalized_numerator.mul(&reciprocal);
    let quotient_shift = target_precision
        .checked_mul(2)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    let candidate = product
        .div_pow10_floor(quotient_shift)
        .ok_or(RuntimePiError::InternalInvariant)?;
    if let Some(certified) = certify_quotient_candidate(candidate, numerator, denominator)? {
        return Ok(certified);
    }

    retry_quotient_candidate(
        &reciprocal,
        &normalized_numerator,
        numerator,
        denominator,
        denominator_digits,
        target_precision,
        quotient_shift,
    )
}

#[cold]
#[inline(never)]
fn retry_quotient_candidate(
    reciprocal: &Nat,
    normalized_numerator: &Nat,
    numerator: &Nat,
    denominator: &Nat,
    denominator_digits: usize,
    target_precision: usize,
    quotient_shift: usize,
) -> Result<Nat, RuntimePiError> {
    let normalized = normalized_denominator(denominator, denominator_digits, target_precision)?;
    let scaled_product = normalized
        .mul(reciprocal)
        .div_pow10_floor(target_precision)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let twice_scale = Nat::pow10(target_precision).mul_small(2);
    let correction = twice_scale
        .checked_sub(&scaled_product)
        .ok_or(RuntimePiError::InternalInvariant)?;
    let reciprocal = reciprocal
        .mul(&correction)
        .div_pow10_floor(target_precision)
        .ok_or(RuntimePiError::InternalInvariant)?;

    let product = normalized_numerator.mul(&reciprocal);
    let candidate = product
        .div_pow10_floor(quotient_shift)
        .ok_or(RuntimePiError::InternalInvariant)?;
    certify_quotient_candidate(candidate, numerator, denominator)?
        .ok_or(RuntimePiError::InternalInvariant)
}

fn certify_quotient_candidate(
    mut candidate: Nat,
    numerator: &Nat,
    denominator: &Nat,
) -> Result<Option<Nat>, RuntimePiError> {
    for _ in 0..MAX_CERTIFICATION_ADJUSTMENTS {
        let candidate_product = candidate.mul(denominator);
        match candidate_product.cmp(numerator) {
            Ordering::Greater => {
                candidate = candidate
                    .checked_sub_small(1)
                    .ok_or(RuntimePiError::InternalInvariant)?;
                continue;
            }
            Ordering::Equal | Ordering::Less => {}
        }

        let next_product = candidate_product.add(denominator);
        match next_product.cmp(numerator) {
            Ordering::Less | Ordering::Equal => candidate = candidate.add_small(1),
            Ordering::Greater => return Ok(Some(candidate)),
        }
    }

    let candidate_product = candidate.mul(denominator);
    if matches!(candidate_product.cmp(numerator), Ordering::Greater) {
        return Ok(None);
    }
    let next_product = candidate_product.add(denominator);
    if matches!(
        next_product.cmp(numerator),
        Ordering::Less | Ordering::Equal
    ) {
        Ok(None)
    } else {
        Ok(Some(candidate))
    }
}

fn normalized_denominator(
    denominator: &Nat,
    denominator_digits: usize,
    precision: usize,
) -> Result<Nat, RuntimePiError> {
    match denominator_digits.cmp(&precision) {
        Ordering::Greater => {
            let drop_digits = denominator_digits
                .checked_sub(precision)
                .ok_or(RuntimePiError::InternalInvariant)?;
            denominator
                .div_pow10_floor(drop_digits)
                .ok_or(RuntimePiError::InternalInvariant)
        }
        Ordering::Equal => Ok(denominator.clone()),
        Ordering::Less => {
            let add_digits = precision
                .checked_sub(denominator_digits)
                .ok_or(RuntimePiError::InternalInvariant)?;
            Ok(denominator.mul_pow10(add_digits))
        }
    }
}

fn normalized_numerator(
    numerator: &Nat,
    denominator_digits: usize,
    precision: usize,
) -> Result<Nat, RuntimePiError> {
    match denominator_digits.cmp(&precision) {
        Ordering::Greater => {
            let drop_digits = denominator_digits
                .checked_sub(precision)
                .ok_or(RuntimePiError::InternalInvariant)?;
            numerator
                .div_pow10_floor(drop_digits)
                .ok_or(RuntimePiError::InternalInvariant)
        }
        Ordering::Equal => Ok(numerator.clone()),
        Ordering::Less => {
            let add_digits = precision
                .checked_sub(denominator_digits)
                .ok_or(RuntimePiError::InternalInvariant)?;
            Ok(numerator.mul_pow10(add_digits))
        }
    }
}

fn certification_adjustment_limit_reached(adjustments: u8) -> bool {
    adjustments >= MAX_CERTIFICATION_ADJUSTMENTS
}

fn reciprocal_sqrt_refinement_limit_reached(refinements: u8) -> bool {
    refinements >= MAX_RECIPROCAL_SQRT_REFINEMENTS
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

fn write_scaled_pi(
    decimal_places: usize,
    scaled: &Nat,
    output: &mut [u8],
    required: usize,
) -> Result<usize, RuntimePiError> {
    let written = scaled
        .write_scaled_decimal(decimal_places, output)
        .ok_or(RuntimePiError::InternalInvariant)?;
    if written == required {
        Ok(written)
    } else {
        Err(RuntimePiError::InternalInvariant)
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
fn machin_pi_bounds(scale: &BigInt, max_terms: usize) -> Result<(BigInt, BigInt), RuntimePiError> {
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

#[cfg(test)]
fn bigint_scaled_ascii(decimal_places: usize, scaled: &BigInt) -> Result<Vec<u8>, RuntimePiError> {
    let digits = scaled.to_str_radix(10).into_bytes();
    let expected_digits = decimal_places
        .checked_add(1)
        .ok_or(RuntimePiError::PrecisionTooLarge)?;
    if digits.len() != expected_digits {
        return Err(RuntimePiError::InternalInvariant);
    }

    if decimal_places == 0 {
        return Ok(digits);
    }

    let required = runtime_pi_ascii_len(decimal_places)?;
    let mut output = Vec::with_capacity(required);
    let Some(&first) = digits.first() else {
        return Err(RuntimePiError::InternalInvariant);
    };
    output.push(first);
    output.push(b'.');
    let Some(rest) = digits.get(1..) else {
        return Err(RuntimePiError::InternalInvariant);
    };
    output.extend_from_slice(rest);
    Ok(output)
}

#[cfg(test)]
mod tests {
    extern crate std;

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
    fn arithmetic_control_boundaries_are_strict() -> Result<(), RuntimePiError> {
        assert!(upper_bound_is_below_next_integer(
            &Nat::from_u64(11),
            &Nat::from_u64(10)
        ));
        assert!(!upper_bound_is_below_next_integer(
            &Nat::from_u64(10),
            &Nat::from_u64(10)
        ));
        assert!(!upper_bound_is_below_next_integer(
            &Nat::from_u64(9),
            &Nat::from_u64(10)
        ));

        assert_eq!(gcd_u128(0, 0), 0);
        assert_eq!(gcd_u128(54, 24), 6);
        assert_eq!(gcd_u128(17, 13), 1);

        let leaf_one = chudnovsky_leaf(1)?;
        let leaf_two = chudnovsky_leaf(2)?;
        assert_eq!(
            leaf_one.t,
            SignedNat::from_magnitude(leaf_one.t.magnitude().clone(), true)
        );
        assert_eq!(
            leaf_two.t,
            SignedNat::from_magnitude(leaf_two.t.magnitude().clone(), false)
        );

        // Force the arbitrary-precision leaf fallback after the u128 fast path
        // overflows, and verify that Chudnovsky's alternating sign is preserved
        // there as well as in the scalar path.
        let fallback_odd = chudnovsky_leaf(100_000_001)?;
        let fallback_even = chudnovsky_leaf(100_000_002)?;
        assert_eq!(
            fallback_odd.t,
            SignedNat::from_magnitude(fallback_odd.t.magnitude().clone(), true)
        );
        assert_eq!(
            fallback_even.t,
            SignedNat::from_magnitude(fallback_even.t.magnitude().clone(), false)
        );

        assert_eq!(
            quotient_floor(&Nat::from_u64(5), &Nat::from_u64(5)),
            Ok(Nat::from_u64(1))
        );
        assert_eq!(
            quotient_floor(&Nat::from_u64(14), &Nat::from_u64(5)),
            Ok(Nat::from_u64(2))
        );
        assert_eq!(
            quotient_floor(&Nat::from_u64(15), &Nat::from_u64(5)),
            Ok(Nat::from_u64(3))
        );

        assert_eq!(
            normalized_denominator(&Nat::from_u64(123), 3, 3)?,
            Nat::from_u64(123)
        );
        assert_eq!(
            normalized_numerator(&Nat::from_u64(123), 3, 3)?,
            Nat::from_u64(123)
        );

        assert!(!certification_adjustment_limit_reached(
            MAX_CERTIFICATION_ADJUSTMENTS - 1
        ));
        assert!(certification_adjustment_limit_reached(
            MAX_CERTIFICATION_ADJUSTMENTS
        ));
        assert!(!reciprocal_sqrt_refinement_limit_reached(
            MAX_RECIPROCAL_SQRT_REFINEMENTS - 1
        ));
        assert!(reciprocal_sqrt_refinement_limit_reached(
            MAX_RECIPROCAL_SQRT_REFINEMENTS
        ));
        let (dispatch, lower, upper) = split_and_constant_bounds(8, 64)?;
        let (sequential, sequential_lower, sequential_upper) =
            split_and_constant_bounds_sequential(8, 64)?;
        assert_eq!(dispatch.p, sequential.p);
        assert_eq!(dispatch.q, sequential.q);
        assert_eq!(dispatch.t, sequential.t);
        assert_eq!(lower, sequential_lower);
        assert_eq!(upper, sequential_upper);

        #[cfg(all(
            feature = "parallel-runtime",
            not(target_os = "none"),
            not(target_family = "wasm")
        ))]
        {
            assert!(!should_parallelize_split(PARALLEL_SPLIT_MIN_TERMS - 1));
            assert!(should_parallelize_split(PARALLEL_SPLIT_MIN_TERMS));
            assert!(should_parallelize_split(PARALLEL_SPLIT_MIN_TERMS + 1));
        }

        Ok(())
    }

    #[test]
    fn binary_split_composition_is_exact() {
        let left = BinarySplit {
            p: Nat::from_u64(2),
            q: Nat::from_u64(3),
            t: SignedNat::from_magnitude(Nat::from_u64(5), false),
        };
        let right = BinarySplit {
            p: Nat::from_u64(7),
            q: Nat::from_u64(11),
            t: SignedNat::from_magnitude(Nat::from_u64(13), false),
        };

        let combined = combine_binary_splits(&left, &right);
        assert_eq!(combined.p, Nat::from_u64(14));
        assert_eq!(combined.q, Nat::from_u64(33));
        assert_eq!(
            combined.t,
            SignedNat::from_magnitude(Nat::from_u64(81), false)
        );
    }

    #[cfg(all(
        feature = "parallel-runtime",
        not(target_os = "none"),
        not(target_family = "wasm")
    ))]
    #[test]
    fn parallel_split_matches_sequential_split() -> Result<(), RuntimePiError> {
        let terms = 64_usize;
        let working_places = 512_usize;

        let (sequential, sequential_lower, sequential_upper) =
            split_and_constant_bounds_sequential(terms, working_places)?;
        let (parallel, parallel_lower, parallel_upper) =
            split_and_constant_bounds_parallel(terms, working_places)?;

        assert_eq!(parallel.p, sequential.p);
        assert_eq!(parallel.q, sequential.q);
        assert_eq!(parallel.t, sequential.t);
        assert_eq!(parallel_lower, sequential_lower);
        assert_eq!(parallel_upper, sequential_upper);
        Ok(())
    }

    #[test]
    fn series_bounds_follow_alternating_partial_sum_parity() {
        let partial = BinarySplit {
            p: Nat::from_u64(1),
            q: Nat::from_u64(2),
            t: SignedNat::from_magnitude(Nat::from_u64(3), false),
        };
        let extended = BinarySplit {
            p: Nat::from_u64(1),
            q: Nat::from_u64(5),
            t: SignedNat::from_magnitude(Nat::from_u64(7), false),
        };

        let even = alternating_series_bounds(&partial, &extended, 2);
        assert!(matches!(
            even,
            Ok(SeriesBounds {
                lower_q,
                upper_q,
                ..
            }) if *lower_q == Nat::from_u64(2) && *upper_q == Nat::from_u64(5)
        ));

        let odd = alternating_series_bounds(&partial, &extended, 3);
        assert!(matches!(
            odd,
            Ok(SeriesBounds {
                lower_q,
                upper_q,
                ..
            }) if *lower_q == Nat::from_u64(5) && *upper_q == Nat::from_u64(2)
        ));
    }

    #[test]
    fn chudnovsky_constant_bounds_are_exact_at_unit_scale() {
        assert_eq!(
            scaled_chudnovsky_constant_bounds(0),
            Ok((Nat::from_u64(42_688_000), Nat::from_u64(43_114_880)))
        );
    }

    #[test]
    fn pell_fundamental_solution_and_powers_are_exact() {
        for exponent in [1_usize, 2, 3, 7, 16] {
            let pair = pell_power(exponent);
            let p_squared = pair.p.square();
            let dq_squared = pair.q.square().mul_small(CHUDNOVSKY_SQRT_RADICAND);
            assert_eq!(
                p_squared.checked_sub(&dq_squared),
                Some(Nat::from_u64(1)),
                "Pell invariant failed at exponent {exponent}"
            );
        }
    }

    #[test]
    fn reciprocal_sqrt_matches_pell_reference() -> Result<(), RuntimePiError> {
        for places in [0_usize, 1, 40, 100, 1_000] {
            assert_eq!(
                scaled_sqrt_10005_floor(places)?,
                scaled_sqrt_10005_floor_pell(places)?,
                "sqrt mismatch at {places} places"
            );
        }
        Ok(())
    }

    #[test]
    fn specialized_engine_matches_independent_machin_reference() {
        for decimal_places in [0_usize, 1, 2, 6, 15, 28, 40, 100, 1_000] {
            let actual = certified_scaled_pi(decimal_places);
            let expected = certified_scaled_pi_machin(decimal_places);
            assert!(actual.is_ok());
            assert!(expected.is_ok());

            if let (Ok(actual), Ok(expected)) = (actual, expected) {
                let required = runtime_pi_ascii_len(decimal_places);
                assert!(required.is_ok());
                if let Ok(required) = required {
                    let mut actual_ascii = vec![0_u8; required];
                    let actual_written =
                        actual.write_scaled_decimal(decimal_places, &mut actual_ascii);
                    let expected_ascii = bigint_scaled_ascii(decimal_places, &expected);
                    assert_eq!(actual_written, Some(required));
                    assert_eq!(expected_ascii, Ok(actual_ascii));
                }
            }
        }
    }

    #[test]
    fn scaled_integer_sqrt_is_a_floor_bound() {
        let places = 80_usize;
        let result = scaled_sqrt_10005_floor(places);
        assert!(result.is_ok());

        if let Ok(root) = result {
            let radicand = Nat::pow10(places.saturating_mul(2)).mul_small(CHUDNOVSKY_SQRT_RADICAND);
            let square = root.square();
            let next_square = square.add(&root.mul_small(2)).add_small(1);
            assert!(square <= radicand);
            assert!(next_square > radicand);
        }
    }

    #[test]
    fn quotient_floor_is_exact() {
        let numerator = Nat::pow10(500).add_small(123_456_789);
        let denominator = Nat::pow10(173).add_small(987_654_321);
        let quotient = quotient_floor(&numerator, &denominator);
        assert!(quotient.is_ok());

        if let Ok(quotient) = quotient {
            let product = quotient.mul(&denominator);
            let next_product = product.add(&denominator);
            assert!(product <= numerator);
            assert!(next_product > numerator);
        }
    }

    #[test]
    fn reciprocal_division_retry_recovers_out_of_window_candidate() -> Result<(), RuntimePiError> {
        let numerator = Nat::from_u64(1_000);
        let denominator = Nat::from_u64(3);
        let denominator_digits = denominator.decimal_digits();
        let target_precision = 4_usize;
        let quotient_shift = target_precision
            .checked_mul(2)
            .ok_or(RuntimePiError::PrecisionTooLarge)?;
        let normalized_numerator =
            normalized_numerator(&numerator, denominator_digits, target_precision)?;
        let reciprocal = Nat::from_u64(32_000);

        let initial_product = normalized_numerator.mul(&reciprocal);
        let initial_candidate = initial_product
            .div_pow10_floor(quotient_shift)
            .ok_or(RuntimePiError::InternalInvariant)?;
        assert_eq!(initial_candidate, Nat::from_u64(320));
        assert_eq!(
            certify_quotient_candidate(initial_candidate, &numerator, &denominator)?,
            None
        );

        assert_eq!(
            retry_quotient_candidate(
                &reciprocal,
                &normalized_numerator,
                &numerator,
                &denominator,
                denominator_digits,
                target_precision,
                quotient_shift,
            )?,
            Nat::from_u64(333)
        );
        Ok(())
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
    fn arithmetic_handoff_matches_verified_prefix() -> Result<(), RuntimePiError> {
        let decimal_places = PRECOMPUTED_DECIMAL_PLACES + 1;
        let required = runtime_pi_ascii_len(decimal_places)?;
        let mut output = vec![0_u8; required];

        assert_eq!(generate_pi_ascii(decimal_places, &mut output), Ok(required));
        assert_eq!(
            output.as_slice(),
            PRECOMPUTED_PI_ASCII
                .get(..required)
                .ok_or(RuntimePiError::InternalInvariant)?
        );
        Ok(())
    }

    #[test]
    fn runtime_edge_and_helper_paths_are_explicit() -> Result<(), RuntimePiError> {
        assert!(uses_precomputed_prefix(0));
        assert!(uses_precomputed_prefix(PRECOMPUTED_DECIMAL_PLACES));
        assert!(!uses_precomputed_prefix(
            PRECOMPUTED_DECIMAL_PLACES.saturating_add(1)
        ));
        #[cfg(all(
            feature = "parallel-runtime",
            not(target_os = "none"),
            not(target_family = "wasm")
        ))]
        assert_eq!(PARALLEL_THREAD_STACK_BYTES, 256 * 1_024);

        let mut nearest = [0_u8; 4];
        assert_eq!(generate_pi_ascii_round_nearest_even(2, &mut nearest), Ok(4));
        assert_eq!(&nearest, b"3.14");

        let mut limited = [0_u8; 4];
        assert_eq!(
            generate_pi_ascii_with_limit(2, 1, RoundingMode::TowardZero, &mut limited),
            Err(RuntimePiError::PrecisionLimitExceeded {
                requested: 2,
                limit: 1,
            })
        );
        assert_eq!(
            generate_pi_ascii_with_limit(2, 2, RoundingMode::TowardZero, &mut limited),
            Ok(4)
        );
        assert_eq!(&limited, b"3.14");

        assert_eq!(
            std::format!(
                "{}",
                RuntimePiError::BufferTooSmall {
                    required: 4,
                    provided: 3,
                }
            ),
            "output buffer is too small"
        );
        assert_eq!(
            std::format!("{}", RuntimePiError::PrecisionTooLarge),
            "requested precision is too large"
        );
        assert_eq!(
            std::format!(
                "{}",
                RuntimePiError::PrecisionLimitExceeded {
                    requested: 2,
                    limit: 1,
                }
            ),
            "requested precision exceeds the configured limit"
        );
        assert_eq!(
            std::format!("{}", RuntimePiError::InternalInvariant),
            "runtime pi generation invariant failed"
        );

        let mut carrying = *b"3.99";
        assert_eq!(increment_ascii_decimal(&mut carrying), Ok(()));
        assert_eq!(&carrying, b"4.00");
        let mut invalid = *b"3.a";
        assert_eq!(
            increment_ascii_decimal(&mut invalid),
            Err(RuntimePiError::InternalInvariant)
        );
        let mut all_nines = *b"9";
        assert_eq!(
            increment_ascii_decimal(&mut all_nines),
            Err(RuntimePiError::InternalInvariant)
        );

        assert_eq!(
            scaled_pi_for_rounding(2, RoundingMode::TowardZero)?,
            Nat::from_u64(314)
        );
        assert_eq!(
            scaled_pi_for_rounding(2, RoundingMode::TowardNegativeInfinity)?,
            Nat::from_u64(314)
        );
        assert_eq!(
            scaled_pi_for_rounding(2, RoundingMode::AwayFromZero)?,
            Nat::from_u64(315)
        );
        assert_eq!(
            scaled_pi_for_rounding(2, RoundingMode::TowardPositiveInfinity)?,
            Nat::from_u64(315)
        );
        assert_eq!(
            scaled_pi_for_rounding(2, RoundingMode::NearestTiesToEven)?,
            Nat::from_u64(314)
        );
        assert_eq!(
            scaled_pi_for_rounding(2, RoundingMode::NearestTiesAwayFromZero)?,
            Nat::from_u64(314)
        );
        assert_eq!(
            scaled_pi_for_rounding(3, RoundingMode::NearestTiesToEven)?,
            Nat::from_u64(3_142)
        );

        assert_eq!(
            quotient_floor(&Nat::from_u64(3), &Nat::zero()),
            Err(RuntimePiError::InternalInvariant)
        );
        assert_eq!(
            quotient_floor(&Nat::from_u64(3), &Nat::from_u64(5)),
            Ok(Nat::zero())
        );

        let normalized_source = Nat::pow10(30).add_small(123);
        let source_digits = normalized_source.decimal_digits();
        assert_eq!(
            normalized_denominator(&normalized_source, source_digits, 18)?,
            normalized_source
                .div_pow10_floor(source_digits - 18)
                .ok_or(RuntimePiError::InternalInvariant)?
        );
        assert_eq!(
            normalized_denominator(&Nat::from_u64(12), 2, 5)?,
            Nat::from_u64(12_000)
        );
        assert_eq!(
            normalized_numerator(&normalized_source, source_digits, 18)?,
            normalized_source
                .div_pow10_floor(source_digits - 18)
                .ok_or(RuntimePiError::InternalInvariant)?
        );
        assert_eq!(
            normalized_numerator(&Nat::from_u64(12), 2, 5)?,
            Nat::from_u64(12_000)
        );

        assert!(matches!(
            chudnovsky_binary_split(1, 1),
            Err(RuntimePiError::InternalInvariant)
        ));
        assert!(matches!(
            chudnovsky_leaf(usize::MAX),
            Err(RuntimePiError::PrecisionTooLarge)
        ));

        let invalid_split = BinarySplit {
            p: Nat::from_u64(1),
            q: Nat::from_u64(1),
            t: SignedNat::from_magnitude(Nat::from_u64(1), true),
        };
        let valid_split = BinarySplit {
            p: Nat::from_u64(1),
            q: Nat::from_u64(1),
            t: SignedNat::from_magnitude(Nat::from_u64(1), false),
        };
        assert!(matches!(
            alternating_series_bounds(&invalid_split, &valid_split, 2),
            Err(RuntimePiError::InternalInvariant)
        ));

        let scaled = Nat::from_u64(314);
        let mut exact = [0_u8; 4];
        assert_eq!(write_scaled_pi(2, &scaled, &mut exact, 4), Ok(4));
        assert_eq!(&exact, b"3.14");
        assert_eq!(
            write_scaled_pi(2, &scaled, &mut exact, 3),
            Err(RuntimePiError::InternalInvariant)
        );
        Ok(())
    }
}
