#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![warn(missing_docs)]

#[cfg(feature = "runtime-generation")]
extern crate alloc;

//! Perfectπ — deterministic, bounded, resource-efficient π infrastructure.
//!
//! The initial core is intentionally small:
//! - native IEEE-754 `f32` and `f64` π constants;
//! - bounded decimal representations from 0 through 40 places;
//! - explicit truncation and round-to-nearest, ties-to-even semantics;
//! - checked precision-preserving and explicitly lossy native-float conversions;
//! - optional dependency-free IEEE binary16 / binary128 interchange formats;
//! - no allocator, no I/O, no runtime π generation, and no `unsafe` in the default bounded core;
//! - optional runtime/arbitrary-precision generation remains feature-gated and variable-cost.

#[cfg(feature = "binary128")]
mod binary128;
#[cfg(feature = "binary16")]
mod binary16;
mod bounded;
mod conversion;
#[cfg(feature = "complex")]
mod interop_complex;
#[cfg(feature = "decimal")]
mod interop_decimal;
#[cfg(feature = "fixed-point")]
mod interop_fixed;
mod native;
mod rounding;
#[cfg(feature = "runtime-generation")]
mod runtime;

#[cfg(feature = "binary16")]
pub use binary16::{
    Binary16, FRAC_PI_2_BINARY16, FRAC_PI_3_BINARY16, FRAC_PI_4_BINARY16, FRAC_PI_6_BINARY16,
    FRAC_PI_8_BINARY16, INV_PI_BINARY16, PI_BINARY16, TAU_BINARY16, TWO_INV_PI_BINARY16,
    TWO_INV_SQRT_PI_BINARY16,
};
#[cfg(feature = "binary128")]
pub use binary128::{
    Binary128, FRAC_PI_2_BINARY128, FRAC_PI_3_BINARY128, FRAC_PI_4_BINARY128, FRAC_PI_6_BINARY128,
    FRAC_PI_8_BINARY128, INV_PI_BINARY128, PI_BINARY128, TAU_BINARY128, TWO_INV_PI_BINARY128,
    TWO_INV_SQRT_PI_BINARY128,
};
pub use bounded::{BufferTooSmall, DecimalPi, MAX_DECIMAL_PLACES, Pi};
pub use conversion::{F32_GUARANTEED_DECIMAL_PLACES, F64_GUARANTEED_DECIMAL_PLACES, PrecisionLoss};
#[cfg(feature = "decimal")]
pub use interop_decimal::{RUST_DECIMAL_MAX_PLACES, RustDecimalInteropError};
#[cfg(feature = "fixed-point")]
pub use interop_fixed::FixedInteropError;
pub use rounding::RoundingMode;
pub use native::{
    FRAC_PI_2_F32, FRAC_PI_2_F64, FRAC_PI_3_F32, FRAC_PI_3_F64, FRAC_PI_4_F32, FRAC_PI_4_F64,
    FRAC_PI_6_F32, FRAC_PI_6_F64, FRAC_PI_8_F32, FRAC_PI_8_F64, INV_PI_F32, INV_PI_F64, PI_F32,
    PI_F64, TAU_F32, TAU_F64, TWO_INV_PI_F32, TWO_INV_PI_F64, TWO_INV_SQRT_PI_F32,
    TWO_INV_SQRT_PI_F64,
};
#[cfg(feature = "runtime-generation")]
pub use runtime::{
    RuntimePiError, generate_pi_ascii, generate_pi_ascii_round_nearest_even,
    generate_pi_ascii_with_limit, generate_pi_ascii_with_rounding, generate_pi_string,
    runtime_pi_ascii_len,
};
