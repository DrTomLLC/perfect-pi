# Optional Interoperability

Perfectπ keeps the default bounded core dependency-free. Domain-library interoperability is opt-in and uses the newest stable release of each direct dependency under the project's [Rust and Dependency Currency Policy](RUST_POLICY.md).

## Features

| Feature | Current direct dependency | Purpose |
| --- | --- | --- |
| `complex` | `num-complex 0.4.6` | Complex values with zero imaginary component |
| `fixed-point` | `fixed 1.31.0` | Generic fixed-point conversion using decimal nearest-even parsing |
| `decimal` | `rust_decimal 1.43.0` | Exact decimal conversion through 28 places and explicit rounding above |
| `interop` | all three | Convenience feature enabling all interoperability adapters |

All direct integration dependencies are:

- optional;
- configured with default features disabled;
- absent from the default Perfectπ dependency graph;
- checked against crates.io for newest-stable currency in required CI.

## Complex

The complex adapter preserves the existing native-float semantics rather than inventing a new precision model.

Available methods:

```rust
to_complex32_lossy()
to_complex64_lossy()
try_to_complex32_preserving_places()
try_to_complex64_preserving_places()
```

The real component is produced by Perfectπ's already-verified `f32` / `f64` conversion path and the imaginary component is exactly zero.

Therefore:

- checked complex binary32 preserves requested decimal places through `D <= 6`;
- checked complex binary64 preserves requested decimal places through `D <= 15`;
- higher-precision sources must explicitly choose the lossy method.

## Fixed point

The fixed-point adapter does not route through binary floating point.

```rust
to_fixed_nearest_even::<F>()
```

Perfectπ writes the finite source decimal into a fixed-size caller-owned stack buffer and passes those exact ASCII digits to `fixed`'s decimal parser. The current `fixed` parser specifies round-to-nearest, ties-to-even behavior.

The destination type controls:

- signed/unsigned range;
- integer width;
- fractional-bit width.

If the destination cannot represent the value, the adapter returns `FixedInteropError`.

Example verified reference values for rounded `Pi<40>`:

```text
I16F16 bits = 205887
I32F32 bits = 13493037705
```

These reference integers are independently regenerated from exact rational arithmetic in `scripts/verify_interop.py`.

## rust_decimal

`rust_decimal::Decimal` supports at most 28 decimal places. Perfectπ exposes two different contracts.

Exact conversion:

```rust
try_to_rust_decimal_exact()
```

This succeeds only for `D <= 28`. For `D > 28`, it returns `RustDecimalInteropError::TooManyDecimalPlaces` rather than silently discarding digits.

Explicit nearest-even conversion:

```rust
to_rust_decimal_nearest_even()
```

For `D <= 28`, this is exact. For wider bounded values, Perfectπ rounds the finite stored decimal value to 28 places using round-to-nearest, ties-to-even and then uses `rust_decimal`'s checked coefficient/scale constructor.

No binary floating-point intermediate is used.

Verified examples:

```text
Pi<28> truncated exact:
3.1415926535897932384626433832

Pi<40> rounded -> rust_decimal nearest-even:
3.1415926535897932384626433833
```

## Resource and dependency boundary

Enabling interoperability is a deliberate trade:

- default Perfectπ remains dependency-free;
- `complex`, `fixed-point`, and `decimal` pull their respective ecosystems only when requested;
- all-feature `no_std` builds currently pass on Cortex-M0, Cortex-M hardware-float, bare-metal RISC-V, WebAssembly, and AArch64;
- exact linked-code and stack costs for each interoperability path remain separate Phase 4 measurement work.

Interop features must never become dependencies of the smallest default path.
