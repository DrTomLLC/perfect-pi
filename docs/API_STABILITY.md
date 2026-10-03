# API Stability and Compatibility

## Frozen technical baseline

The supported public API documented in the normative specification is frozen for the first stable-release candidate. Changes that alter names, precision semantics, rounding/truncation behavior, conversion guarantees, error contracts, serialization behavior, feature meanings, or the bounded `0..=40` domain require deliberate specification review.

## Stable public surfaces

The baseline includes:

- native `f32` / `f64` π-family constants;
- `Pi<D>` and `DecimalPi<D>` for `D = 0..=40`;
- allocation-free bounded ASCII output and `BufferTooSmall`;
- checked and explicitly lossy `f32` / `f64` conversion APIs;
- optional `Binary16` and `Binary128` interchange types and constants;
- optional complex, fixed-point, and `rust_decimal` interoperability;
- shared `RoundingMode` semantics covering six conventional decimal rounding directions;
- optional `runtime-generation` / `arbitrary-precision` generation with caller-buffer and caller-limit entry points; final output storage remains caller-owned.

## Feature compatibility

The default feature set remains empty. Existing feature names are part of the compatibility surface: `binary16`, `binary128`, `all-float-formats`, `complex`, `fixed-point`, `decimal`, `interop`, `runtime-generation`, `parallel-runtime`, `arbitrary-precision`, and `full`. The `full` feature is an additive umbrella over numerical capability features but intentionally excludes `parallel-runtime`; host threading/resource policy must remain an explicit caller choice.

## Rust and platform policy

Perfectπ intentionally tracks the newest stable Rust release rather than promising an older MSRV. Supported targets are validated by CI as documented in `docs/RUST_POLICY.md` and the verification documents. Dropping a declared target requires documented review.

## Semver policy

Before an actual public 1.0 package release, the repository may still carry a pre-release Cargo version while the rights holder keeps publication disabled. Once a 1.0 package is deliberately released, incompatible public API or semantic changes require a new major version; compatible additions use minor versions; compatible fixes use patch versions.

The technical freeze does not itself create a software-use license, publish a crate, or make a safety-certification claim.
