# Release Criteria

Perfectπ will not call a release production-ready merely because it compiles.

The repository licensing baseline is now established: PolyForm Noncommercial 1.0.0 for community/noncommercial use, separate paid commercial licensing, and the Perfectπ Contributor Agreement. Cargo publication remains disabled until an explicit package-release action is approved.

## Before the first public crate release

- final licensing model and contributor-rights framework selected and published;
- stable public naming and package metadata;
- critical core builds with `#![no_std]`;
- bounded core requires no allocator;
- critical core contains no `unsafe` code;
- critical public APIs have no known panic paths;
- `f32` / `f64` native constants verified;
- `Pi<D>` semantics implemented for the declared range;
- rounding, truncation, and lossy conversion behavior documented and tested;
- all `D = 0..40` known-answer tests pass;
- supported target matrix builds successfully;
- optional feature tiers build independently and in the all-features configuration without contaminating the default dependency graph;
- every direct optional integration dependency is at the newest appropriate stable release, optional, and configured with default features disabled unless explicitly justified;
- resource measurements published for representative constrained and general-purpose targets;
- independent canonical-digit verification completed;
- security and dependency review completed.

## Before a stable 1.0 release

- API and semver policy frozen;
- current-Rust-only policy documented, with the manifest minimum updated to the newest stable Rust release;
- compatibility commitments documented;
- cross-target reproducibility results published;
- fuzz/property and mutation testing integrated;
- benchmark methodology stabilized;
- no unresolved correctness issue affecting canonical values or conversion semantics;
- documentation examples validated against released API.

## Safety-related claims

Any statement using words such as certified, formally verified, failsafe, or qualified must identify the exact process, scope, target, and evidence that justifies it.
