# Verification Strategy

Perfectπ must earn trust through reproducible evidence rather than branding claims.

## Bounded-domain verification

The public bounded precision domain contains only 41 precision points: `Pi<0>` through `Pi<40>`. Each supported precision must have known-answer coverage for every defined rounding policy.

## Native representation verification

For supported native floating-point constants, tests should verify exact bit patterns where the Rust/platform contract makes that meaningful, alongside numerical error documentation.

## Independent references

Canonical π digits and derived constants should be checked against more than one independent source or generation method. A single library must not serve as both implementation and sole oracle.

## Required verification classes

- unit and known-answer tests;
- every bounded precision value;
- rounding and truncation boundaries;
- checked/lossy conversion boundaries;
- integer overflow and narrowing boundaries;
- cross-target builds;
- cross-target reproducibility checks;
- property/fuzz tests where inputs exist;
- mutation testing of critical logic;
- static analysis and lints;
- model checking or exhaustive-state techniques where they materially improve confidence.

## Verification isolation

Reference generators, arbitrary-precision packages, fuzzers, model checkers, and other heavy tools belong in development/verification dependency graphs only.

## Evidence retention

Release artifacts should record the compiler version, targets, feature sets, test results, benchmark methodology, and source revision used to support published claims.
