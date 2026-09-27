# Contributing to Perfectπ

Perfectπ is intended to become foundational numerical infrastructure. Changes should be small, evidence-driven, and easy to audit.

## Core rules

For the critical core:

- stable Rust unless a documented compatibility decision says otherwise;
- `#![no_std]` compatible;
- no heap allocation;
- no `unsafe` code;
- no `unwrap`, `expect`, `panic!`, `todo!`, or equivalent panic-driven control flow in production critical paths;
- no hidden I/O, randomness, environment state, or runtime initialization;
- no silent narrowing, truncation, or overflow;
- no dependency added without a documented necessity and impact review.

## Numerical changes

Any change affecting digits, rounding, conversion, derived constants, or error bounds must include:

1. the mathematical rule being implemented;
2. independent reference evidence;
3. boundary tests;
4. an explanation of whether behavior is exact, rounded, truncated, or lossy;
5. resource impact when material.

## Pull requests

Keep pull requests focused. Include tests and documentation with the behavior they change. Do not combine broad refactors with numerical-semantic changes unless unavoidable.

## Performance

Performance claims require measurements. Prefer smaller trusted code over clever micro-optimizations that are not demonstrated by benchmarks.

## Safety language

Do not describe code as certified, failsafe, formally verified, constant-time, or zero-overhead unless the specific claim is backed by evidence and its scope is stated.
