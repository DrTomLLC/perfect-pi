#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![warn(missing_docs)]

//! Perfectπ — deterministic, bounded, resource-efficient π infrastructure.
//!
//! The initial core is intentionally small:
//! - native IEEE-754 `f32` and `f64` π constants;
//! - bounded decimal representations from 0 through 40 places;
//! - explicit truncation and round-to-nearest, ties-to-even semantics;
//! - no allocator, no I/O, no runtime π generation, and no `unsafe`.

mod bounded;
mod native;

pub use bounded::{BufferTooSmall, DecimalPi, MAX_DECIMAL_PLACES, Pi};
pub use native::{
    FRAC_PI_2_F32, FRAC_PI_2_F64, FRAC_PI_3_F32, FRAC_PI_3_F64, FRAC_PI_4_F32, FRAC_PI_4_F64,
    FRAC_PI_6_F32, FRAC_PI_6_F64, FRAC_PI_8_F32, FRAC_PI_8_F64, INV_PI_F32, INV_PI_F64, PI_F32,
    PI_F64, TAU_F32, TAU_F64, TWO_INV_PI_F32, TWO_INV_PI_F64, TWO_INV_SQRT_PI_F32,
    TWO_INV_SQRT_PI_F64,
};
