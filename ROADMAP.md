# Perfectπ Roadmap

## Phase 0 — Architecture baseline

- [x] define project mission;
- [x] establish public `0..40` bounded precision ceiling;
- [x] separate native, bounded, optional float, arbitrary, calculation, and verification layers;
- [x] define `no_std` / no-allocation / deterministic critical-core goals;
- [x] define precision and rounding terminology;
- [x] establish public repository and governance documentation.

## Phase 1 — Native core

- [ ] create minimal Rust workspace;
- [ ] establish MSRV policy;
- [ ] implement native `f32` / `f64` constants;
- [ ] implement verified derived constants;
- [ ] enforce `no_std`, no allocation, and `forbid(unsafe_code)`;
- [ ] add exact bit-pattern and known-answer tests.

## Phase 2 — Bounded precision

- [ ] implement `Pi<D>` for `0 <= D <= 40`;
- [ ] define stable compile-time bound enforcement;
- [ ] implement explicit rounding and truncation semantics;
- [ ] implement checked and explicitly lossy conversions;
- [ ] exhaustively test all supported `D` values.

## Phase 3 — Portability and optional adapters

- [ ] validate Cortex-M and RISC-V `no_std` builds;
- [ ] validate x86-64 and AArch64 builds;
- [ ] add optional `f16` support where appropriate;
- [ ] add optional `f128` support where appropriate;
- [ ] add optional fixed/decimal/complex interoperability without contaminating core dependencies.

## Phase 4 — Verification and resource evidence

- [ ] independent reference generation;
- [ ] fuzz/property testing;
- [ ] mutation testing;
- [ ] model-check suitable bounded invariants;
- [ ] cross-target reproducibility tests;
- [ ] measure ROM, RAM, stack, binary-size, instruction, and timing costs;
- [ ] publish benchmark and resource reports.

## Phase 5 — Extended ecosystem

- [ ] optional runtime π-generation algorithms;
- [ ] optional arbitrary precision above 40 places;
- [ ] scientific/engineering integration examples;
- [ ] evaluate optional startup integrity self-test.

## Phase 6 — First stable release

- [ ] freeze supported API;
- [ ] decide final licensing model and contributor-rights framework;
- [ ] complete independent review;
- [ ] publish crate only after tests and resource claims are reproducible.
