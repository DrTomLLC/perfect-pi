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

The core will expose native `f32` / `f64` representations, verified derived constants, the bounded `Pi<D>` precision family, and checked conversions.

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

The implementation should retain enough independently verified guard digits internally to round every public value through 40 decimal places correctly. The current design target is approximately 50 fractional reference digits.

Straightforward canonical data is preferred over clever compression when the saved bytes are insignificant compared with the increase in verification complexity.

## `Pi<D>` representation

`D` is a compile-time precision marker, not runtime state. Implementations should avoid duplicating substantial generic code for each precision value and should keep shared internals non-generic where practical.

The valid public bounded domain is:

```text
0 <= D <= 40
```

Out-of-range precision should be rejected at compile time where stable Rust permits a clear and maintainable implementation.

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
