# Changelog

All notable released changes to Perfectπ will be documented here.

## Unreleased

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
