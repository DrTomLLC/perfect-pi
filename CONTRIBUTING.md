# Contributing to Perfectπ

## Current contribution status

Perfectπ is currently **all rights reserved** and is **not accepting external code, documentation, or other copyrightable contributions** while the project owner determines the final licensing and contributor-rights model.

This restriction is intentional: accepting third-party code before that framework exists could create ownership or relicensing ambiguity.

You may still participate through:

- bug reports;
- numerical / precision discrepancy reports;
- feature proposals;
- design discussion;
- reproducible benchmark or compatibility observations.

Please do **not** submit source-code patches, pull requests containing implementation material, or substantial replacement documentation unless the project owner explicitly requests that contribution under separately stated terms.

## Future contribution policy

Before external code contributions are opened, Perfectπ will publish a contributor-rights policy appropriate to the final licensing model. That may include contribution terms, a Developer Certificate of Origin, a contributor license agreement, assignment terms, or another explicit mechanism.

No such mechanism is in effect today.

## Engineering rules

When code contributions are eventually opened, the critical core is intended to follow these requirements:

- stable Rust unless a documented compatibility decision says otherwise;
- `#![no_std]` compatible;
- no heap allocation;
- no `unsafe` code;
- no `unwrap`, `expect`, `panic!`, `todo!`, or equivalent panic-driven control flow in production critical paths;
- no hidden I/O, randomness, environment state, or runtime initialization;
- no silent narrowing, truncation, or overflow;
- no dependency added without a documented necessity and impact review.

## Numerical changes

Any future change affecting digits, rounding, conversion, derived constants, or error bounds must include independent reference evidence, boundary tests, explicit exact/rounded/truncated/lossy semantics, and resource impact when material.

## Safety language

Perfectπ must not be described as certified, failsafe, formally verified, constant-time, or zero-overhead unless the exact claim is supported by evidence and its scope is stated.
