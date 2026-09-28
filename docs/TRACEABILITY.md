# Universal 1.0 Traceability

This file maps Universal 1.0 requirements to implementation and verification
evidence. A row marked CI is not considered satisfied for a merge unless the
exact pull-request head passes the referenced gate.

| ID | Requirement | Implementation / evidence |
| --- | --- | --- |
| PPI-U-001 | Default core is no_std, dependency-free, allocator-free, and unsafe-free | Cargo.toml default features; src/lib.rs; source-policy and dependency-policy CI |
| PPI-U-002 | Bounded decimal π supports every D=0..=40 | src/bounded.rs; bounded_precision and finite_state_model tests |
| PPI-U-003 | Six explicit decimal rounding policies are available | src/rounding.rs; Pi::with_rounding; rounding_modes tests |
| PPI-U-004 | Native f32/f64 constants and conversions have exact verified semantics | src/native.rs; src/conversion.rs; verify_conversions.py |
| PPI-U-005 | Stable binary16/binary128 interchange is optional and dependency-free | src/binary16.rs; src/binary128.rs; verify_float_formats.py |
| PPI-U-006 | Complex/fixed/decimal interoperability is optional | interop modules; tests/interop.rs; verify_interop.py |
| PPI-U-007 | Precision above 40 is opt-in and independently certified before output | src/runtime.rs; runtime tests; verify_runtime_generation.py |
| PPI-U-008 | Runtime callers can enforce a precision/resource ceiling before expensive work or mutation | generate_pi_ascii_with_limit; generate_pi_string; runtime_rounding tests; runtime fuzz target |
| PPI-U-009 | One aggregate feature enables all production capability tiers | Cargo.toml full feature; feature-isolation CI |
| PPI-U-010 | Core portability spans representative embedded, endian, word-size, OS, mobile, and WebAssembly classes | portable-core-matrix CI |
| PPI-U-011 | Selected no_std targets validate all optional production features | no-std-targets CI |
| PPI-U-012 | Host behavior is validated on Linux, Windows, and macOS | host-matrix CI |
| PPI-U-013 | Production paths reject expected errors as values and implement core Error where applicable | error types; error_contracts tests; clippy/source policy |
| PPI-U-014 | New logic remains covered by source coverage, mutation, and sanitizer fuzzing | coverage, mutation, fuzz CI jobs |
| PPI-U-015 | Dependency advisories and direct-dependency currency are checked | cargo audit; dependency policy CI |
| PPI-U-016 | Public numerical claims are reproducible independently | reference/conversion/float/interop/runtime Python verifiers |
| PPI-U-017 | Critical-system use has an explicit target-qualification boundary | docs/QUALIFICATION.md and docs/SAFETY.md |
| PPI-U-018 | Package/release publication is owner-controlled | Cargo.toml publish=false; release documentation |
| PPI-U-019 | Licensing matches the source-available dual-license model | LICENSE, RIGHTS.md, COMMERCIAL_LICENSING.md, SPECIFICATION.md |
| PPI-U-020 | Public API and semantic changes are controlled | SPECIFICATION.md; API_STABILITY.md; SemVer policy |

## Evidence rule

The strongest available evidence controls: exact source and commit, reproducible
tests, CI job records, independent reference calculations, then explanatory
documentation. A documentation statement cannot override failing executable
evidence.

## Toolchain limitation log

Rust 1.98.1 does not yet make slice get/get_mut const-stable for the operations
used by the panic-safe bounded implementation. Perfectπ therefore does not
weaken safe indexing or introduce unsafe code merely to make bounded
materialization const. Revisit this when stable Rust can express the same
invariants without reducing the production safety policy.
