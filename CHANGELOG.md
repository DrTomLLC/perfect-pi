# Changelog

All notable released changes to Perfectπ will be documented here.

## Unreleased

- Repository and architecture baseline established.
- Public brand defined as Perfectπ / Perfect Pi.
- Bounded precision design fixed at 0 through 40 decimal places after the decimal point.
- Critical-core reliability and resource goals documented.
- Initial Rust 2024 / MSRV 1.85 `no_std` core implemented.
- Native `f32` / `f64` π and related constants implemented as direct `core` aliases.
- Bounded `Pi<D>` implemented for every `D = 0..=40`.
- Explicit truncation and round-to-nearest-even implemented.
- Allocation-free caller-buffer ASCII output implemented.
- All 41 bounded precision vectors independently verified.
- Chudnovsky and Gauss–Legendre verification agree through 64 fractional digits.
- Bare-metal Cortex-M and RISC-V, WASM, AArch64, Windows host, and MSRV checks established.
- Initial object-level resource measurements published.
