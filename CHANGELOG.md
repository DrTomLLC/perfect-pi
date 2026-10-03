# Changelog

All notable released changes to Perfectπ will be documented here.

## Unreleased

- Minimum supported Rust advanced to current stable Rust 1.99.0; nightly forward-compatibility validation now uses Rust 1.101.0-nightly (2026-10-01).
- Universal 1.0 repository-scoped acceptance completed after exact-head CI, independent critic review, and merge of pull request #10; package publication remains separately gated.
- Universal 1.0 support contract, qualification guide, and executable requirement traceability added.
- Shared six-mode `RoundingMode` added for bounded and runtime π: toward zero, away from zero, toward negative infinity, toward positive infinity, nearest ties-to-even, and nearest ties-away-from-zero.
- Runtime generation now offers explicit caller precision ceilings while retaining caller-owned output storage and the no-direct-allocation source policy.
- Runtime independent verification expanded so all six rounding modes are checked against both Chudnovsky and Gauss–Legendre through 36,808 fractional digits at fourteen checkpoints, explicitly covering the 10,000/10,001 fast-path handoff and reciprocal-division regression points at 36,000, 36,700, and 36,808 places.
- An independent Astra Max audit exposed precision-dependent reciprocal-division certification failures in the 36k-place range. Quotient division now performs one bounded same-scale Newton refinement only when exact certification exceeds its adjustment budget; regression hashes and independent dual-algorithm verification cover the failing boundaries.
- Runtime ASan fuzzing now makes the caller-limit rejection selector reachable and reserves structured cases for 10,001, 36,000, 36,700, and 36,808 places so the specialized arithmetic backend is exercised in addition to the inexpensive prefix/API paths.
- Specialized runtime arithmetic hardened after exhaustive mutation review: progress-sensitive loops were replaced with bounded forms, arithmetic/NTT boundary tests were expanded, and the Astra-corrected full-repository campaign now reports 580 generated mutants after two documented equivalent exclusions, with 510 caught, 70 compiler-unviable, 0 missed, and 0 timed out.
- Corrected-tree all-feature coverage now measures 96.06% lines, 92.31% regions, and 100% functions; coverage-only builds disable forced inlining for four private conversion lookup helpers to prevent LLVM cross-binary profile-hash mismatches while normal production object bytes remain unchanged; the Linux/WSL AddressSanitizer campaign completed 60,064 fuzz executions (25,000 bounded API, 25,000 interoperability, 10,000 general runtime, and 64 seeded deep-arithmetic runs) with no crash or sanitizer finding.
- Final NTT hot-path review restored invariant-backed plain modular arithmetic while retaining bounded control flow. The retained pre-Astra-repair Windows measurements were 116.948 ms serial / 77.504 ms parallel at 100k places and 1.798 s serial / 1.131 s parallel at 1M; the reciprocal-division repair preserves the normal hot path via a cold retry, but final competitor timings are intentionally deferred until the post-repair Astra re-audit.
- Aggregate `full` feature added to enable all numerical capability tiers while preserving an empty default feature set; host-threaded `parallel-runtime` remains a separate explicit performance policy.
- Portable-core CI expanded across embedded ARM/RISC-V, 32/64-bit, little/big-endian, Linux/musl, Windows, Android, Apple, WebAssembly/WASI, BSD/illumos, s390x, PowerPC, and LoongArch representative targets.
- Public error types now implement `core::error::Error` where their contained dependency errors permit it.

- Repository and architecture baseline established.
- Public brand defined as Perfectπ / Perfect Pi.
- Bounded precision design fixed at 0 through 40 decimal places after the decimal point.
- Critical-core reliability and resource goals documented.
- Initial Rust 2024 `no_std` core implemented; minimum supported Rust raised to current stable Rust 1.98.1.
- Native `f32` / `f64` π and related constants implemented as direct `core` aliases.
- Bounded `Pi<D>` implemented for every `D = 0..=40`.
- Explicit truncation and round-to-nearest-even implemented.
- Allocation-free caller-buffer ASCII output implemented.
- All 41 bounded precision vectors independently verified.
- Chudnovsky and Gauss–Legendre verification agree through 64 fractional digits.
- Bare-metal Cortex-M and RISC-V, WASM, AArch64, Windows host, current-stable, and current-nightly checks established.
- Initial object-level resource measurements published.
- Checked decimal-place-preserving `f32` / `f64` conversions implemented with verified guarantees through `D=6` and `D=15`, respectively.
- Explicitly lossy `to_f32_lossy()` / `to_f64_lossy()` conversions implemented for the full bounded domain.
- All 164 bounded native-conversion outcomes independently verified against exact-rational IEEE-754 reference bits.
- Conversion implementation reduced to compact verified bit tables/native-π collapse paths and measured on Cortex-M and RISC-V.
- Decimal-place preservation guarantees fixed at `D<=6` for `f32` and `D<=15` for `f64`, with first failing cases verified at `D=7` and `D=16`.
- Conversion resource probes added for high-precision/native-collapse and low-precision table-backed paths.
- Optional dependency-free IEEE-754 `binary16` and `binary128` interchange adapters added.
- Ten π-family constants independently verified for each optional format using Chudnovsky and Gauss–Legendre references.
- Optional float formats verified under current stable Rust 1.98.1, current nightly, and the full host/`no_std` target matrix.
- Resource probes added for optional binary16 and binary128 paths.
- Optional complex interoperability added using current stable `num-complex 0.4.6`.
- Optional fixed-point interoperability added using current stable `fixed 1.31.0`, with nearest-even decimal parsing and no binary-float detour.
- Optional decimal interoperability added using current stable `rust_decimal 1.43.0`, exact through 28 places with explicit nearest-even rounding above that limit.
- Direct dependency currency/isolation CI added; default Perfectπ remains dependency-free.
- All-feature interoperability verified on the full `no_std` cross-target matrix.
- Phase-4 exhaustive bounded-state verification added, covering 3,690 caller-buffer states.
- Sanitizer-backed libFuzzer targets added for bounded-core and interoperability surfaces.
- Post-Phase-5 mutation testing established with 170 generated mutants: 158 caught, 12 compiler-unviable, 0 missed, and 0 timed out.
- Post-Phase-5 all-feature source coverage measured at 95.23% lines, 91.80% regions, and 100% functions with required CI floors unchanged at 92% / 91% / 100%.
- Cross-host reproducibility established for 30 representative objects across Windows and Linux under the same Rust compiler commit.
- Verification-tool currency is now enforced for cargo-audit, cargo-fuzz, cargo-mutants, cargo-llvm-cov, and libfuzzer-sys.
- Required RustSec dependency auditing added; the current all-feature dependency set has no known advisories.
- Optional `runtime-generation` and `arbitrary-precision` now use Perfectπ's internal specialized arbitrary-precision integer engine; no normal bigint crate dependency is required. `num-bigint 0.5.1` remains development-only as an independent arithmetic oracle.
- Runtime π generation uses certified Chudnovsky binary splitting over base-`10^18` limbs with measured schoolbook/Karatsuba/NTT multiplication tiers, reciprocal-based exact quotient/square-root certification, and caller-owned final output. The former Machin implementation remains test-only as an algorithmically independent cross-check.
- Optional `parallel-runtime` adds dependency-free scoped host parallelism at and above the measured 512-term crossover; `full` intentionally leaves threading policy opt-in.
- Runtime generation independently verified against both Chudnovsky and Gauss-Legendre through 10,001 fractional digits at eleven checkpoints, with independent complete-output digest agreement against Dashu at 100,000 and 1,000,000 places during the comparative benchmark campaign.
- Scientific/engineering, bounded-output, runtime-generation, host-benchmark, and linked-size examples added.
- Startup integrity self-test evaluated and intentionally left application-owned rather than adding an automatic library startup hook.
- Resource probes extended with writable `.data`/`.bss` and static instruction counts; all 18 constrained-target probes measure 0 bytes of `.data` and `.bss`.
- Current-nightly static stack-frame probes added for Cortex-M0, Cortex-M hardware-float, and bare-metal RISC-V.
- Reproducible stripped fat-LTO host linked-size measurements and five-run host timing medians published.
