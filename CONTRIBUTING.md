# Contributing to Perfectπ

Perfectπ accepts bug reports, numerical-correctness reports, design proposals, benchmarks, documentation improvements, tests, and implementation contributions.

Copyrightable contributions are accepted only under the [Perfectπ Contributor Agreement](CONTRIBUTOR_AGREEMENT.md).

## Required contributor attestation

Every pull request containing copyrightable material must state that the contributor:

- has read and agrees to the Perfectπ Contributor Agreement;
- has the right and authority to submit the contribution;
- identifies any third-party material and its applicable terms;
- understands that accepted contributions may be distributed under the community license and separate paid commercial licenses.

Each contribution commit must also include a `Signed-off-by: Name <email>` trailer. The trailer records the contributor's attestation; it does not replace the Contributor Agreement.

## Engineering rules

Changes to the critical bounded core must preserve:

- the newest stable Rust release policy;
- `#![no_std]` compatibility;
- no heap allocation in the bounded critical core;
- no `unsafe` code;
- no `unwrap`, `expect`, `panic!`, `todo!`, or equivalent panic-driven control flow in production critical paths;
- no hidden I/O, randomness, environment state, or runtime initialization;
- no silent narrowing, truncation, or overflow;
- dependency isolation and current-version policy.

Optional extended tiers may use explicitly feature-gated dependencies or allocation only when their documented contract requires it. They must not contaminate the default dependency-free bounded core.

## Numerical changes

Any change affecting digits, rounding, conversion, derived constants, runtime generation, or error bounds must include independent reference evidence, boundary tests, explicit exact/rounded/truncated/lossy semantics, and resource impact when material.

## Verification

Before merge, applicable repository gates must remain green: formatting, source policy, dependency policy, stable/nightly checks, tests, Clippy with warnings denied, rustdoc, cross-target checks, independent numerical verification, security audit, coverage floors, fuzzing, mutation testing, and reproducibility checks.

Never weaken a legitimate test or verification gate merely to make a change pass.

## Safety language

Perfectπ must not be described as certified, failsafe, formally verified, constant-time, production-qualified, or zero-overhead unless the exact claim is supported by documented evidence and scope.
