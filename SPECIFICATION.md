# Perfectπ Normative Specification v1

Status: **Universal 1.0 release-candidate baseline**

This document is the normative technical specification for Perfectπ. It defines the invariant bounded core plus optional interoperability and arbitrary-precision tiers. If explanatory documentation conflicts with this file, this file controls until it is deliberately revised.

## 1. Scope

Perfectπ provides π representations and closely related constants for Rust systems ranging from constrained `no_std` targets to general scientific computing.

The bounded critical core shall:

- support native `f32` and `f64` π constants;
- support decimal precision requests from 0 through 40 places after the decimal point;
- avoid runtime π-generation algorithms;
- require no allocator;
- require no operating-system services;
- perform no I/O, networking, clock access, randomness, or environment inspection;
- contain no `unsafe` code;
- expose no intentional panic-driven control flow in public critical operations;
- provide deterministic output for identical inputs and build semantics.

## 2. Precision terminology

`Pi<D>` means π represented to exactly `D` decimal places after the decimal point.

`D` is not a count of significant digits.

The bounded public precision domain is exactly:

```text
0 <= D <= 40
```

Requests outside that range shall not acquire bounded π values through the public `Pi<D>` API.

## 3. Canonical mathematical reference

The canonical reference begins:

```text
3.1415926535897932384626433832795028841971693993751058209749445923...
```

The implementation may retain additional guard/reference digits internally. Public bounded precision shall not exceed 40 decimal places.

Canonical digits shall be independently reproducible and shall not rely on the implementation under test as their sole oracle.

## 4. Native floating-point constants

`PI_F32` and `PI_F64` shall correspond exactly to Rust `core`'s native π constants for the supported toolchain.

Perfectπ shall not claim that a native binary floating-point value contains more exact decimal information than its representation provides.

Native constants shall be testable by exact stored bit pattern.

## 5. Bounded decimal representation

`Pi<D>` is a compile-time precision selector. It should have zero runtime size.

A materialized bounded decimal value shall use fixed-size storage determined entirely by `D`. It shall not allocate.

The implementation shall provide direct access to the stored fractional decimal digits and a caller-buffer formatting path that performs no allocation.

## 6. Truncation

Truncation shall mean retaining exactly the first `D` fractional decimal digits of mathematical π and discarding all later digits without incrementing the retained value.

Examples:

```text
D=2  -> 3.14
D=3  -> 3.141
D=40 -> 3.1415926535897932384626433832795028841971
```

## 7. Round-to-nearest, ties-to-even

The primary rounded bounded operation shall use decimal round-to-nearest, ties-to-even semantics.

Because π is irrational, π cannot be exactly halfway between two finite decimal values. The ties-to-even rule is nevertheless part of the stable semantic contract.

Examples:

```text
D=3  -> 3.142
D=12 -> 3.141592653590
D=40 -> 3.1415926535897932384626433832795028841972
```

Truncation and rounding shall never be silently interchanged.

### 7.1 General decimal rounding policy

The public RoundingMode policy shall expose:

- toward zero;
- away from zero;
- toward negative infinity;
- toward positive infinity;
- nearest with ties to even;
- nearest with ties away from zero.

Pi<D>::with_rounding shall apply the selected policy to the bounded domain.
The runtime-generation tier shall apply the same policy names and mathematical
semantics. Existing truncation and nearest-even methods remain compatibility
conveniences.

Because π is positive and irrational, at any finite decimal precision:

- toward zero equals toward negative infinity;
- away from zero equals toward positive infinity;
- an exact halfway tie is impossible, so both nearest tie policies return the
  same value for π.

These equivalences are properties of π, not permission to collapse or rename
the public policies.

## 8. Conversion policy

The bounded core shall support explicit conversion of canonical bounded decimal π values to native `f32` and `f64`.

Lossy conversion methods shall be named explicitly:

- `to_f32_lossy()`;
- `to_f64_lossy()`.

Those methods convert the finite stored decimal value to the nearest IEEE-754 binary32/binary64 value using round-to-nearest, ties-to-even semantics. They do not claim to preserve the source's decimal-place count. Callers that want mathematical π directly in a native float shall use `PI_F32` or `PI_F64`; bounded-decimal conversion shall not silently substitute those semantics except where the correctly rounded finite decimal source has the same native encoding.

Checked preservation methods shall be available:

- `try_to_f32_preserving_places()`;
- `try_to_f64_preserving_places()`.

For the bounded π domain, checked preservation means the binary result lies within half of one unit in the source's last requested decimal place, so rounding the binary result back to `D` decimal places recovers the source value.

The verified guaranteed ranges are:

- `f32`: `D <= 6`;
- `f64`: `D <= 15`.

`D=7` contains a bounded π value that cannot preserve all seven requested places in `f32`. `D=16` contains a bounded π value that cannot preserve all sixteen requested places in `f64`. Therefore the checked APIs shall reject higher `D` values with a precision-loss error rather than silently narrow.

No API shall imply that converting `Pi<40>` to `f64` creates a 40-decimal-place `f64`.

Conversion bit patterns shall be independently verifiable from exact rational arithmetic rather than relying on Perfectπ or a language decimal parser as the sole oracle.

## 9. Formatting and serialization

The bounded core shall offer an allocation-free ASCII decimal output path using caller-provided storage.

Canonical textual output shall use:

- ASCII digits `0` through `9`;
- `.` as the decimal separator;
- no locale-dependent formatting;
- no exponent notation for bounded decimal π;
- exactly `D` fractional digits when `D > 0`;
- no decimal separator when `D = 0`.

Any future binary serialization format shall specify byte order and versioning explicitly.

## 10. Error behavior

Expected resource/input errors shall be represented as values rather than intentional panics.

Caller-buffer formatting shall report the required and provided byte counts when the supplied buffer is too small.

## 11. Resource behavior

The native constant path shall not require the bounded decimal machinery at runtime.

The bounded path shall have finite, documented storage and execution bounds.

Perfectπ shall prefer straightforward, auditable representations over micro-optimizations unless measurement demonstrates a material resource benefit.

Resource claims shall be published only after measurement on representative targets.

## 12. Derived constants

Derived constants such as `TAU`, `FRAC_PI_2`, `FRAC_PI_3`, `FRAC_PI_4`, `FRAC_PI_6`, `FRAC_PI_8`, `INV_PI`, `TWO_INV_PI`, and `TWO_INV_SQRT_PI` may be exposed when their semantics are explicit and verified.

For the native tier, direct aliases of Rust `core` constants are preferred over runtime recomputation.

## 13. Optional numeric tiers

IEEE-754 binary16 and binary128 support shall remain optional and shall not define the canonical Perfectπ precision model.

While Rust's native `f16` / `f128` primitive types remain unstable on the supported stable toolchains, Perfectπ shall provide stable interchange adapters instead of requiring nightly Rust:

- feature `binary16` exposes a transparent `Binary16(u16)` bit representation;
- feature `binary128` exposes a transparent `Binary128(u128)` bit representation;
- feature `all-float-formats` enables both;
- the default feature set enables neither.

The optional adapters shall:

- contain no arithmetic emulation;
- require no allocator;
- require no external dependency;
- provide exact IEEE bit access and endian-stable byte extraction;
- expose π, τ, π/2, π/3, π/4, π/6, π/8, 1/π, 2/π, and 2/√π;
- use round-to-nearest, ties-to-even reference encodings;
- be independently verified from more than one π computation;
- compile on the current stable Rust release and the supported `no_std` target matrix.

Names implying stable native Rust primitives, including `PI_F16` and `PI_F128`, are reserved until those primitive types can be supported on current stable Rust without weakening the portability contract.

Arbitrary precision beyond 40 decimal places belongs outside the bounded critical core.

Runtime π-generation algorithms belong outside the bounded critical core.

Optional tiers must not become dependencies of the smallest default path.

### 13.1 Optional interoperability

Interoperability features shall remain opt-in and shall not add dependencies to the default bounded core.

Current interoperability contracts are:

- `complex`: uses the newest stable `num-complex` release and reuses Perfectπ's existing checked/lossy `f32` / `f64` conversion semantics with an exactly zero imaginary component;
- `fixed-point`: uses the newest stable `fixed` release and converts the stored finite decimal representation through the destination crate's decimal parser using round-to-nearest, ties-to-even semantics without routing through binary floating point;
- `decimal`: uses the newest stable `rust_decimal` release, converts exactly when `D <= 28`, rejects wider exact requests, and provides an explicitly named nearest-even conversion for wider bounded values;
- `interop`: enables all current interoperability features.

All direct interoperability dependencies shall:

- remain optional;
- have default features disabled unless a documented requirement proves otherwise;
- be kept at the newest appropriate stable release by required CI;
- compile across the supported current-stable `no_std` target matrix when their feature is enabled.

The default Perfectπ dependency graph shall remain empty.

### 13.2 Optional runtime generation and arbitrary precision

Runtime generation shall remain opt-in and outside the bounded critical core.

The `runtime-generation` feature shall:

- use arbitrary-precision integer arithmetic without changing the default dependency graph;
- keep its direct dependency optional, exact-version pinned, and configured with default features disabled;
- expose caller-buffer ASCII generation with the shared RoundingMode semantics;
- retain explicit truncation and nearest-even compatibility entry points;
- provide a caller-selected maximum-precision gate that rejects before expensive generation or output mutation;
- keep final output storage caller-owned rather than adding a direct Perfectπ allocation convenience;
- avoid binary floating-point as an intermediate representation;
- reject undersized caller buffers before modifying them;
- use variable resource cost proportional to the requested precision;
- compile on the supported current-stable `no_std` target matrix.

The `arbitrary-precision` feature is the semantic alias for this precision-above-40 capability. The `full` feature enables all current production capability tiers: all IEEE interchange adapters, all interoperability adapters, and arbitrary precision. The default feature set remains empty.

The implemented generator uses Machin's identity with conservative integer lower/upper bounds. Guard precision shall increase until both bounds prove the same requested truncation. Nearest-even generation shall certify at least one additional decimal digit before rounding. Because pi is irrational, an exact finite decimal halfway tie cannot occur.

Runtime-generation results shall be independently checked against at least two separately implemented pi algorithms at precision materially above the bounded 40-place tier. All public runtime rounding modes shall be covered by independent reference checks or by a documented mathematical reduction to independently verified values.

## 14. Verification

Before a bounded release is described as production-ready, verification shall include:

- known-answer testing for every `D` from 0 through 40;
- truncation vectors;
- round-to-nearest-even vectors;
- exhaustive bounded equivalence checks for every public RoundingMode at every D from 0 through 40;
- native exact-bit tests;
- allocation-free formatting tests;
- compile-time rejection evidence for unsupported `Pi<D>` operations;
- `no_std` builds;
- representative cross-target builds;
- static linting;
- independent canonical-digit verification;
- exact-rational verification of every bounded truncation/rounding conversion to `f32` and `f64`;
- verification of the checked decimal-place preservation boundaries and their first failing cases.

Additional verification should include fuzz/property testing, mutation testing, reproducibility checks, and model/exhaustive checking where they materially improve confidence.

## 15. Rights and release status

Perfectπ is source-available under the PolyForm Noncommercial License 1.0.0 for
the community/noncommercial uses granted by LICENSE. Commercial use requires a
separate written paid commercial license from DrTomLLC as described by
RIGHTS.md and COMMERCIAL_LICENSING.md.

The Cargo package remains `publish = false`. Package publication and creation
of a public release are separate owner-controlled actions and are not implied
by implementation or verification completion.

## 16. Change control

Changes to any of the following are specification changes and require deliberate review:

- the `0..=40` bounded precision range;
- canonical digits;
- rounding semantics;
- truncation semantics;
- conversion semantics;
- critical-core allocation or `unsafe` policy;
- panic/error behavior;
- public serialization semantics;
- claims about safety, certification, or measured resource usage;
- public RoundingMode meanings;
- feature-tier meanings, including full;
- caller-visible runtime resource-limit behavior;
- declared portability and qualification evidence.
