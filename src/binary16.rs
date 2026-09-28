//! Optional IEEE-754 binary16 π constants.
//!
//! Rust's native `f16` primitive is still unstable on the supported stable
//! toolchains. This module therefore exposes standards-level binary16 bit
//! patterns without requiring nightly Rust, software floating-point arithmetic,
//! allocation, or an external dependency.

/// Raw IEEE-754 binary16 value.
///
/// This is an interchange representation, not an arithmetic type.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Binary16(u16);

impl Binary16 {
    /// Creates a binary16 value from its IEEE-754 bit pattern.
    #[must_use]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// Returns the IEEE-754 bit pattern.
    #[must_use]
    pub const fn to_bits(self) -> u16 {
        self.0
    }

    /// Returns the big-endian byte representation.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    /// Returns the little-endian byte representation.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

/// π rounded to nearest, ties-to-even, in IEEE-754 binary16.
pub const PI_BINARY16: Binary16 = Binary16::from_bits(0x4248);
/// τ = 2π in IEEE-754 binary16.
pub const TAU_BINARY16: Binary16 = Binary16::from_bits(0x4648);
/// π/2 in IEEE-754 binary16.
pub const FRAC_PI_2_BINARY16: Binary16 = Binary16::from_bits(0x3e48);
/// π/3 in IEEE-754 binary16.
pub const FRAC_PI_3_BINARY16: Binary16 = Binary16::from_bits(0x3c30);
/// π/4 in IEEE-754 binary16.
pub const FRAC_PI_4_BINARY16: Binary16 = Binary16::from_bits(0x3a48);
/// π/6 in IEEE-754 binary16.
pub const FRAC_PI_6_BINARY16: Binary16 = Binary16::from_bits(0x3830);
/// π/8 in IEEE-754 binary16.
pub const FRAC_PI_8_BINARY16: Binary16 = Binary16::from_bits(0x3648);
/// 1/π in IEEE-754 binary16.
pub const INV_PI_BINARY16: Binary16 = Binary16::from_bits(0x3518);
/// 2/π in IEEE-754 binary16.
pub const TWO_INV_PI_BINARY16: Binary16 = Binary16::from_bits(0x3918);
/// 2/√π in IEEE-754 binary16.
pub const TWO_INV_SQRT_PI_BINARY16: Binary16 = Binary16::from_bits(0x3c83);
