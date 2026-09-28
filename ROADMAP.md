# Perfectπ Roadmap

## Phase 0 — Architecture baseline

- [x] define project mission;
- [x] establish public `0..40` bounded precision ceiling;
- [x] separate native, bounded, optional float, arbitrary, calculation, and verification layers;
- [x] define `no_std` / no-allocation / deterministic critical-core goals;
- [x] define precision and rounding terminology;
- [x] establish public repository and governance documentation.

## Phase 1 — Native core

- [x] create minimal Rust workspace;
- [x] establish current-Rust-only policy: minimum supported Rust tracks the newest stable release (currently Rust 1.98.1);
- [x] implement native `f32` / `f64` constants;
- [x] implement verified derived constants;
- [x] enforce `no_std`, no allocation, and `forbid(unsafe_code)`;
- [x] add exact bit-pattern and known-answer tests;
- [x] add CI quality and cross-target gates.

## Phase 2 — Bounded precision

- [x] implement `Pi<D>` for `0 <= D <= 40`;
- [x] define stable compile-time bound enforcement;
- [x] implement explicit rounding and truncation semantics;
- [x] implement checked and explicitly lossy `f32` / `f64` conversions;
- [x] exhaustively test all supported `D` values.

## Phase 3 — Portability and optional adapters

- [x] validate Cortex-M and RISC-V `no_std` builds;
- [x] validate x86-64 and AArch64 builds;
- [x] validate WebAssembly build;
- [x] add optional IEEE-754 binary16 support on stable Rust (bit-format adapter; native `f16` reserved until Rust stabilization);
- [x] add optional IEEE-754 binary128 support on stable Rust (bit-format adapter; native `f128` reserved until Rust stabilization);
- [x] add optional fixed/decimal/complex interoperability without contaminating core dependencies.

## Phase 4 — Verification and resource evidence

- [x] independent reference generation;
- [x] fuzz/property testing;
- [x] mutation testing;
- [x] exhaustively check bounded-state invariants where the finite state space is tractable;
- [x] cross-target reproducibility tests;
- [x] measure software-side ROM/read-only data, writable sections, static stack frames, linked host size, static instruction counts, and host timing;
- [x] publish reproducible benchmark and resource reports.

Target-hardware transitive stack high-water, WCET/cycles, power, software-vs-hardware-float cost, and final firmware deltas remain target-specific qualification work; they require a selected board/application and are not inferred from generic repository measurements.

## Phase 5 — Extended ecosystem

- [x] optional runtime π-generation algorithm with independently verified interval bounds;
- [x] optional arbitrary precision above 40 places;
- [x] scientific/engineering integration examples;
- [x] evaluate optional startup integrity self-test (decision: application-owned; no automatic library startup hook).

## Phase 6 — Release readiness

- [x] freeze supported technical API baseline and compatibility policy;
- [x] establish community/noncommercial licensing, paid commercial licensing, and contributor-rights framework;
- [x] complete an independent critic/verifier review of the release-candidate tree and retain the evidence;
- [x] establish the publication gate: `publish = false`; no crate publication or GitHub release occurs without a separate explicit release action.

## Phase 7 — Universal 1.0

- [x] unify bounded and arbitrary-precision rounding under six explicit policies;
- [x] add caller-enforced runtime precision/resource ceilings;
- [x] add an aggregate `full` feature without changing the empty default feature set;
- [x] expand runtime independent verification and sanitizer fuzzing to the new policies;
- [x] define the Universal Support Contract and critical-system qualification boundary;
- [x] establish requirement-to-evidence traceability;
- [x] expand core portability CI across embedded, endian, word-size, OS, mobile, WebAssembly, and architecture classes;
- [ ] pass all repository verification gates on the exact Universal 1.0 pull-request head;
- [x] complete independent critic review of the Universal 1.0 production candidate and retain the evidence;
- [ ] merge only after exact-head CI and critic evidence are green.

Phases 0 through 6 remain complete. Phase 7 is the current Universal 1.0 release-candidate gate. Publishing a stable crate/release remains a separate owner-controlled action and is not an implicit side effect of completing or merging Phase 7.
