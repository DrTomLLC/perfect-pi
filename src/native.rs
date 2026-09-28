//! Native floating-point π constants.
//!
//! These are direct aliases of Rust `core` constants. Perfectπ does not
//! manufacture extra decimal precision for a binary floating-point value.

/// π as IEEE-754 binary32.
pub const PI_F32: f32 = core::f32::consts::PI;
/// τ = 2π as IEEE-754 binary32.
pub const TAU_F32: f32 = core::f32::consts::TAU;
/// π/2 as IEEE-754 binary32.
pub const FRAC_PI_2_F32: f32 = core::f32::consts::FRAC_PI_2;
/// π/3 as IEEE-754 binary32.
pub const FRAC_PI_3_F32: f32 = core::f32::consts::FRAC_PI_3;
/// π/4 as IEEE-754 binary32.
pub const FRAC_PI_4_F32: f32 = core::f32::consts::FRAC_PI_4;
/// π/6 as IEEE-754 binary32.
pub const FRAC_PI_6_F32: f32 = core::f32::consts::FRAC_PI_6;
/// π/8 as IEEE-754 binary32.
pub const FRAC_PI_8_F32: f32 = core::f32::consts::FRAC_PI_8;
/// 1/π as IEEE-754 binary32.
pub const INV_PI_F32: f32 = core::f32::consts::FRAC_1_PI;
/// 2/π as IEEE-754 binary32.
pub const TWO_INV_PI_F32: f32 = core::f32::consts::FRAC_2_PI;
/// 2/√π as IEEE-754 binary32.
pub const TWO_INV_SQRT_PI_F32: f32 = core::f32::consts::FRAC_2_SQRT_PI;

/// π as IEEE-754 binary64.
pub const PI_F64: f64 = core::f64::consts::PI;
/// τ = 2π as IEEE-754 binary64.
pub const TAU_F64: f64 = core::f64::consts::TAU;
/// π/2 as IEEE-754 binary64.
pub const FRAC_PI_2_F64: f64 = core::f64::consts::FRAC_PI_2;
/// π/3 as IEEE-754 binary64.
pub const FRAC_PI_3_F64: f64 = core::f64::consts::FRAC_PI_3;
/// π/4 as IEEE-754 binary64.
pub const FRAC_PI_4_F64: f64 = core::f64::consts::FRAC_PI_4;
/// π/6 as IEEE-754 binary64.
pub const FRAC_PI_6_F64: f64 = core::f64::consts::FRAC_PI_6;
/// π/8 as IEEE-754 binary64.
pub const FRAC_PI_8_F64: f64 = core::f64::consts::FRAC_PI_8;
/// 1/π as IEEE-754 binary64.
pub const INV_PI_F64: f64 = core::f64::consts::FRAC_1_PI;
/// 2/π as IEEE-754 binary64.
pub const TWO_INV_PI_F64: f64 = core::f64::consts::FRAC_2_PI;
/// 2/√π as IEEE-754 binary64.
pub const TWO_INV_SQRT_PI_F64: f64 = core::f64::consts::FRAC_2_SQRT_PI;
