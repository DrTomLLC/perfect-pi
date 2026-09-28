# Perfectπ Architecture

## Objective

Perfectπ provides a single Rust π ecosystem without forcing constrained targets to carry high-precision machinery they do not use.

## Architectural rule

**The default path is optimized for the smallest valid target. Expensive capability is opt-in.**

## Layers

### 1. Bounded critical core

Target properties:

- `#![no_std]`;
- no allocator;
- no filesystem, network, environment, clock, or randomness dependencies;
- no runtime π generation;
- no `unsafe` code;
- no panicking public critical path;
- deterministic output;
- compile-time bounded precision;
- fixed and documented memory and execution bounds.

The implemented bounded core exposes native `f32` / `f64` representations, verified native derived constants, the bounded `Pi<D>` precision family, checked precision-preserving conversions, and explicitly named lossy conversions to `f32` / `f64`. Conversion semantics are exhaustively verified across the finite bounded domain, while native-only callers remain separate from bounded conversion machinery.

### 2. Optional float adapters

`f16` and `f128` support must not determine the core precision model. They are adapters to the canonical Perfectπ representation and may be conditioned on Rust/toolchain/target support.

### 3. Optional interoperability

Interoperability with complex, fixed-point, decimal, unit-aware, and other numerical crates must remain optional and must not introduce dependencies into the bounded core.

### 4. Optional arbitrary precision

Precision beyond 40 decimal places belongs outside the bounded core. It may allocate, use larger dependencies, and have variable execution cost. None of that code may be pulled into critical builds unless explicitly enabled.

### 5. Optional π calculation algorithms

Algorithms that derive π at runtime are research/verification functionality, not the mechanism used by the bounded critical core to obtain a constant that is already known.

### 6. Verification infrastructure

Heavy reference libraries, independent π algorithms, fuzzing, mutation testing, model checking, and cross-target validation are development tools only.

## Canonical data

The production core stores 41 fractional digits: the 40 public bounded digits plus the one additional digit required to round at `D=40`. π's irrationality makes an exact finite decimal halfway tie impossible.

Verification does not depend on carrying audit-only digits in flight/runtime data. Two independent algorithms currently agree through at least 64 fractional digits and verify the 41 production digits plus every bounded precision vector.

Straightforward canonical data is preferred over packing when the saved bytes do not justify additional decode logic and verification complexity.

## `Pi<D>` representation

`D` is a compile-time precision marker, not runtime state. Implementations should avoid duplicating substantial generic code for each precision value and should keep shared internals non-generic where practical.

The valid public bounded domain is:

```text
0 <= D <= 40
```

Out-of-range precision should be rejected at compile time where stable Rust permits a clear and maintainable implementation.

## Native conversion architecture

Bounded decimal values convert to the nearest IEEE-754 binary32/binary64 representation of the **stored finite decimal value**, using round-to-nearest, ties-to-even semantics. This is deliberately distinct from requesting mathematical π directly as `PI_F32` or `PI_F64`.

The finite Perfectπ domain permits a compact verified implementation: only `D=0..7` need distinct binary32 reference encodings, while every canonical value at `D>=8` maps to `PI_F32`; only `D=0..14` need distinct binary64 encodings, while every canonical value at `D>=15` maps to `PI_F64`. The low-precision encodings are generated and checked independently from exact rational arithmetic.

Checked conversion is stricter than merely returning a nearest float: it guarantees that all requested fractional decimal places remain recoverable. The verified conservative boundaries are `D<=6` for `f32` and `D<=15` for `f64`. Beyond those boundaries the caller must opt into an explicitly lossy method.

## Separation of representation and computation

Perfectπ distinguishes:

- mathematical π;
- canonical decimal digits;
- requested precision;
- stored representation;
- display precision;
- conversion precision;
- rounding policy;
- runtime computation algorithms.

These concepts must not be silently conflated.

## Derived constants

Frequently used values such as `TAU`, `FRAC_PI_2`, `FRAC_PI_4`, `INV_PI`, and related constants may be first-class verified values when doing so improves determinism and avoids unnecessary runtime operations.

## Compatibility

The critical core should target stable Rust. Experimental numeric types must remain optional until their compiler and target behavior is suitable for the project's support guarantees.
