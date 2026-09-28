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

Automatic startup self-test is intentionally not provided. The evaluation and fault-model rationale are documented in [Startup Integrity Self-Test Evaluation](INTEGRITY_SELF_TEST.md). Applications that require startup or periodic integrity checking should use independently stored expected values at the application boundary.

## Certification and qualification

No README claim, crate feature, or test suite may describe Perfectπ as safety-certified unless an applicable certification process has actually been completed and its scope is documented.

Perfectπ is designed to be qualification-friendly across critical-system domains. Repository evidence can be reused, but a deployment must bind the exact crate commit, features, compiler, target, linker/application, hardware, fault model, and governing assurance process. The concrete evidence checklist and requalification triggers are defined in [Critical-System Qualification Guide](QUALIFICATION.md).

## Verification expectations

The bounded domain permits exhaustive precision-vector testing from `Pi<0>` through `Pi<40>`. Verification should also include exact native bit-pattern checks, rounding-boundary tests, cross-target tests, fuzz/property tests, mutation testing, and independent reference generation.

Verification-only dependencies must remain outside production dependency graphs.
