# Perfectπ Release-Candidate Critic Review

Date: 2026-09-28

## Scope

This review was performed after implementation from an isolated Git worktree based on main commit `44e9215781fdaa5fa87b230db695b91bdcd1f3ea`. It evaluates the repository-scoped release candidate, not downstream hardware qualification or a published crate release.

## Findings resolved during review

- Removed reliance on run-dependent libFuzzer corpus counts as stable evidence; execution counts and coverage/features remain documented.
- Documented that callers must bound untrusted runtime precision requests because arbitrary-precision generation is deliberately variable-cost and allocation-backed.
- Re-ran host timing in the isolated candidate and updated the retained benchmark medians.
- Confirmed the selected licensing model is the repository's intended dual path: PolyForm Noncommercial 1.0.0 for community/noncommercial use plus separate paid commercial licensing, with the Perfectπ Contributor Agreement preserving project-owner relicensing/commercialization rights.
- Confirmed Cargo publication remains explicitly disabled with `publish = false`.

## Correctness and numerical evidence

- Every bounded precision `Pi<0>` through `Pi<40>` is covered by known-answer testing.
- All 164 bounded decimal-to-native conversion outcomes are independently checked against exact-rational IEEE bit generation.
- Binary16 and binary128 π-family constants are independently checked against two π algorithms.
- Runtime truncation and nearest-even generation match independent Chudnovsky and Gauss-Legendre references through 1,000 fractional digits at nine checkpoints.
- Runtime generation rejects undersized buffers before modifying them.
- The runtime algorithm uses conservative integer lower/upper bounds and emits a result only when the requested decimal truncation is certified.

## Local verification evidence

The isolated candidate passed:

- `cargo fmt --all --check`;
- source, dependency, and current-Rust policy scripts;
- independent canonical, conversion, float-format, interoperability, and runtime-generation verification scripts;
- `cargo check --all-targets --all-features`;
- `cargo test --all-targets --all-features`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --doc --all-features`;
- rustdoc with warnings denied;
- current stable and current nightly checks/tests;
- nightly native f16/f128 comparison;
- all-feature checks on Cortex-M0, Cortex-M hardware-float, bare-metal RISC-V, WebAssembly, and AArch64;
- RustSec audit of the resolved all-feature dependency graph;
- coverage floors, measuring 95.23% lines, 91.80% regions, and 100% functions;
- mutation testing: 170 generated, 158 caught, 12 compiler-unviable, 0 missed, 0 timed out;
- Linux/WSL AddressSanitizer fuzzing: 25,000 bounded API runs, 25,000 interoperability runs, and 10,000 runtime-generation runs with no crash or assertion failure;
- reproducible constrained-target resource, static instruction, static stack-frame, linked-size, and host-timing measurement scripts.

## GitHub CI evidence

PR #9 on candidate commit `b842266255cdb1b6d1379b22e9e0779aa853c9a7` passed every configured CI job before the final documentation-only review corrections: quality/verification, mutation, fuzzing, coverage, RustSec audit, current stable, current nightly, all declared no_std targets, and Linux/Windows/macOS host reproducibility.

The final documentation corrections must also receive a green CI run on their exact commit before merge.

## Remaining scope boundaries

No repository-generic test can establish target-specific transitive stack high-water, WCET/cycle counts, power consumption, or final firmware/application deltas without a selected board, linker script, clock/memory configuration, workload, and measurement method. Those are downstream hardware-qualification tasks, not missing generic repository implementation.

No crates.io publication or GitHub release is performed by repository completion. Publication is a separate explicit owner-controlled action.

## Review conclusion

No unresolved repository-scoped correctness, portability, verification, licensing-framework, or documentation blocker was identified after the corrections above. Merge readiness is conditional only on the final GitHub CI run passing on the exact corrected PR head.

## 2026-10-02 specialized-runtime assurance addendum

This addendum covers the later specialized dependency-free runtime and performance work on branch `perf/specialized-pi-arithmetic-local`, whose pre-specialization base is commit `d95e1f7864bd12bf58f38951680126aedbf63412`. The authoritative corrected remote SHA is tracked by pull request #13 and Git history rather than hard-coded into this review document.

The final local candidate has no known repository-scoped implementation blocker. Evidence on the exact corrected working tree includes:

- Rust 1.99.0 stable formatting, all-target/all-feature check, Clippy with warnings denied, full tests, doc tests, and rustdoc with warnings denied;
- current nightly full tests and the complete declared feature matrix;
- representative all-feature cross-target checks for Cortex-M0, Cortex-M hardware-float, bare-metal RISC-V, WebAssembly, and AArch64 Linux;
- independent runtime-prefix regeneration plus all six public runtime rounding modes verified through 36,808 fractional digits at fourteen checkpoints, including the 10,000/10,001 fast-path handoff and reciprocal-division regression points at 36,000, 36,700, and 36,808 places;
- full-repository mutation testing with 580 generated mutants after two documented mathematically equivalent bit-reversal exclusions: 510 caught, 70 compiler-unviable, 0 missed, and 0 timed out;
- all-feature source coverage of 96.06% lines, 92.31% regions, and 100.00% functions;
- Linux/WSL AddressSanitizer fuzzing totaling 60,064 executions: 25,000 bounded API, 25,000 interoperability, 10,000 general runtime-generation runs, and 64 seeded deep-arithmetic runs covering 10,001 / 36,000 / 36,700 / 36,808 places, with no crash or sanitizer finding;
- RustSec audit of the 21-crate resolved lockfile with no vulnerability failure;
- package construction and verification of 113 files;
- byte-for-byte reproducibility of all 30 representative probe objects across repeated Windows and Linux builds using rustc 1.99.0 commit `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`;
- retained pre-Astra-repair performance medians on the Windows host of 116.948 ms serial / 77.504 ms parallel at 100,000 fractional places and 1.798 s serial / 1.131 s parallel at 1,000,000 places, with retained full-output digests unchanged; the final competitor benchmark is intentionally deferred until the repaired exact SHA passes the follow-up Astra audit.

The two mutation exclusions are behaviorally equivalent for valid power-of-two NTT lengths: using `index > reversed` performs the same disjoint bit-reversal transpositions from the opposite endpoint, while `index <= reversed` only adds self-swaps. An independent exhaustive check confirmed identical permutations for power-of-two lengths through 4096.

The remaining gate is external rather than a local implementation task: the eventual exact pushed commit must pass the repository's required GitHub Actions matrix, including macOS host reproducibility. No commit, push, merge, package publication, or release is performed by this local assurance pass.
