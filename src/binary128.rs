//! Optional IEEE-754 binary128 π constants.
//!
//! Rust's native `f128` primitive is still unstable on the supported stable
//! toolchains. This module therefore exposes standards-level binary128 bit
//! patterns without requiring nightly Rust, software floating-point arithmetic,
//! allocation, or an external dependency.

/// Raw IEEE-754 binary128 value.
///
/// This is an interchange representation, not an arithmetic type.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Binary128(u128);

impl Binary128 {
    /// Creates a binary128 value from its IEEE-754 bit pattern.
    #[must_use]
    pub const fn from_bits(bits: u128) -> Self {
        Self(bits)
    }

    /// Returns the IEEE-754 bit pattern.
    #[must_use]
    pub const fn to_bits(self) -> u128 {
        self.0
    }

    /// Returns the big-endian byte representation.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Returns the little-endian byte representation.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }
}

/// π rounded to nearest, ties-to-even, in IEEE-754 binary128.
pub const PI_BINARY128: Binary128 = Binary128::from_bits(0x4000_921f_b544_42d1_8469_898c_c517_01b8);
/// τ = 2π in IEEE-754 binary128.
pub const TAU_BINARY128: Binary128 =
    Binary128::from_bits(0x4001_921f_b544_42d1_8469_898c_c517_01b8);
/// π/2 in IEEE-754 binary128.
pub const FRAC_PI_2_BINARY128: Binary128 =
    Binary128::from_bits(0x3fff_921f_b544_42d1_8469_898c_c517_01b8);
/// π/3 in IEEE-754 binary128.
pub const FRAC_PI_3_BINARY128: Binary128 =
    Binary128::from_bits(0x3fff_0c15_2382_d736_5846_5bb3_2e0f_567b);
/// π/4 in IEEE-754 binary128.
pub const FRAC_PI_4_BINARY128: Binary128 =
    Binary128::from_bits(0x3ffe_921f_b544_42d1_8469_898c_c517_01b8);
/// π/6 in IEEE-754 binary128.
pub const FRAC_PI_6_BINARY128: Binary128 =
    Binary128::from_bits(0x3ffe_0c15_2382_d736_5846_5bb3_2e0f_567b);
/// π/8 in IEEE-754 binary128.
pub const FRAC_PI_8_BINARY128: Binary128 =
    Binary128::from_bits(0x3ffd_921f_b544_42d1_8469_898c_c517_01b8);
/// 1/π in IEEE-754 binary128.
pub const INV_PI_BINARY128: Binary128 =
    Binary128::from_bits(0x3ffd_45f3_06dc_9c88_2a53_f84e_afa3_ea6a);
/// 2/π in IEEE-754 binary128.
pub const TWO_INV_PI_BINARY128: Binary128 =
    Binary128::from_bits(0x3ffe_45f3_06dc_9c88_2a53_f84e_afa3_ea6a);
/// 2/√π in IEEE-754 binary128.
pub const TWO_INV_SQRT_PI_BINARY128: Binary128 =
    Binary128::from_bits(0x3fff_20dd_7504_29b6_d11a_e3a9_14fe_d7fe);
