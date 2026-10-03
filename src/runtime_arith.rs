use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;

const BASE: u64 = 1_000_000_000_000_000_000;
const BASE_WIDE: u128 = BASE as u128;
const BASE_DIGITS: usize = 18;
const KARATSUBA_THRESHOLD: usize = 32;
const KARATSUBA_UNBALANCED_FACTOR: usize = 4;
const NTT_THRESHOLD: usize = 256;
const SINGLE_FAST_CONV_DIGITS: usize = 7;
const SINGLE_FAST_CONV_BASE: u64 = 10_000_000;
const SINGLE_CONV_BASE: u64 = 1_000_000;
const SINGLE_NTT_MOD: u64 = 9_223_372_006_790_004_737;
const SINGLE_NTT_ROOT: u64 = 3;
const SINGLE_NTT_MONT_INV: u64 = 9_223_372_006_790_004_735;
const SINGLE_NTT_MONT_R2: u64 = 11_544_872_091_260;
const CONV_BASE: u64 = 1_000_000_000;
const NTT_MOD_1: u64 = 1_125_899_437_080_577;
const NTT_MOD_2: u64 = 1_125_900_443_713_537;
const NTT_ROOT: u64 = 5;
const NTT_MONT_INV_1: u64 = 18_227_193_591_405_477_887;
const NTT_MONT_INV_2: u64 = 18_159_639_598_001_553_407;
const NTT_MONT_R2_1: u64 = 10_154_644_843_296;
const NTT_MONT_R2_2: u64 = 17_386_296_082_176;
const NTT_INV_M1_MOD_M2: u64 = 225_180_089_861_189;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MulStrategy {
    Schoolbook,
    Karatsuba,
    Ntt,
}

fn multiplication_strategy(shorter: usize, longer: usize) -> MulStrategy {
    let balanced_limit = shorter.saturating_mul(KARATSUBA_UNBALANCED_FACTOR);
    if shorter >= NTT_THRESHOLD && longer <= balanced_limit {
        MulStrategy::Ntt
    } else if shorter <= KARATSUBA_THRESHOLD || longer > balanced_limit {
        MulStrategy::Schoolbook
    } else {
        MulStrategy::Karatsuba
    }
}

fn should_use_ntt_square(limb_count: usize) -> bool {
    limb_count >= NTT_THRESHOLD
}

fn trim_trailing_zeroes(values: &mut Vec<u64>) {
    let normalized_len = values
        .iter()
        .rposition(|&value| value != 0)
        .map_or(0, |index| index.saturating_add(1));
    values.truncate(normalized_len);
}

fn append_radix_carry(values: &mut Vec<u64>, mut carry: u128, radix: u128) -> Option<()> {
    if matches!(radix, 0 | 1) {
        return None;
    }

    for _ in 0..u128::BITS {
        if carry == 0 {
            break;
        }
        values.push(carry.rem_euclid(radix) as u64);
        carry = carry.div_euclid(radix);
    }

    Some(())
}

const POW10: [u64; BASE_DIGITS] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Nat {
    limbs: Vec<u64>,
}

impl Ord for Nat {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.limbs.len().cmp(&other.limbs.len()) {
            Ordering::Equal => {
                for index in (0..self.limbs.len()).rev() {
                    match self.limbs[index].cmp(&other.limbs[index]) {
                        Ordering::Equal => {}
                        ordering => return ordering,
                    }
                }
                Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for Nat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Nat {
    pub(super) fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    pub(super) fn from_u64(value: u64) -> Self {
        Self::from_u128(u128::from(value))
    }

    pub(super) fn from_u128(value: u128) -> Self {
        if value == 0 {
            return Self::zero();
        }

        let mut limbs = Vec::with_capacity(3);
        limbs.push((value % BASE_WIDE) as u64);

        let upper = value / BASE_WIDE;
        if upper != 0 {
            limbs.push((upper % BASE_WIDE) as u64);
            let top = upper / BASE_WIDE;
            if top != 0 {
                limbs.push(top as u64);
            }
        }

        Self { limbs }
    }

    fn from_limbs(mut limbs: Vec<u64>) -> Self {
        trim_trailing_zeroes(&mut limbs);
        Self { limbs }
    }

    pub(super) fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    pub(super) fn decimal_digits(&self) -> usize {
        let Some(&top) = self.limbs.last() else {
            return 1;
        };
        let top_digits = match top.checked_ilog10() {
            Some(log) => log as usize + 1,
            None => 1,
        };
        (self.limbs.len() - 1)
            .saturating_mul(BASE_DIGITS)
            .saturating_add(top_digits)
    }

    pub(super) fn pow10(decimal_places: usize) -> Self {
        let whole_limbs = decimal_places.div_euclid(BASE_DIGITS);
        let remainder = decimal_places.rem_euclid(BASE_DIGITS);
        let mut limbs = Vec::with_capacity(whole_limbs.saturating_add(1));
        limbs.resize(whole_limbs, 0);
        limbs.push(POW10[remainder]);
        Self { limbs }
    }

    pub(super) fn add(&self, other: &Self) -> Self {
        let length = self.limbs.len().max(other.limbs.len());
        let mut limbs = Vec::with_capacity(length.saturating_add(1));
        let mut carry = 0_u128;

        for index in 0..length {
            let left = if index < self.limbs.len() {
                self.limbs[index]
            } else {
                0
            };
            let right = if index < other.limbs.len() {
                other.limbs[index]
            } else {
                0
            };
            let sum = u128::from(left)
                .saturating_add(u128::from(right))
                .saturating_add(carry);
            limbs.push(sum.rem_euclid(BASE_WIDE) as u64);
            carry = sum.div_euclid(BASE_WIDE);
        }

        limbs.push(carry as u64);
        Self::from_limbs(limbs)
    }

    pub(super) fn add_small(&self, value: u64) -> Self {
        let mut limbs = self.limbs.clone();
        let mut carry = u128::from(value);

        for limb in &mut limbs {
            if carry == 0 {
                break;
            }
            let sum = u128::from(*limb).saturating_add(carry);
            *limb = sum.rem_euclid(BASE_WIDE) as u64;
            carry = sum.div_euclid(BASE_WIDE);
        }

        limbs.push(carry.rem_euclid(BASE_WIDE) as u64);
        limbs.push(carry.div_euclid(BASE_WIDE) as u64);
        Self::from_limbs(limbs)
    }

    pub(super) fn checked_sub(&self, other: &Self) -> Option<Self> {
        if self.cmp(other) == Ordering::Less {
            return None;
        }

        let mut limbs = Vec::with_capacity(self.limbs.len());
        let mut borrow = 0_u128;

        for index in 0..self.limbs.len() {
            let left = u128::from(self.limbs[index]);
            let other_limb = if index < other.limbs.len() {
                other.limbs[index]
            } else {
                0
            };
            let right = u128::from(other_limb) + borrow;
            if left >= right {
                limbs.push(left.saturating_sub(right) as u64);
                borrow = 0;
            } else {
                limbs.push(left.saturating_add(BASE_WIDE).saturating_sub(right) as u64);
                borrow = 1;
            }
        }

        if borrow != 0 {
            return None;
        }

        Some(Self::from_limbs(limbs))
    }

    pub(super) fn checked_sub_small(&self, value: u64) -> Option<Self> {
        self.checked_sub(&Self::from_u64(value))
    }

    pub(super) fn mul_small(&self, value: u64) -> Self {
        let mut limbs = Vec::with_capacity(self.limbs.len().saturating_add(2));
        let mut carry = 0_u128;

        for &limb in &self.limbs {
            // Canonical limbs are below BASE and value is u64, so this is exact in u128.
            let product = u128::from(limb)
                .saturating_mul(u128::from(value))
                .saturating_add(carry);
            limbs.push(product.rem_euclid(BASE_WIDE) as u64);
            carry = product.div_euclid(BASE_WIDE);
        }

        limbs.push(carry.rem_euclid(BASE_WIDE) as u64);
        limbs.push(carry.div_euclid(BASE_WIDE) as u64);
        Self::from_limbs(limbs)
    }

    pub(super) fn mul(&self, other: &Self) -> Self {
        let shorter = self.limbs.len().min(other.limbs.len());
        let longer = self.limbs.len().max(other.limbs.len());

        match multiplication_strategy(shorter, longer) {
            MulStrategy::Schoolbook => self.mul_schoolbook(other),
            MulStrategy::Karatsuba => match self.mul_karatsuba(other) {
                Some(product) => product,
                None => self.mul_schoolbook(other),
            },
            MulStrategy::Ntt => match self.mul_ntt(other) {
                Some(product) => product,
                None => match self.mul_karatsuba(other) {
                    Some(product) => product,
                    None => self.mul_schoolbook(other),
                },
            },
        }
    }

    pub(super) fn square(&self) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        if should_use_ntt_square(self.limbs.len()) {
            if let Some(square) = self.square_ntt_single_modulus_fast() {
                return square;
            }
            if let Some(square) = self.square_ntt_single_modulus() {
                return square;
            }
        }

        self.mul(self)
    }

    fn square_ntt_single_modulus_fast(&self) -> Option<Self> {
        let values = self.to_fast_single_convolution_coefficients()?;
        if values.is_empty() {
            return Some(Self::zero());
        }

        let result_coefficients = values.len().checked_mul(2)?.checked_sub(1)?;
        let transform_len = result_coefficients.checked_next_power_of_two()?;
        let transform_len_u64 = u64::try_from(transform_len).ok()?;
        if !SINGLE_NTT_MOD
            .saturating_sub(1)
            .is_multiple_of(transform_len_u64)
        {
            return None;
        }

        let overlap = u128::try_from(values.len()).ok()?;
        let max_digit = u128::from(SINGLE_FAST_CONV_BASE.saturating_sub(1));
        let coefficient_bound = overlap.checked_mul(max_digit)?.checked_mul(max_digit)?;
        if coefficient_bound >= u128::from(SINGLE_NTT_MOD) {
            return None;
        }

        let modulus = MontgomeryModulus::new(
            SINGLE_NTT_MOD,
            SINGLE_NTT_MONT_INV,
            SINGLE_NTT_MONT_R2,
            SINGLE_NTT_ROOT,
        );
        let residues = convolution_square_mod(&values, transform_len, modulus)?;

        let radix = u128::from(SINGLE_FAST_CONV_BASE);
        let mut coefficients = Vec::with_capacity(result_coefficients.saturating_add(4));
        let mut carry = 0_u128;
        for &residue in residues.get(..result_coefficients)? {
            let value = u128::from(residue).checked_add(carry)?;
            coefficients.push((value % radix) as u64);
            carry = value / radix;
        }

        append_radix_carry(&mut coefficients, carry, radix)?;

        Self::from_fast_single_convolution_coefficients(&coefficients)
    }

    fn square_ntt_single_modulus(&self) -> Option<Self> {
        let values = self.to_single_convolution_coefficients();
        if values.is_empty() {
            return Some(Self::zero());
        }

        let result_coefficients = values.len().checked_mul(2)?.checked_sub(1)?;
        let transform_len = result_coefficients.checked_next_power_of_two()?;
        let transform_len_u64 = u64::try_from(transform_len).ok()?;
        if !SINGLE_NTT_MOD
            .saturating_sub(1)
            .is_multiple_of(transform_len_u64)
        {
            return None;
        }

        let overlap = u128::try_from(values.len()).ok()?;
        let max_digit = u128::from(SINGLE_CONV_BASE.saturating_sub(1));
        let coefficient_bound = overlap.checked_mul(max_digit)?.checked_mul(max_digit)?;
        if coefficient_bound >= u128::from(SINGLE_NTT_MOD) {
            return None;
        }

        let modulus = MontgomeryModulus::new(
            SINGLE_NTT_MOD,
            SINGLE_NTT_MONT_INV,
            SINGLE_NTT_MONT_R2,
            SINGLE_NTT_ROOT,
        );
        let residues = convolution_square_mod(&values, transform_len, modulus)?;

        let mut coefficients = Vec::with_capacity(result_coefficients.saturating_add(4));
        let mut carry = 0_u128;
        for &residue in residues.get(..result_coefficients)? {
            let value = u128::from(residue).checked_add(carry)?;
            coefficients.push((value % u128::from(SINGLE_CONV_BASE)) as u64);
            carry = value / u128::from(SINGLE_CONV_BASE);
        }

        append_radix_carry(&mut coefficients, carry, u128::from(SINGLE_CONV_BASE))?;

        Some(Self::from_single_convolution_coefficients(&coefficients))
    }

    fn mul_ntt_single_modulus_fast(&self, other: &Self) -> Option<Self> {
        let left = self.to_fast_single_convolution_coefficients()?;
        let right = other.to_fast_single_convolution_coefficients()?;
        if left.is_empty() {
            return Some(Self::zero());
        }
        if right.is_empty() {
            return Some(Self::zero());
        }

        let result_coefficients = left.len().checked_add(right.len())?.checked_sub(1)?;
        let transform_len = result_coefficients.checked_next_power_of_two()?;
        let transform_len_u64 = u64::try_from(transform_len).ok()?;
        if !SINGLE_NTT_MOD
            .saturating_sub(1)
            .is_multiple_of(transform_len_u64)
        {
            return None;
        }

        let overlap = u128::try_from(left.len().min(right.len())).ok()?;
        let max_digit = u128::from(SINGLE_FAST_CONV_BASE.saturating_sub(1));
        let coefficient_bound = overlap.checked_mul(max_digit)?.checked_mul(max_digit)?;
        if coefficient_bound >= u128::from(SINGLE_NTT_MOD) {
            return None;
        }

        let modulus = MontgomeryModulus::new(
            SINGLE_NTT_MOD,
            SINGLE_NTT_MONT_INV,
            SINGLE_NTT_MONT_R2,
            SINGLE_NTT_ROOT,
        );
        let residues = convolution_mod(&left, &right, transform_len, modulus)?;

        let mut coefficients = Vec::with_capacity(result_coefficients.saturating_add(4));
        let mut carry = 0_u128;
        let radix = u128::from(SINGLE_FAST_CONV_BASE);
        for &residue in residues.get(..result_coefficients)? {
            let value = u128::from(residue).checked_add(carry)?;
            coefficients.push((value % radix) as u64);
            carry = value / radix;
        }

        append_radix_carry(&mut coefficients, carry, radix)?;

        Self::from_fast_single_convolution_coefficients(&coefficients)
    }

    fn to_fast_single_convolution_coefficients(&self) -> Option<Vec<u64>> {
        if self.is_zero() {
            return Some(Vec::new());
        }

        let coefficient_count = self
            .decimal_digits()
            .checked_add(SINGLE_FAST_CONV_DIGITS.saturating_sub(1))?
            .checked_div(SINGLE_FAST_CONV_DIGITS)?;
        let mut coefficients = Vec::with_capacity(coefficient_count);

        for coefficient_index in 0..coefficient_count {
            let decimal_offset = coefficient_index.checked_mul(SINGLE_FAST_CONV_DIGITS)?;
            let limb_index = decimal_offset.div_euclid(BASE_DIGITS);
            let offset_in_limb = decimal_offset.rem_euclid(BASE_DIGITS);
            let limb = *self.limbs.get(limb_index)?;
            let divisor = *POW10.get(offset_in_limb)?;
            let available_digits = BASE_DIGITS.saturating_sub(offset_in_limb);
            let remaining_digits = SINGLE_FAST_CONV_DIGITS.saturating_sub(available_digits);
            let mut coefficient = limb.div_euclid(divisor);

            if remaining_digits != 0 {
                let next_limb = match self.limbs.get(limb_index.checked_add(1)?) {
                    Some(&value) => value,
                    None => 0,
                };
                let next_modulus = *POW10.get(remaining_digits)?;
                let multiplier = *POW10.get(available_digits)?;
                coefficient = coefficient
                    .checked_add(next_limb.rem_euclid(next_modulus).checked_mul(multiplier)?)?;
            }

            coefficients.push(coefficient.rem_euclid(SINGLE_FAST_CONV_BASE));
        }

        trim_trailing_zeroes(&mut coefficients);
        Some(coefficients)
    }

    fn from_fast_single_convolution_coefficients(coefficients: &[u64]) -> Option<Self> {
        if coefficients.is_empty() {
            return Some(Self::zero());
        }

        let total_digits = coefficients.len().checked_mul(SINGLE_FAST_CONV_DIGITS)?;
        let limb_count = total_digits
            .checked_add(BASE_DIGITS.saturating_sub(1))?
            .checked_div(BASE_DIGITS)?;
        let mut limbs = vec![0_u64; limb_count];

        for (coefficient_index, &coefficient) in coefficients.iter().enumerate() {
            if coefficient >= SINGLE_FAST_CONV_BASE {
                return None;
            }

            let decimal_offset = coefficient_index.checked_mul(SINGLE_FAST_CONV_DIGITS)?;
            let limb_index = decimal_offset.div_euclid(BASE_DIGITS);
            let offset_in_limb = decimal_offset.rem_euclid(BASE_DIGITS);
            let available_digits = BASE_DIGITS.saturating_sub(offset_in_limb);
            let multiplier = *POW10.get(offset_in_limb)?;

            if available_digits >= SINGLE_FAST_CONV_DIGITS {
                let contribution = coefficient.checked_mul(multiplier)?;
                let limb = limbs.get_mut(limb_index)?;
                *limb = limb.checked_add(contribution)?;
                if *limb >= BASE {
                    return None;
                }
            } else {
                let split = *POW10.get(available_digits)?;
                let low = coefficient.rem_euclid(split);
                let high = coefficient.div_euclid(split);
                let low_contribution = low.checked_mul(multiplier)?;
                let limb = limbs.get_mut(limb_index)?;
                *limb = limb.checked_add(low_contribution)?;
                if *limb >= BASE {
                    return None;
                }

                if high != 0 {
                    let next_index = limb_index.checked_add(1)?;
                    let next_limb = limbs.get_mut(next_index)?;
                    *next_limb = next_limb.checked_add(high)?;
                    if *next_limb >= BASE {
                        return None;
                    }
                }
            }
        }

        Some(Self::from_limbs(limbs))
    }

    fn mul_ntt_single_modulus(&self, other: &Self) -> Option<Self> {
        if let Some(product) = self.mul_ntt_single_modulus_fast(other) {
            return Some(product);
        }

        self.mul_ntt_single_modulus_standard(other)
    }

    fn mul_ntt_single_modulus_standard(&self, other: &Self) -> Option<Self> {
        let left = self.to_single_convolution_coefficients();
        let right = other.to_single_convolution_coefficients();
        if left.is_empty() {
            return Some(Self::zero());
        }
        if right.is_empty() {
            return Some(Self::zero());
        }

        let result_coefficients = left.len().checked_add(right.len())?.checked_sub(1)?;
        let transform_len = result_coefficients.checked_next_power_of_two()?;
        let transform_len_u64 = u64::try_from(transform_len).ok()?;
        if !SINGLE_NTT_MOD
            .saturating_sub(1)
            .is_multiple_of(transform_len_u64)
        {
            return None;
        }

        let overlap = u128::try_from(left.len().min(right.len())).ok()?;
        let max_digit = u128::from(SINGLE_CONV_BASE.saturating_sub(1));
        let coefficient_bound = overlap.checked_mul(max_digit)?.checked_mul(max_digit)?;
        if coefficient_bound >= u128::from(SINGLE_NTT_MOD) {
            return None;
        }

        let modulus = MontgomeryModulus::new(
            SINGLE_NTT_MOD,
            SINGLE_NTT_MONT_INV,
            SINGLE_NTT_MONT_R2,
            SINGLE_NTT_ROOT,
        );
        let residues = convolution_mod(&left, &right, transform_len, modulus)?;

        let mut coefficients = Vec::with_capacity(result_coefficients.saturating_add(4));
        let mut carry = 0_u128;
        for &residue in residues.get(..result_coefficients)? {
            let value = u128::from(residue).checked_add(carry)?;
            coefficients.push((value % u128::from(SINGLE_CONV_BASE)) as u64);
            carry = value / u128::from(SINGLE_CONV_BASE);
        }

        append_radix_carry(&mut coefficients, carry, u128::from(SINGLE_CONV_BASE))?;

        Some(Self::from_single_convolution_coefficients(&coefficients))
    }

    fn to_single_convolution_coefficients(&self) -> Vec<u64> {
        let mut coefficients = Vec::with_capacity(self.limbs.len().saturating_mul(3));
        for &limb in &self.limbs {
            coefficients.push(limb % SINGLE_CONV_BASE);
            coefficients.push((limb / SINGLE_CONV_BASE) % SINGLE_CONV_BASE);
            coefficients.push(limb / (SINGLE_CONV_BASE * SINGLE_CONV_BASE));
        }
        trim_trailing_zeroes(&mut coefficients);
        coefficients
    }

    fn from_single_convolution_coefficients(coefficients: &[u64]) -> Self {
        if coefficients.is_empty() {
            return Self::zero();
        }

        let mut limbs = Vec::with_capacity(coefficients.len().saturating_add(2).div_euclid(3));
        for chunk in coefficients.chunks(3) {
            let low = match chunk.first() {
                Some(&value) => value,
                None => 0,
            };
            let middle = match chunk.get(1) {
                Some(&value) => value,
                None => 0,
            };
            let high = match chunk.get(2) {
                Some(&value) => value,
                None => 0,
            };
            limbs.push(
                low.saturating_add(middle.saturating_mul(SINGLE_CONV_BASE))
                    .saturating_add(
                        high.saturating_mul(SINGLE_CONV_BASE.saturating_mul(SINGLE_CONV_BASE)),
                    ),
            );
        }
        Self::from_limbs(limbs)
    }

    fn mul_ntt(&self, other: &Self) -> Option<Self> {
        if let Some(product) = self.mul_ntt_single_modulus(other) {
            return Some(product);
        }

        self.mul_ntt_dual_modulus(other)
    }

    fn mul_ntt_dual_modulus(&self, other: &Self) -> Option<Self> {
        let left = self.to_convolution_coefficients();
        let right = other.to_convolution_coefficients();
        if left.is_empty() {
            return Some(Self::zero());
        }
        if right.is_empty() {
            return Some(Self::zero());
        }

        let result_coefficients = left.len().checked_add(right.len())?.checked_sub(1)?;
        let transform_len = result_coefficients.checked_next_power_of_two()?;

        let transform_len_u64 = u64::try_from(transform_len).ok()?;
        if !NTT_MOD_1
            .saturating_sub(1)
            .is_multiple_of(transform_len_u64)
        {
            return None;
        }
        if !NTT_MOD_2
            .saturating_sub(1)
            .is_multiple_of(transform_len_u64)
        {
            return None;
        }

        let overlap = u128::try_from(left.len().min(right.len())).ok()?;
        let max_digit = u128::from(CONV_BASE.saturating_sub(1));
        let coefficient_bound = overlap.checked_mul(max_digit)?.checked_mul(max_digit)?;
        let modulus_product = u128::from(NTT_MOD_1).checked_mul(u128::from(NTT_MOD_2))?;
        if coefficient_bound >= modulus_product {
            return None;
        }

        let modulus_1 = MontgomeryModulus::new(NTT_MOD_1, NTT_MONT_INV_1, NTT_MONT_R2_1, NTT_ROOT);
        let modulus_2 = MontgomeryModulus::new(NTT_MOD_2, NTT_MONT_INV_2, NTT_MONT_R2_2, NTT_ROOT);
        let residues_1 = convolution_mod(&left, &right, transform_len, modulus_1)?;
        let residues_2 = convolution_mod(&left, &right, transform_len, modulus_2)?;

        let inverse_m1_mont = modulus_2.to_montgomery(NTT_INV_M1_MOD_M2);
        let mut coefficients = Vec::with_capacity(result_coefficients.saturating_add(4));
        let mut carry = 0_u128;

        for index in 0..result_coefficients {
            let r1 = *residues_1.get(index)?;
            let r2 = *residues_2.get(index)?;
            let delta = if r2 >= r1 {
                r2 - r1
            } else {
                r2 + NTT_MOD_2 - r1
            };
            let delta_mont = modulus_2.to_montgomery(delta);
            let t2 = modulus_2.decode_montgomery(modulus_2.multiply(delta_mont, inverse_m1_mont));
            let coefficient =
                u128::from(r1).checked_add(u128::from(NTT_MOD_1).checked_mul(u128::from(t2))?)?;

            let value = coefficient.checked_add(carry)?;
            coefficients.push((value % u128::from(CONV_BASE)) as u64);
            carry = value / u128::from(CONV_BASE);
        }

        append_radix_carry(&mut coefficients, carry, u128::from(CONV_BASE))?;

        Some(Self::from_convolution_coefficients(&coefficients))
    }

    fn to_convolution_coefficients(&self) -> Vec<u64> {
        let mut coefficients = Vec::with_capacity(self.limbs.len().saturating_mul(2));
        for &limb in &self.limbs {
            coefficients.push(limb % CONV_BASE);
            coefficients.push(limb / CONV_BASE);
        }
        trim_trailing_zeroes(&mut coefficients);
        coefficients
    }

    fn from_convolution_coefficients(coefficients: &[u64]) -> Self {
        if coefficients.is_empty() {
            return Self::zero();
        }

        let mut limbs = Vec::with_capacity(coefficients.len().saturating_add(1).div_euclid(2));
        for chunk in coefficients.chunks(2) {
            let low = match chunk.first() {
                Some(&value) => value,
                None => 0,
            };
            let high = match chunk.get(1) {
                Some(&value) => value,
                None => 0,
            };
            limbs.push(low.saturating_add(high.saturating_mul(CONV_BASE)));
        }
        Self::from_limbs(limbs)
    }

    fn mul_schoolbook(&self, other: &Self) -> Self {
        let capacity = self
            .limbs
            .len()
            .saturating_add(other.limbs.len())
            .saturating_add(1);
        let mut limbs = vec![0_u64; capacity];

        for (left_index, &left) in self.limbs.iter().enumerate() {
            let mut carry = 0_u128;
            for (right_index, &right) in other.limbs.iter().enumerate() {
                let index = left_index.saturating_add(right_index);
                let current =
                    u128::from(limbs[index]) + u128::from(left) * u128::from(right) + carry;
                limbs[index] = (current % BASE_WIDE) as u64;
                carry = current / BASE_WIDE;
            }

            let carry_start = left_index.saturating_add(other.limbs.len());
            for limb in limbs.iter_mut().skip(carry_start) {
                if carry == 0 {
                    break;
                }
                let current = u128::from(*limb) + carry;
                *limb = current.rem_euclid(BASE_WIDE) as u64;
                carry = current.div_euclid(BASE_WIDE);
            }
        }

        Self::from_limbs(limbs)
    }

    fn mul_karatsuba(&self, other: &Self) -> Option<Self> {
        let split = self.limbs.len().max(other.limbs.len()).div_euclid(2);
        if split == 0 {
            return None;
        }

        let left_split = split.min(self.limbs.len());
        let right_split = split.min(other.limbs.len());
        let left_low = Self::from_limbs(self.limbs.get(..left_split)?.to_vec());
        let left_high = Self::from_limbs(self.limbs.get(left_split..)?.to_vec());
        let right_low = Self::from_limbs(other.limbs.get(..right_split)?.to_vec());
        let right_high = Self::from_limbs(other.limbs.get(right_split..)?.to_vec());

        let z0 = left_low.mul(&right_low);
        let z2 = left_high.mul(&right_high);
        let sum_left = left_low.add(&left_high);
        let sum_right = right_low.add(&right_high);
        let combined = sum_left.mul(&sum_right);
        let without_z0 = combined.checked_sub(&z0)?;
        let z1 = without_z0.checked_sub(&z2)?;

        let mut result = z0;
        result.add_shifted(&z1, split);
        result.add_shifted(&z2, split.saturating_mul(2));
        Some(result)
    }

    fn add_shifted(&mut self, other: &Self, shift: usize) {
        if other.is_zero() {
            return;
        }

        let needed = shift.saturating_add(other.limbs.len()).saturating_add(1);
        self.limbs.resize(self.limbs.len().max(needed), 0);

        let mut carry = 0_u128;
        for (index, &added) in other.limbs.iter().enumerate() {
            let target = shift.saturating_add(index);
            let sum = u128::from(self.limbs[target]) + u128::from(added) + carry;
            self.limbs[target] = sum.rem_euclid(BASE_WIDE) as u64;
            carry = sum.div_euclid(BASE_WIDE);
        }

        let carry_start = shift.saturating_add(other.limbs.len());
        for limb in self.limbs.iter_mut().skip(carry_start) {
            if carry == 0 {
                break;
            }
            let sum = u128::from(*limb) + carry;
            *limb = sum.rem_euclid(BASE_WIDE) as u64;
            carry = sum.div_euclid(BASE_WIDE);
        }
        trim_trailing_zeroes(&mut self.limbs);
    }

    pub(super) fn mul_pow10(&self, decimal_places: usize) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        let whole_limbs = decimal_places.div_euclid(BASE_DIGITS);
        let remainder = decimal_places.rem_euclid(BASE_DIGITS);
        let scaled = self.mul_small(POW10[remainder]);
        if whole_limbs == 0 {
            return scaled;
        }

        let mut limbs = Vec::with_capacity(scaled.limbs.len().saturating_add(whole_limbs));
        limbs.resize(whole_limbs, 0);
        limbs.extend_from_slice(&scaled.limbs);
        Self::from_limbs(limbs)
    }

    pub(super) fn div_small(&self, divisor: u64) -> Option<(Self, u64)> {
        if divisor == 0 {
            return None;
        }
        if self.is_zero() {
            return Some((Self::zero(), 0));
        }

        let mut quotient = vec![0_u64; self.limbs.len()];
        let mut remainder = 0_u128;
        for index in (0..self.limbs.len()).rev() {
            let current = remainder * BASE_WIDE + u128::from(self.limbs[index]);
            quotient[index] = (current / u128::from(divisor)) as u64;
            remainder = current % u128::from(divisor);
        }

        Some((Self::from_limbs(quotient), remainder as u64))
    }

    pub(super) fn mod_small(&self, divisor: u64) -> Option<u64> {
        if divisor == 0 {
            return None;
        }

        let mut remainder = 0_u128;
        for &limb in self.limbs.iter().rev() {
            let current = remainder
                .saturating_mul(BASE_WIDE)
                .saturating_add(u128::from(limb));
            remainder = current.rem_euclid(u128::from(divisor));
        }
        Some(remainder as u64)
    }

    pub(super) fn div_pow10_floor(&self, decimal_places: usize) -> Option<Self> {
        if decimal_places == 0 {
            return Some(self.clone());
        }

        let whole_limbs = decimal_places / BASE_DIGITS;
        if whole_limbs >= self.limbs.len() {
            return Some(Self::zero());
        }

        let remainder = decimal_places % BASE_DIGITS;
        let truncated = Self::from_limbs(self.limbs.get(whole_limbs..)?.to_vec());
        if remainder == 0 {
            Some(truncated)
        } else {
            truncated
                .div_small(POW10[remainder])
                .map(|(value, _)| value)
        }
    }

    pub(super) fn to_u64(&self) -> Option<u64> {
        match self.limbs.as_slice() {
            [] => Some(0),
            [value] => Some(*value),
            _ => None,
        }
    }

    pub(super) fn write_scaled_decimal(
        &self,
        decimal_places: usize,
        output: &mut [u8],
    ) -> Option<usize> {
        let required = if decimal_places == 0 {
            1
        } else {
            decimal_places.checked_add(2)?
        };
        output.get(..required)?;
        if self.decimal_digits() != decimal_places.saturating_add(1) {
            return None;
        }

        let mut cursor = 0_usize;
        let mut digit_index = 0_usize;
        let top = *self.limbs.last()?;
        let mut top_digits = [0_u8; 20];
        let top_len = encode_unpadded(top, &mut top_digits)?;
        for &digit in top_digits.get(..top_len)? {
            emit_decimal_digit(digit, decimal_places, output, &mut cursor, &mut digit_index)?;
        }

        for index in (0..self.limbs.len().saturating_sub(1)).rev() {
            let mut digits = [b'0'; BASE_DIGITS];
            encode_fixed(self.limbs[index], &mut digits);
            for &digit in &digits {
                emit_decimal_digit(digit, decimal_places, output, &mut cursor, &mut digit_index)?;
            }
        }

        if cursor == required {
            Some(required)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct MontgomeryModulus {
    modulus: u64,
    negative_inverse: u64,
    r2: u64,
    primitive_root: u64,
}

impl MontgomeryModulus {
    const fn new(modulus: u64, negative_inverse: u64, r2: u64, primitive_root: u64) -> Self {
        Self {
            modulus,
            negative_inverse,
            r2,
            primitive_root,
        }
    }

    fn reduce(self, value: u128) -> u64 {
        let factor = (value as u64).wrapping_mul(self.negative_inverse);
        let adjusted = value + u128::from(factor) * u128::from(self.modulus);
        let mut reduced = (adjusted >> 64) as u64;
        if reduced >= self.modulus {
            reduced -= self.modulus;
        }
        reduced
    }

    fn to_montgomery(self, value: u64) -> u64 {
        self.reduce(u128::from(value) * u128::from(self.r2))
    }

    fn decode_montgomery(self, value: u64) -> u64 {
        self.reduce(u128::from(value))
    }

    fn multiply(self, left: u64, right: u64) -> u64 {
        self.reduce(u128::from(left) * u128::from(right))
    }

    fn one(self) -> u64 {
        self.to_montgomery(1)
    }

    fn pow(self, base: u64, mut exponent: u64) -> u64 {
        let mut result = self.one();
        let mut factor = self.to_montgomery(base);
        for _ in 0..u64::BITS {
            if exponent == 0 {
                break;
            }
            if exponent & 1 == 1 {
                result = self.multiply(result, factor);
            }
            exponent >>= 1;
            if exponent != 0 {
                factor = self.multiply(factor, factor);
            }
        }
        result
    }
}

fn convolution_square_mod(
    values: &[u64],
    transform_len: usize,
    modulus: MontgomeryModulus,
) -> Option<Vec<u64>> {
    if transform_len == 0 {
        return None;
    }
    if !transform_len.is_power_of_two() {
        return None;
    }
    let transform_len_u64 = u64::try_from(transform_len).ok()?;
    if !modulus
        .modulus
        .saturating_sub(1)
        .is_multiple_of(transform_len_u64)
    {
        return None;
    }

    let mut transformed = vec![0_u64; transform_len];
    for (index, &value) in values.iter().enumerate() {
        *transformed.get_mut(index)? = modulus.to_montgomery(value);
    }

    ntt(&mut transformed, false, modulus)?;
    for value in &mut transformed {
        *value = modulus.multiply(*value, *value);
    }
    ntt(&mut transformed, true, modulus)?;
    for value in &mut transformed {
        *value = modulus.decode_montgomery(*value);
    }
    Some(transformed)
}

fn convolution_mod(
    left: &[u64],
    right: &[u64],
    transform_len: usize,
    modulus: MontgomeryModulus,
) -> Option<Vec<u64>> {
    if transform_len == 0 {
        return None;
    }
    if !transform_len.is_power_of_two() {
        return None;
    }
    let transform_len_u64 = u64::try_from(transform_len).ok()?;
    if !modulus
        .modulus
        .saturating_sub(1)
        .is_multiple_of(transform_len_u64)
    {
        return None;
    }

    let mut left_values = vec![0_u64; transform_len];
    let mut right_values = vec![0_u64; transform_len];

    for (index, &value) in left.iter().enumerate() {
        *left_values.get_mut(index)? = modulus.to_montgomery(value);
    }
    for (index, &value) in right.iter().enumerate() {
        *right_values.get_mut(index)? = modulus.to_montgomery(value);
    }

    ntt(&mut left_values, false, modulus)?;
    ntt(&mut right_values, false, modulus)?;

    for index in 0..transform_len {
        let left_value = *left_values.get(index)?;
        let right_value = *right_values.get(index)?;
        *left_values.get_mut(index)? = modulus.multiply(left_value, right_value);
    }

    ntt(&mut left_values, true, modulus)?;
    for value in &mut left_values {
        *value = modulus.decode_montgomery(*value);
    }
    Some(left_values)
}

fn bit_reverse_permute(values: &mut [u64]) {
    let length = values.len();
    if length <= 1 {
        return;
    }

    let bit_width = length.trailing_zeros();
    let shift = usize::BITS.saturating_sub(bit_width);
    for index in 0..length {
        let reversed = index.reverse_bits() >> shift;
        if index < reversed {
            values.swap(index, reversed);
        }
    }
}

fn ntt(values: &mut [u64], invert: bool, modulus: MontgomeryModulus) -> Option<()> {
    let length = values.len();
    if length == 0 {
        return None;
    }
    if !length.is_power_of_two() {
        return None;
    }

    bit_reverse_permute(values);

    let group_order = modulus.modulus.saturating_sub(1);
    let stage_count = length.trailing_zeros();
    let mut twiddles = Vec::with_capacity(length.div_euclid(2));

    for stage_power in 1..=stage_count {
        let stage_len = 1_usize.checked_shl(stage_power)?;
        let stage_len_u64 = u64::try_from(stage_len).ok()?;
        if !group_order.is_multiple_of(stage_len_u64) {
            return None;
        }

        let exponent = group_order.checked_div(stage_len_u64)?;
        let root_exponent = if invert {
            group_order.checked_sub(exponent)?
        } else {
            exponent
        };
        let root = modulus.pow(modulus.primitive_root, root_exponent);

        let half = stage_len.div_euclid(2);
        twiddles.clear();
        twiddles.push(modulus.one());
        for _ in 1..half {
            let previous = *twiddles.last()?;
            twiddles.push(modulus.multiply(previous, root));
        }

        for block in (0..length).step_by(stage_len) {
            for offset in 0..half {
                let left_index = block.checked_add(offset)?;
                let right_index = left_index.checked_add(half)?;
                let left_value = *values.get(left_index)?;
                let factor = *twiddles.get(offset)?;
                let right_value = modulus.multiply(*values.get(right_index)?, factor);
                let modulus_minus_right = modulus.modulus - right_value;

                let sum = if left_value >= modulus_minus_right {
                    left_value - modulus_minus_right
                } else {
                    left_value + right_value
                };
                let difference = if left_value >= right_value {
                    left_value - right_value
                } else {
                    left_value + modulus.modulus - right_value
                };

                *values.get_mut(left_index)? = sum;
                *values.get_mut(right_index)? = difference;
            }
        }
    }

    if invert {
        let length_mod = u64::try_from(length).ok()?;
        let inverse_length = modulus.pow(length_mod, modulus.modulus.saturating_sub(2));
        for value in values {
            *value = modulus.multiply(*value, inverse_length);
        }
    }

    Some(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SignedNat {
    negative: bool,
    magnitude: Nat,
}

impl SignedNat {
    pub(super) fn from_magnitude(magnitude: Nat, negative: bool) -> Self {
        let negative = negative && !magnitude.is_zero();
        Self {
            negative,
            magnitude,
        }
    }

    pub(super) fn is_positive(&self) -> bool {
        !self.negative && !self.magnitude.is_zero()
    }

    pub(super) fn magnitude(&self) -> &Nat {
        &self.magnitude
    }

    pub(super) fn mul_nat(&self, value: &Nat) -> Self {
        Self::from_magnitude(self.magnitude.mul(value), self.negative)
    }

    pub(super) fn add(&self, other: &Self) -> Self {
        if self.negative == other.negative {
            return Self::from_magnitude(self.magnitude.add(&other.magnitude), self.negative);
        }

        match self.magnitude.cmp(&other.magnitude) {
            Ordering::Greater => match self.magnitude.checked_sub(&other.magnitude) {
                Some(value) => Self::from_magnitude(value, self.negative),
                None => Self::from_magnitude(Nat::zero(), false),
            },
            Ordering::Less => match other.magnitude.checked_sub(&self.magnitude) {
                Some(value) => Self::from_magnitude(value, other.negative),
                None => Self::from_magnitude(Nat::zero(), false),
            },
            Ordering::Equal => Self::from_magnitude(Nat::zero(), false),
        }
    }
}

fn encode_unpadded(mut value: u64, output: &mut [u8; 20]) -> Option<usize> {
    if value == 0 {
        output[0] = b'0';
        return Some(1);
    }

    let mut reversed = [0_u8; 20];
    let mut length = 0_usize;
    for _ in 0..reversed.len() {
        if value == 0 {
            break;
        }
        let slot = reversed.get_mut(length)?;
        *slot = b'0' + value.rem_euclid(10) as u8;
        value = value.div_euclid(10);
        length = length.checked_add(1)?;
    }
    if value != 0 {
        return None;
    }

    for index in 0..length {
        let source = length.checked_sub(index)?.checked_sub(1)?;
        *output.get_mut(index)? = *reversed.get(source)?;
    }
    Some(length)
}

fn encode_fixed(mut value: u64, output: &mut [u8; BASE_DIGITS]) {
    for index in (0..BASE_DIGITS).rev() {
        output[index] = b'0' + (value % 10) as u8;
        value /= 10;
    }
}

fn emit_decimal_digit(
    digit: u8,
    decimal_places: usize,
    output: &mut [u8],
    cursor: &mut usize,
    digit_index: &mut usize,
) -> Option<()> {
    if decimal_places != 0 && *digit_index == 1 {
        *output.get_mut(*cursor)? = b'.';
        *cursor = cursor.checked_add(1)?;
    }

    *output.get_mut(*cursor)? = digit;
    *cursor = cursor.checked_add(1)?;
    *digit_index = digit_index.checked_add(1)?;
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_scaling_round_trips() {
        let value = Nat::from_u64(3_141_592_653_589_793);
        let scaled = value.mul_pow10(37);
        assert_eq!(scaled.div_pow10_floor(37), Some(value));
    }

    #[test]
    fn karatsuba_matches_schoolbook() {
        let left = Nat::pow10(900).add_small(123_456_789);
        let right = Nat::pow10(850).add_small(987_654_321);
        assert_eq!(left.mul(&right), left.mul_schoolbook(&right));
    }

    #[test]
    fn ntt_matches_schoolbook() {
        let left = Nat::pow10(3_000).add_small(123_456_789);
        let right = Nat::pow10(2_900).add_small(987_654_321);
        assert_eq!(left.mul(&right), left.mul_schoolbook(&right));
    }

    #[test]
    fn montgomery_modular_multiplication_matches_reference() {
        let moduli = [
            MontgomeryModulus::new(NTT_MOD_1, NTT_MONT_INV_1, NTT_MONT_R2_1, NTT_ROOT),
            MontgomeryModulus::new(NTT_MOD_2, NTT_MONT_INV_2, NTT_MONT_R2_2, NTT_ROOT),
        ];

        for modulus in moduli {
            let samples = [
                0_u64,
                1,
                2,
                3,
                modulus.modulus / 2,
                modulus.modulus - 2,
                modulus.modulus - 1,
            ];
            for &value in &samples {
                assert_eq!(
                    modulus.decode_montgomery(modulus.to_montgomery(value)),
                    value
                );
            }
            for &left in &samples {
                for &right in &samples {
                    let reference = ((u128::from(left) * u128::from(right))
                        % u128::from(modulus.modulus)) as u64;
                    let actual = modulus.decode_montgomery(
                        modulus.multiply(modulus.to_montgomery(left), modulus.to_montgomery(right)),
                    );
                    assert_eq!(actual, reference);
                }
            }

            let mut state = 0x9e37_79b9_7f4a_7c15_u64;
            for _ in 0..10_000 {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let left = state % modulus.modulus;
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let right = state % modulus.modulus;
                let reference =
                    ((u128::from(left) * u128::from(right)) % u128::from(modulus.modulus)) as u64;
                let actual = modulus.decode_montgomery(
                    modulus.multiply(modulus.to_montgomery(left), modulus.to_montgomery(right)),
                );
                assert_eq!(actual, reference);
            }
        }
    }

    #[test]
    fn signed_addition_handles_opposite_signs() {
        let positive = SignedNat::from_magnitude(Nat::from_u64(100), false);
        let negative = SignedNat::from_magnitude(Nat::from_u64(40), true);
        assert_eq!(
            positive.add(&negative),
            SignedNat::from_magnitude(Nat::from_u64(60), false)
        );
    }

    #[test]
    fn scaled_decimal_writer_inserts_decimal_point() {
        let value = Nat::from_u64(314_159_265);
        let mut output = [0_u8; 10];
        assert_eq!(value.write_scaled_decimal(8, &mut output), Some(10));
        assert_eq!(&output, b"3.14159265");
    }

    #[test]
    fn arithmetic_edge_paths_are_explicit() {
        let zero = Nat::zero();
        assert_eq!(Nat::from_u128(0), zero);
        assert_eq!(zero.decimal_digits(), 1);
        assert_eq!(zero.add_small(0), zero);
        assert_eq!(zero.add_small(7), Nat::from_u64(7));

        let base = Nat::from_u128(BASE_WIDE);
        assert_eq!(base.decimal_digits(), 19);
        assert_eq!(Nat::from_limbs(vec![1, 0, 0]), Nat::from_u64(1));
        assert_eq!(Nat::from_u64(1).add(&base), base.add(&Nat::from_u64(1)));
        assert_eq!(
            Nat::from_u64(BASE - 1).add_small(1),
            Nat::from_u128(BASE_WIDE)
        );
        assert_eq!(Nat::from_u64(3).checked_sub(&Nat::from_u64(5)), None);
        assert_eq!(base.checked_sub_small(1), Some(Nat::from_u64(BASE - 1)));

        assert_eq!(Nat::from_u64(7).mul_small(0), Nat::zero());
        assert_eq!(
            Nat::from_u64(2).mul_small(BASE),
            Nat::from_u128(BASE_WIDE * 2)
        );
        assert_eq!(Nat::zero().mul(&Nat::from_u64(9)), Nat::zero());
        assert_eq!(Nat::zero().square(), Nat::zero());
        assert_eq!(
            Nat::zero().square_ntt_single_modulus_fast(),
            Some(Nat::zero())
        );
        assert_eq!(Nat::zero().square_ntt_single_modulus(), Some(Nat::zero()));
        assert_eq!(
            Nat::zero().mul_ntt_single_modulus_fast(&Nat::from_u64(1)),
            Some(Nat::zero())
        );
        assert_eq!(
            Nat::from_u64(1).mul_ntt_single_modulus_fast(&Nat::zero()),
            Some(Nat::zero())
        );
        assert_eq!(
            Nat::zero().mul_ntt_single_modulus_standard(&Nat::from_u64(1)),
            Some(Nat::zero())
        );
        assert_eq!(
            Nat::from_u64(1).mul_ntt_single_modulus_standard(&Nat::zero()),
            Some(Nat::zero())
        );
        assert_eq!(
            Nat::zero().mul_ntt_dual_modulus(&Nat::from_u64(1)),
            Some(Nat::zero())
        );
        assert_eq!(
            Nat::from_u64(1).mul_ntt_dual_modulus(&Nat::zero()),
            Some(Nat::zero())
        );
        assert_eq!(Nat::zero().mul_pow10(17), Nat::zero());
        assert_eq!(Nat::from_u64(12).mul_pow10(1), Nat::from_u64(120));

        assert_eq!(Nat::from_u64(7).div_small(0), None);
        assert_eq!(Nat::zero().div_small(7), Some((Nat::zero(), 0)));
        assert_eq!(
            Nat::from_u64(100).div_small(9),
            Some((Nat::from_u64(11), 1))
        );
        assert_eq!(Nat::from_u64(100).mod_small(0), None);
        assert_eq!(Nat::from_u64(100).mod_small(9), Some(1));

        assert_eq!(
            Nat::from_u64(123).div_pow10_floor(0),
            Some(Nat::from_u64(123))
        );
        assert_eq!(Nat::from_u64(123).div_pow10_floor(99), Some(Nat::zero()));
        assert_eq!(
            Nat::pow10(BASE_DIGITS).div_pow10_floor(BASE_DIGITS),
            Some(Nat::from_u64(1))
        );
        assert_eq!(Nat::zero().to_u64(), Some(0));
        assert_eq!(Nat::from_u64(7).to_u64(), Some(7));
        assert_eq!(base.to_u64(), None);

        let mut one = [0_u8; 1];
        assert_eq!(Nat::from_u64(3).write_scaled_decimal(0, &mut one), Some(1));
        assert_eq!(&one, b"3");
        let mut short = [0_u8; 3];
        assert_eq!(Nat::from_u64(314).write_scaled_decimal(2, &mut short), None);
        let mut wrong_scale = [0_u8; 4];
        assert_eq!(
            Nat::from_u64(31).write_scaled_decimal(2, &mut wrong_scale),
            None
        );

        let mut digits = [0_u8; 20];
        assert_eq!(encode_unpadded(0, &mut digits), Some(1));
        assert_eq!(digits[0], b'0');
        assert_eq!(encode_unpadded(12_345, &mut digits), Some(5));
        assert_eq!(&digits[..5], b"12345");

        let mut fixed = [0_u8; BASE_DIGITS];
        encode_fixed(42, &mut fixed);
        assert_eq!(&fixed[BASE_DIGITS - 2..], b"42");

        let mut output = [0_u8; 4];
        let mut cursor = 0_usize;
        let mut digit_index = 0_usize;
        assert_eq!(
            emit_decimal_digit(b'3', 2, &mut output, &mut cursor, &mut digit_index),
            Some(())
        );
        assert_eq!(
            emit_decimal_digit(b'1', 2, &mut output, &mut cursor, &mut digit_index),
            Some(())
        );
        assert_eq!(
            emit_decimal_digit(b'4', 2, &mut output, &mut cursor, &mut digit_index),
            Some(())
        );
        assert_eq!(&output, b"3.14");

        let positive = SignedNat::from_magnitude(Nat::from_u64(5), false);
        let positive_two = SignedNat::from_magnitude(Nat::from_u64(2), false);
        let negative = SignedNat::from_magnitude(Nat::from_u64(7), true);
        let equal_negative = SignedNat::from_magnitude(Nat::from_u64(5), true);
        assert_eq!(
            positive.add(&positive_two),
            SignedNat::from_magnitude(Nat::from_u64(7), false)
        );
        assert_eq!(
            positive.add(&negative),
            SignedNat::from_magnitude(Nat::from_u64(2), true)
        );
        assert_eq!(
            positive.add(&equal_negative),
            SignedNat::from_magnitude(Nat::zero(), false)
        );
        assert_eq!(
            SignedNat::from_magnitude(Nat::zero(), true),
            SignedNat::from_magnitude(Nat::zero(), false)
        );
    }

    #[test]
    fn ntt_backends_and_coefficient_round_trips_are_exact() {
        let mut left_limbs = vec![0_u64; NTT_THRESHOLD];
        let mut right_limbs = vec![0_u64; NTT_THRESHOLD];
        for index in 0..NTT_THRESHOLD {
            left_limbs[index] = (index as u64 + 1) * 1_000_003;
            right_limbs[index] = (index as u64 + 3) * 1_000_033;
        }
        let left = Nat::from_limbs(left_limbs);
        let right = Nat::from_limbs(right_limbs);
        let reference = left.mul_schoolbook(&right);

        let fast = left.mul_ntt_single_modulus_fast(&right);
        assert_eq!(fast, Some(reference.clone()));
        let standard = left.mul_ntt_single_modulus_standard(&right);
        assert_eq!(standard, Some(reference.clone()));
        let dual = left.mul_ntt_dual_modulus(&right);
        assert_eq!(dual, Some(reference.clone()));
        let adaptive = left.mul_ntt(&right);
        assert_eq!(adaptive, Some(reference.clone()));
        assert_eq!(left.mul(&right), reference);

        let square_reference = left.mul_schoolbook(&left);
        assert_eq!(
            left.square_ntt_single_modulus_fast(),
            Some(square_reference.clone())
        );
        assert_eq!(
            left.square_ntt_single_modulus(),
            Some(square_reference.clone())
        );
        assert_eq!(left.square(), square_reference);

        let fast_coefficients = left.to_fast_single_convolution_coefficients();
        assert!(fast_coefficients.is_some());
        if let Some(coefficients) = fast_coefficients {
            assert_eq!(
                Nat::from_fast_single_convolution_coefficients(&coefficients),
                Some(left.clone())
            );
        }
        assert_eq!(
            Nat::from_fast_single_convolution_coefficients(&[SINGLE_FAST_CONV_BASE]),
            None
        );
        assert_eq!(
            Nat::from_fast_single_convolution_coefficients(&[]),
            Some(Nat::zero())
        );

        let single_coefficients = left.to_single_convolution_coefficients();
        assert_eq!(
            Nat::from_single_convolution_coefficients(&single_coefficients),
            left
        );
        assert_eq!(
            Nat::from_single_convolution_coefficients(&[1, 2]),
            Nat::from_u128(1 + u128::from(2_u64) * u128::from(SINGLE_CONV_BASE))
        );
        assert_eq!(Nat::from_single_convolution_coefficients(&[]), Nat::zero());

        let dual_coefficients = right.to_convolution_coefficients();
        assert_eq!(
            Nat::from_convolution_coefficients(&dual_coefficients),
            right
        );
        assert_eq!(Nat::from_convolution_coefficients(&[1]), Nat::from_u64(1));
        assert_eq!(Nat::from_convolution_coefficients(&[]), Nat::zero());

        let modulus = MontgomeryModulus::new(NTT_MOD_1, NTT_MONT_INV_1, NTT_MONT_R2_1, NTT_ROOT);
        assert_eq!(modulus.decode_montgomery(modulus.one()), 1);
        assert_eq!(modulus.decode_montgomery(modulus.pow(7, 0)), 1);
        assert_eq!(
            modulus.decode_montgomery(modulus.pow(7, 5)),
            16_807 % modulus.modulus
        );

        let convolution = convolution_mod(&[1, 2, 3], &[4, 5], 8, modulus);
        assert!(convolution.is_some());
        if let Some(values) = convolution {
            assert_eq!(&values[..4], &[4, 13, 22, 15]);
        }

        let square = convolution_square_mod(&[1, 2, 3], 8, modulus);
        assert!(square.is_some());
        if let Some(values) = square {
            assert_eq!(&values[..5], &[1, 4, 10, 12, 9]);
        }

        assert_eq!(convolution_mod(&[1], &[1], 0, modulus), None);
        assert_eq!(convolution_mod(&[1], &[1], 3, modulus), None);
        assert_eq!(convolution_square_mod(&[1], 0, modulus), None);
        assert_eq!(convolution_square_mod(&[1], 3, modulus), None);

        let mut empty = [];
        assert_eq!(ntt(&mut empty, false, modulus), None);
        let mut invalid = [0_u64; 3];
        assert_eq!(ntt(&mut invalid, false, modulus), None);

        let mut permutation = [0_u64, 1, 2, 3, 4, 5, 6, 7];
        bit_reverse_permute(&mut permutation);
        assert_eq!(permutation, [0, 4, 2, 6, 1, 5, 3, 7]);
    }

    #[test]
    fn small_nat_arithmetic_matches_u128_reference() {
        let values = [
            0_u128,
            1,
            2,
            u128::from(BASE - 1),
            u128::from(BASE),
            u128::from(BASE) + 1,
            u128::from(BASE) * 2 - 1,
            u128::from(u64::MAX),
        ];
        let small_values = [0_u64, 1, 2, BASE - 1, BASE, BASE + 1, u64::MAX];

        for left in values {
            let left_nat = Nat::from_u128(left);
            for right in values {
                let right_nat = Nat::from_u128(right);
                assert_eq!(left_nat.add(&right_nat), Nat::from_u128(left + right));
                assert_eq!(
                    left_nat.checked_sub(&right_nat),
                    left.checked_sub(right).map(Nat::from_u128)
                );
                assert_eq!(left_nat.mul(&right_nat), Nat::from_u128(left * right));
            }

            for small in small_values {
                assert_eq!(
                    left_nat.add_small(small),
                    Nat::from_u128(left + u128::from(small))
                );
                assert_eq!(
                    left_nat.mul_small(small),
                    Nat::from_u128(left * u128::from(small))
                );
            }
        }

        assert_eq!(
            Nat::from_u64(1).partial_cmp(&Nat::from_u64(2)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Nat::from_u64(2).partial_cmp(&Nat::from_u64(2)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Nat::from_u64(3).partial_cmp(&Nat::from_u64(2)),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn multiplication_strategy_boundaries_are_explicit() {
        assert_eq!(multiplication_strategy(0, 0), MulStrategy::Schoolbook);
        assert_eq!(
            multiplication_strategy(KARATSUBA_THRESHOLD, KARATSUBA_THRESHOLD),
            MulStrategy::Schoolbook
        );
        assert_eq!(
            multiplication_strategy(KARATSUBA_THRESHOLD + 1, KARATSUBA_THRESHOLD + 1),
            MulStrategy::Karatsuba
        );
        assert_eq!(
            multiplication_strategy(
                KARATSUBA_THRESHOLD + 1,
                (KARATSUBA_THRESHOLD + 1).saturating_mul(KARATSUBA_UNBALANCED_FACTOR)
            ),
            MulStrategy::Karatsuba
        );
        assert_eq!(
            multiplication_strategy(
                KARATSUBA_THRESHOLD + 1,
                (KARATSUBA_THRESHOLD + 1)
                    .saturating_mul(KARATSUBA_UNBALANCED_FACTOR)
                    .saturating_add(1)
            ),
            MulStrategy::Schoolbook
        );
        assert_eq!(
            multiplication_strategy(NTT_THRESHOLD - 1, NTT_THRESHOLD - 1),
            MulStrategy::Karatsuba
        );
        assert_eq!(
            multiplication_strategy(NTT_THRESHOLD, NTT_THRESHOLD),
            MulStrategy::Ntt
        );
        assert!(!should_use_ntt_square(NTT_THRESHOLD - 1));
        assert!(should_use_ntt_square(NTT_THRESHOLD));
    }

    #[test]
    fn karatsuba_and_shifted_add_boundaries_are_exact() {
        let left = Nat::from_limbs(vec![BASE - 1; 64]);
        let mut right_limbs = vec![0_u64; 64];
        for (index, limb) in right_limbs.iter_mut().enumerate() {
            *limb = ((index as u64 + 1) * 1_000_003) % BASE;
        }
        let right = Nat::from_limbs(right_limbs);
        assert_eq!(
            left.mul_karatsuba(&right),
            Some(left.mul_schoolbook(&right))
        );
        assert_eq!(Nat::from_u64(1).mul_karatsuba(&Nat::from_u64(2)), None);

        let mut shifted = Nat::from_limbs(vec![BASE - 1, BASE - 1, 5]);
        shifted.add_shifted(&Nat::from_limbs(vec![1, 1]), 0);
        assert_eq!(shifted, Nat::from_limbs(vec![0, 1, 6]));

        let mut shifted = Nat::from_limbs(vec![7, BASE - 1, BASE - 1, 9]);
        shifted.add_shifted(&Nat::from_limbs(vec![1, 1]), 1);
        assert_eq!(shifted, Nat::from_limbs(vec![7, 0, 1, 10]));
    }

    #[test]
    fn dense_ntt_and_conversion_paths_match_references() {
        let left = Nat::from_limbs(vec![BASE - 1; 32]);
        let mut right_limbs = vec![0_u64; 32];
        for (index, limb) in right_limbs.iter_mut().enumerate() {
            *limb = BASE - 1 - (index as u64 * 97_003);
        }
        let right = Nat::from_limbs(right_limbs);
        let reference = left.mul_schoolbook(&right);

        assert_eq!(
            left.mul_ntt_single_modulus_fast(&right),
            Some(reference.clone())
        );
        assert_eq!(
            left.mul_ntt_single_modulus_standard(&right),
            Some(reference.clone())
        );
        assert_eq!(left.mul_ntt_single_modulus(&right), Some(reference.clone()));
        assert_eq!(left.mul_ntt_dual_modulus(&right), Some(reference.clone()));
        assert_eq!(left.mul_ntt(&right), Some(reference));

        let square_reference = left.mul_schoolbook(&left);
        assert_eq!(
            left.square_ntt_single_modulus_fast(),
            Some(square_reference.clone())
        );
        assert_eq!(left.square_ntt_single_modulus(), Some(square_reference));

        let samples = [
            Nat::from_limbs(vec![BASE - 1, BASE - 1, 1]),
            Nat::from_limbs(vec![1, 0, BASE - 1, 123_456_789_012_345_678]),
            Nat::from_limbs(vec![999_999_999_999_999_999, 1, 999_999_999_999_999_999]),
        ];
        for sample in samples {
            let fast = sample.to_fast_single_convolution_coefficients();
            assert!(fast.is_some());
            if let Some(coefficients) = fast {
                assert_eq!(
                    Nat::from_fast_single_convolution_coefficients(&coefficients),
                    Some(sample.clone())
                );
            }
            let single = sample.to_single_convolution_coefficients();
            assert_eq!(Nat::from_single_convolution_coefficients(&single), sample);
            let dual = sample.to_convolution_coefficients();
            assert_eq!(Nat::from_convolution_coefficients(&dual), sample);
        }

        let modulus = MontgomeryModulus::new(NTT_MOD_1, NTT_MONT_INV_1, NTT_MONT_R2_1, NTT_ROOT);
        for length in [1_usize, 2, 4, 8, 16, 32] {
            let mut values: Vec<u64> = (0..length)
                .map(|index| modulus.to_montgomery((index as u64 + 3) * 97_003))
                .collect();
            let original = values.clone();
            assert_eq!(ntt(&mut values, false, modulus), Some(()));
            assert_eq!(ntt(&mut values, true, modulus), Some(()));
            assert_eq!(values, original);
        }

        let mut carry_digits = Vec::new();
        let carry = BASE_WIDE
            .checked_mul(BASE_WIDE)
            .and_then(|value| value.checked_add(5));
        assert!(carry.is_some());
        if let Some(carry) = carry {
            assert_eq!(
                append_radix_carry(&mut carry_digits, carry, BASE_WIDE),
                Some(())
            );
            assert_eq!(carry_digits, vec![5, 0, 1]);
        }
        assert_eq!(append_radix_carry(&mut Vec::new(), 1, 0), None);
        assert_eq!(append_radix_carry(&mut Vec::new(), 1, 1), None);

        let mut binary = Vec::new();
        assert_eq!(append_radix_carry(&mut binary, u128::MAX, 2), Some(()));
        assert_eq!(binary.len(), u128::BITS as usize);
        assert!(binary.iter().all(|&digit| digit == 1));

        let mut zero_carry = vec![7_u64];
        assert_eq!(append_radix_carry(&mut zero_carry, 0, 2), Some(()));
        assert_eq!(zero_carry, vec![7]);
    }
}
