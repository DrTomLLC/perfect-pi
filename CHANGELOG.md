# Changelog

All notable released changes to Perfectπ will be documented here.

## Unreleased

- Universal 1.0 repository-scoped acceptance completed after exact-head CI, independent critic review, and merge of pull request #10; package publication remains separately gated.
- Universal 1.0 support contract, qualification guide, and executable requirement traceability added.
- Shared six-mode `RoundingMode` added for bounded and runtime π: toward zero, away from zero, toward negative infinity, toward positive infinity, nearest ties-to-even, and nearest ties-away-from-zero.
- Runtime generation now offers explicit caller precision ceilings while retaining caller-owned output storage and the no-direct-allocation source policy.
- Runtime independent verification expanded so all six rounding modes are checked against both Chudnovsky and Gauss–Legendre through 1,000 fractional digits at nine checkpoints.
- Runtime ASan fuzzing expanded across rounding policies, precision limits, buffer capacities, and deterministic regeneration.
- Aggregate `full` feature added to enable all current production capability tiers while preserving an empty default feature set.
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
- Optional `runtime-generation` and `arbitrary-precision` features added using exact-pinned `num-bigint 0.5.1` with default features disabled.
- Runtime π generation implemented with Machin's identity and conservative arbitrary-precision integer bounds; requested truncation is emitted only after lower/upper bounds converge.
- Runtime generation independently verified against both Chudnovsky and Gauss-Legendre through 1,000 fractional digits.
- Scientific/engineering, bounded-output, runtime-generation, host-benchmark, and linked-size examples added.
- Startup integrity self-test evaluated and intentionally left application-owned rather than adding an automatic library startup hook.
- Resource probes extended with writable `.data`/`.bss` and static instruction counts; all 18 constrained-target probes measure 0 bytes of `.data` and `.bss`.
- Current-nightly static stack-frame probes added for Cortex-M0, Cortex-M hardware-float, and bare-metal RISC-V.
- Reproducible stripped fat-LTO host linked-size measurements and five-run host timing medians published.
