# Optional IEEE Float Formats

Perfectπ supports stable IEEE-754 binary16 and binary128 interchange formats without requiring Rust's unstable native `f16` / `f128` primitives.

Rust tracking issue: <https://github.com/rust-lang/rust/issues/116909>

## Design

The adapters are deliberately representation-only:

- `Binary16` is a transparent `u16` IEEE-754 binary16 bit wrapper;
- `Binary128` is a transparent `u128` IEEE-754 binary128 bit wrapper;
- neither type implements floating-point arithmetic;
- neither type allocates;
- neither type adds a dependency;
- neither type requires nightly Rust;
- both support exact bit access and big-/little-endian byte extraction.

This preserves Perfectπ's current-stable contract while allowing callers to move exact IEEE values into hardware registers, file formats, wire formats, foreign-function boundaries, or external numeric libraries.

## Features

No optional float format is enabled by default.

```text
binary16
binary128
all-float-formats = binary16 + binary128
```

The formats can be enabled independently.

## Constants

Each optional format exposes the same verified π-family set:

- π;
- τ = 2π;
- π/2;
- π/3;
- π/4;
- π/6;
- π/8;
- 1/π;
- 2/π;
- 2/√π.

The primary π encodings are:

```text
PI_BINARY16  = 0x4248
PI_BINARY128 = 0x4000921fb54442d18469898cc51701b8
```

All constants use IEEE round-to-nearest, ties-to-even semantics.

## Verification

The reference generator computes the constant family twice using independent π algorithms:

1. Chudnovsky;
2. Gauss-Legendre.

Each result is converted independently to IEEE binary16 and binary128 using exact rational rounding logic. The source bit patterns must agree with both computations.

The optional formats are also tested under:

- current stable Rust 1.99.0;
- current nightly Rust (currently 1.101.0-nightly), including direct comparison with native nightly `f16` / `f128` constants;
- Linux;
- Windows;
- macOS;
- Cortex-M0;
- Cortex-M hardware-float;
- bare-metal RISC-V;
- WebAssembly;
- AArch64.

## Native Rust `f16` / `f128`

Perfectπ intentionally does not expose `PI_F16` or `PI_F128` today.

Those names are reserved for a native adapter once the primitive types reach current stable Rust and satisfy Perfectπ's portability requirements. Current nightly is already used to verify that its native `f16` / `f128` π-family encodings match Perfectπ's stable IEEE contracts. The existing `Binary16` / `Binary128` bit contracts will remain useful for serialization and cross-language interchange regardless of future native support.
