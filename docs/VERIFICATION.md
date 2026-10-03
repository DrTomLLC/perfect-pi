# Verification Strategy

Perfectπ must earn trust through reproducible evidence rather than branding claims.

## Current verified baseline

The initial core currently has:

- known-answer coverage for every bounded precision `D = 0..40`;
- explicit truncation and round-to-nearest-even vectors;
- exhaustive bounded equivalence checks for all six public `RoundingMode` policies at every `D = 0..40`;
- exact known bit-pattern checks for native `f32` and `f64` π;
- exact-rational generation and IEEE-bit verification of all 246 bounded native-conversion outcomes (41 precisions × truncation/nearest/ceiling × `f32`/`f64`);
- independent proof of the checked preservation boundaries through `D=6` for `f32` and `D=15` for `f64`, including concrete failing cases at `D=7` and `D=16`;
- allocation-free caller-buffer output tests;
- a compile-fail doctest proving bounded operations are unavailable for `Pi<41>`;
- independent standard-library Python Chudnovsky and Gauss–Legendre computations agreeing through 64 fractional digits, verifying all 41 production source digits and all 41 bounded precision vectors;
- successful bare-metal Cortex-M and RISC-V `no_std` builds;
- successful WASM and AArch64 builds;
- successful current-stable Rust 1.99.0 check, tests, and Clippy, with no support target below current stable;
- dual-algorithm independent verification of all 10 binary16 and all 10 binary128 optional constants;
- all-feature builds and tests across the supported host and `no_std` target matrix;
- independent exact-rational reference checks for fixed-point interop and rust_decimal exact/28-place nearest-even results;
- required crates.io currency checks for all direct optional interoperability dependencies and verification tools;
- RustSec advisory auditing of the resolved optional dependency graph;
- exhaustive bounded-state caller-buffer verification across 3,690 legal precision/mode/capacity states;
- sanitizer-backed libFuzzer targets for the bounded core, optional interoperability surfaces, and runtime arbitrary-precision generation;
- independent checks of all six runtime rounding policies against both Chudnovsky and Gauss-Legendre through 36,808 fractional digits at fourteen precision checkpoints, explicitly covering the 10,000/10,001 fast-path handoff and reciprocal-division regression points at 36,000, 36,700, and 36,808 places, plus production Chudnovsky cross-checks against the retained Machin reference through 1,000 places;
- runtime precision-limit tests proving rejection occurs before output mutation, with sanitizer-backed fuzzing using a reachable limit selector and structured deep-precision cases that cross into the specialized arithmetic engine;
- a 29-target dependency-free core portability matrix spanning embedded, word-size, endian, OS, mobile, WebAssembly, and architecture classes;
- mutation testing with zero surviving viable mutants required by CI; the current corrected-tree local pass generated 580 mutants after two documented mathematically equivalent exclusions, with 510 caught, 70 compiler-unviable, 0 missed, and 0 timed out;
- measured corrected-tree all-feature source coverage of 96.06% lines, 92.31% regions, and 100% functions, with enforced CI floors of 92% / 91% / 100%;
- byte-for-byte reproducibility of 30 representative probe objects across repeated Windows and Linux builds using Rust 1.99.0 commit `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`; required host CI also checks the same golden manifest on macOS.

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
- cross-target builds, including dependency-free core portability and selected all-feature targets;
- cross-target reproducibility checks;
- property/fuzz tests where inputs exist;
- mutation testing of critical logic;
- static analysis and lints;
- model checking or exhaustive-state techniques where they materially improve confidence.

## Verification isolation

Reference generators, fuzzers, model checkers, and other verification-only tools belong outside production dependency graphs. Runtime arbitrary precision is now implemented by Perfectπ's internal specialized integer engine and introduces no normal bigint dependency. `num-bigint` remains development-only as an independent arithmetic oracle and never enters the production dependency graph.

## Evidence retention

Release artifacts should record the compiler version, targets, feature sets, test results, benchmark methodology, and source revision used to support published claims.
