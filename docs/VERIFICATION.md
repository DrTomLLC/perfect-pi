# Verification Strategy

Perfectπ must earn trust through reproducible evidence rather than branding claims.

## Current verified baseline

The initial core currently has:

- known-answer coverage for every bounded precision `D = 0..40`;
- explicit truncation and round-to-nearest-even vectors;
- exact known bit-pattern checks for native `f32` and `f64` π;
- allocation-free caller-buffer output tests;
- a compile-fail doctest proving bounded operations are unavailable for `Pi<41>`;
- independent standard-library Python Chudnovsky and Gauss–Legendre computations agreeing through 64 fractional digits, verifying all 41 production source digits and all 41 bounded precision vectors;
- successful bare-metal Cortex-M and RISC-V `no_std` builds;
- successful WASM and AArch64 builds;
- successful Rust 1.85.0 MSRV check, tests, and Clippy.

## Bounded-domain verification

The public bounded precision domain contains only 41 precision points: `Pi<0>` through `Pi<40>`. Each supported precision must have known-answer coverage for every defined rounding policy.

## Native representation verification

For supported native floating-point constants, tests should verify exact bit patterns where the Rust/platform contract makes that meaningful, alongside numerical error documentation.

## Independent references

Canonical π digits and derived constants should be checked against more than one independent source or generation method. A single library must not serve as both implementation and sole oracle.

## Required verification classes

- unit and known-answer tests;
- every bounded precision value;
- rounding and truncation boundaries;
- checked/lossy conversion boundaries;
- integer overflow and narrowing boundaries;
- cross-target builds;
- cross-target reproducibility checks;
- property/fuzz tests where inputs exist;
- mutation testing of critical logic;
- static analysis and lints;
- model checking or exhaustive-state techniques where they materially improve confidence.

## Verification isolation

Reference generators, arbitrary-precision packages, fuzzers, model checkers, and other heavy tools belong in development/verification dependency graphs only.

## Evidence retention

Release artifacts should record the compiler version, targets, feature sets, test results, benchmark methodology, and source revision used to support published claims.
