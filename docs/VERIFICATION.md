# Verification Strategy

Perfectπ must earn trust through reproducible evidence rather than branding claims.

## Current verified baseline

The initial core currently has:

- known-answer coverage for every bounded precision `D = 0..40`;
- explicit truncation and round-to-nearest-even vectors;
- exact known bit-pattern checks for native `f32` and `f64` π;
- exact-rational generation and IEEE-bit verification of all 164 bounded native-conversion outcomes (41 precisions × truncation/rounding × `f32`/`f64`);
- independent proof of the checked preservation boundaries through `D=6` for `f32` and `D=15` for `f64`, including concrete failing cases at `D=7` and `D=16`;
- allocation-free caller-buffer output tests;
- a compile-fail doctest proving bounded operations are unavailable for `Pi<41>`;
- independent standard-library Python Chudnovsky and Gauss–Legendre computations agreeing through 64 fractional digits, verifying all 41 production source digits and all 41 bounded precision vectors;
- successful bare-metal Cortex-M and RISC-V `no_std` builds;
- successful WASM and AArch64 builds;
- successful current-stable Rust 1.98.1 check, tests, and Clippy, with no support target below current stable;
- dual-algorithm independent verification of all 10 binary16 and all 10 binary128 optional constants;
- all-feature builds and tests across the supported host and `no_std` target matrix;
- independent exact-rational reference checks for fixed-point interop and rust_decimal exact/28-place nearest-even results;
- required crates.io currency checks for all direct optional interoperability dependencies and verification tools;
- RustSec advisory auditing of the resolved optional dependency graph;
- exhaustive bounded-state caller-buffer verification across 3,690 legal precision/mode/capacity states;
- sanitizer-backed libFuzzer targets for the bounded core, optional interoperability surfaces, and runtime arbitrary-precision generation;
- independent truncation and nearest-even runtime-generation checks against both Chudnovsky and Gauss-Legendre through 1,000 fractional digits at nine precision checkpoints;
- mutation testing with zero surviving viable mutants required by CI; retained exact counts are updated whenever the production tree changes;
- measured post-Phase-5 all-feature source coverage of 95.23% lines, 91.80% regions, and 100% functions, with enforced CI floors of 92% / 91% / 100%;
- byte-for-byte reproducibility of 30 representative probe objects across repeated Windows and Linux builds using the same Rust 1.98.1 compiler commit.

Detailed methodology and retained evidence are documented in [Phase 4 Verification Hardening](PHASE4_VERIFICATION.md).

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

Reference generators, fuzzers, model checkers, and other verification-only tools belong outside production dependency graphs. The one production arbitrary-precision dependency, `num-bigint`, is permitted only behind the explicit `runtime-generation`/`arbitrary-precision` feature boundary; it remains absent from the default bounded core.

## Evidence retention

Release artifacts should record the compiler version, targets, feature sets, test results, benchmark methodology, and source revision used to support published claims.
