# Safety and Reliability

Perfectπ is designed to be suitable as a small numerical dependency in high-reliability Rust systems. The project will make only claims that are supported by implementation evidence and measured verification.

## Critical-core design goals

- no allocator;
- no hidden initialization;
- no runtime π generation;
- no I/O;
- no randomness;
- no environment dependence;
- no `unsafe` code;
- no panicking public critical path;
- no silent truncation or narrowing;
- no silent integer overflow;
- deterministic results;
- bounded precision;
- bounded resource use;
- reproducible test vectors.

## Fault model

A software crate cannot guarantee immunity from radiation-induced bit flips, memory corruption, flash corruption, CPU faults, power faults, compiler defects, or system-integration failures.

Perfectπ should therefore support fault-aware systems rather than falsely claim to replace them.

An optional startup integrity/self-test facility may be provided for environments that want to verify canonical data before use. It must not impose continuous runtime overhead on ordinary callers.

## Certification

No README claim, crate feature, or test suite may describe Perfectπ as safety-certified unless an applicable certification process has actually been completed and its scope is documented.

## Verification expectations

The bounded domain permits exhaustive precision-vector testing from `Pi<0>` through `Pi<40>`. Verification should also include exact native bit-pattern checks, rounding-boundary tests, cross-target tests, fuzz/property tests, mutation testing, and independent reference generation.

Verification-only dependencies must remain outside production dependency graphs.
