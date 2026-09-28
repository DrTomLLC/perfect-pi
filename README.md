# Perfectπ

<p align="center">
  <img src="docs/assets/brand/perfect-pi-icon-dark-512.png" alt="Perfectπ logo" width="160">
</p>

**Perfect Pi** — universal, deterministic, resource-explicit π infrastructure for Rust.

> One π ecosystem from tiny `no_std` targets through application, scientific, arbitrary-precision, and qualification-oriented critical-system use—without making small targets pay for large-target capabilities.

## Status

**Universal 1.0 work is implemented on the release-candidate path: bounded core, explicit six-mode rounding, optional adapters, guarded arbitrary precision, broad portability CI, and qualification evidence; minimum supported Rust tracks current stable Rust 1.98.1; no public crate release yet.**

The repository contains a working `no_std` bounded core, optional IEEE/interoperability adapters, and opt-in runtime/arbitrary-precision generation. The bounded core remains allocation-free and dependency-free by default. The project is still pre-release and must not be treated as safety-certified.

## Why Perfectπ

Rust already provides π for native floating-point types, and arbitrary-precision crates can calculate far more digits than physical systems normally need. Perfectπ is designed to fill the space between those extremes:

- native `f32` / `f64` fast paths with effectively no library overhead;
- a bounded, deterministic decimal precision model from 0 through **40 places after the decimal point**;
- `#![no_std]` and no-allocation operation for the critical bounded core;
- six explicit decimal rounding policies plus checked precision-preserving and explicitly named lossy conversions — never silent precision claims;
- fixed and documented memory / execution bounds;
- optional stable IEEE `binary16` / `binary128` interchange adapters, ecosystem interoperability, and independently verified runtime/arbitrary-precision generation;
- verification tooling kept outside production dependencies.

## Design priorities

1. **Correctness before cleverness.**
2. **The smallest target pays only for the capability it uses.**
3. **Precision is explicit, bounded, and testable.**
4. **No hidden allocation, initialization, I/O, randomness, or runtime π generation in the critical core.**
5. **No `unsafe` code in the critical core.**
6. **No panicking public path in the critical core.**
7. **No silent precision loss.**
8. **Portable, reproducible behavior across supported Rust targets.**

## Precision model

| Tier | Purpose | Expected use |
| --- | --- | --- |
| Native `f32` | smallest practical binary float path | embedded, controls, graphics, DSP |
| Native `f64` | default general/scientific path | engineering, navigation, simulation, biomedical, finance |
| Optional IEEE `binary16` / `binary128` adapters | exact stable interchange bits while native Rust `f16` / `f128` remain unstable | specialized hardware, file/wire formats, extended numerical work |
| `Pi<D>` where `0 <= D <= 40` | bounded decimal precision | validation, deterministic scientific work, high-precision physical computation |
| Optional arbitrary precision | more than 40 decimal places | mathematics, benchmarking, research |

`Pi<40>` means **40 digits after the decimal point**, not 40 significant digits.

The bounded core stops at 40 decimal places deliberately. That ceiling is far beyond meaningful physical measurement requirements at observable-universe scales while keeping the state space finite, resource-bounded, and exhaustively testable.

## Intended domains

Perfectπ is being designed as a reusable π foundation for:

- aerospace, spacecraft, navigation, and deep-space systems;
- electrical and electronics engineering, RF, DSP, power systems, and controls;
- mechanical, civil, construction, manufacturing, CAD/CAM, and surveying;
- quantum computing and quantum physics;
- particle, nuclear, and high-energy physics;
- biomedical engineering, imaging, instrumentation, and computational medicine;
- computer science, embedded systems, robotics, graphics, simulation, and WebAssembly;
- scientific and numerical computing;
- quantitative finance and statistical computing.

Perfectπ owns **π and its numerical semantics**. It does not attempt to become a domain-specific mathematics framework.

## Planned architecture

```text
perfect-pi
├── bounded core
│   ├── PI_F32 / PI_F64
│   ├── verified derived constants
│   ├── Pi<D>, D = 0..40
│   ├── checked conversions
│   └── fixed resource bounds
├── optional float adapters
│   ├── IEEE binary16
│   └── IEEE binary128
├── optional interoperability
│   ├── complex
│   ├── fixed-point
│   └── decimal / scientific numeric types
├── optional arbitrary precision
├── optional runtime π-generation algorithms
└── verification infrastructure
```

See [Architecture](docs/ARCHITECTURE.md) for the full separation of concerns.

## Current core API

The native path directly exposes Rust `core` floating-point constants:

```rust
use perfect_pi::PI_F64;

let circumference = 2.0 * PI_F64 * radius;
```

Bounded decimal precision is compile-time selected and has explicit semantics:

```rust
use perfect_pi::Pi;

let truncated = Pi::<40>::truncated();
let rounded = Pi::<40>::round_nearest_even();
```

`Pi<41>` cannot use the bounded-value operations. Native-float conversion is explicit:

```rust
let p6 = Pi::<6>::round_nearest_even();
let f32_checked = p6.try_to_f32_preserving_places(); // guaranteed through D=6

let p40 = Pi::<40>::round_nearest_even();
let f64_lossy = p40.to_f64_lossy(); // explicit precision loss
```

Checked conversion guarantees all requested decimal places through `D=6` for `f32` and `D=15` for `f64`. Beyond those boundaries callers must deliberately choose the lossy API.

### Optional IEEE binary16 / binary128

Current stable Rust 1.98.1 still treats native `f16` and `f128` as experimental, so Perfectπ does not make nightly Rust part of its production portability contract. Current nightly is nevertheless tested continuously, and its native `f16` / `f128` π-family bit patterns are required to match Perfectπ. Until those primitives reach stable, optional features expose exact IEEE interchange bits:

```text
cargo build --features binary16
cargo build --features binary128
cargo build --features all-float-formats
```

```rust
use perfect_pi::{PI_BINARY16, PI_BINARY128};

let half_bits: u16 = PI_BINARY16.to_bits();
let quad_bits: u128 = PI_BINARY128.to_bits();
```

`Binary16` and `Binary128` are transparent bit-format wrappers, not software arithmetic types. They add no dependencies or allocation, and the default feature set includes neither adapter. `PI_F16` and `PI_F128` are intentionally reserved for a future native-Rust adapter if those primitives become stable.

### Optional interoperability

Perfectπ's default dependency graph remains empty. Interoperability is opt-in:

```text
cargo build --features complex
cargo build --features fixed-point
cargo build --features decimal
cargo build --features interop
```

Current latest-stable integrations are:

- `num-complex 0.4.6`;
- `fixed 1.31.0`;
- `rust_decimal 1.43.0`.

Complex adapters reuse Perfectπ's existing checked/lossy native-float contracts. Fixed-point conversion is explicitly nearest-even and does not detour through binary floating point. `rust_decimal` conversion is exact through 28 decimal places; wider bounded values require the explicitly rounded nearest-even path.

See [Optional Interoperability](docs/INTEROPERABILITY.md).

### Optional runtime / arbitrary precision

Precision beyond the bounded `0..=40` tier is opt-in:

```text
cargo run --example runtime_generate --features runtime-generation -- 256
cargo run --example runtime_generate --features runtime-generation -- 256 round
cargo build --features arbitrary-precision
cargo build --features full
```

`runtime-generation` uses current `num-bigint 0.5.1` with default features disabled. It computes Machin's identity with conservative arbitrary-precision integer bounds and increases guard precision until the requested decimal result is certified. All six public decimal rounding modes are explicit, with no binary-float detour. The limit-taking API rejects untrusted precision above a caller-selected ceiling before expensive generation or output mutation, while final output storage remains caller-owned. `arbitrary-precision` is the precision-above-40 alias; `full` enables every current production capability. The default dependency graph remains unchanged.

See [Runtime Generation and Arbitrary Precision](docs/RUNTIME_GENERATION.md). Engineering and bounded-output examples are under `examples/`.

## Resource philosophy

`PI_F32` and `PI_F64` are direct aliases of Rust `core` constants. `Pi<D>` is verified as a zero-sized type, while the current straightforward `DecimalPi<40>` materialization occupies 41 bytes (one integer byte plus 40 fractional digit bytes).

The production source holds exactly 41 fractional digits: 40 public digits plus the one additional digit required to round `D=40`. Independent verification computes at least 64 fractional digits using two separate algorithms, so audit depth does not become runtime data. Native conversion uses independently verified IEEE bit patterns only where a bounded decimal value differs from native π; higher precisions collapse directly to `PI_F32` or `PI_F64`.

Optional-format probes remain tiny: binary16 π+τ measures 16–24 bytes of text across the measured constrained targets, while binary128 π+τ measures 62–80 bytes, with 0 measured rodata in those probes. Reproducible reports now cover constrained-target text/rodata/writable sections, static instruction counts, compiler-emitted stack frames, host linked-size deltas, and host timing. Target-hardware transitive stack high-water, WCET/cycles, and power remain hardware-qualification work. See [Measurements](docs/MEASUREMENTS.md), [Benchmark Report](docs/BENCHMARKS.md), and [Resource Budget](docs/RESOURCE_BUDGET.md).

## Reliability scope

Perfectπ is being designed for use in high-reliability software, including resource-constrained and fault-aware systems. That does **not** by itself make a downstream system safety-certified, radiation-hardened, or immune to hardware, compiler, memory, power, or integration faults.

See [Universal Support](docs/UNIVERSAL_SUPPORT.md), [Safety and Reliability](docs/SAFETY.md), and the [Critical-System Qualification Guide](docs/QUALIFICATION.md). Universal support means a common, verified library architecture; it does not mean every downstream device or safety case is pre-certified.

## Verification strategy

The bounded precision space is intentionally finite. Every supported decimal precision from `Pi<0>` through `Pi<40>` is covered by known-answer tests. All 164 bounded conversion outcomes (41 precisions × truncation/rounding × `f32`/`f64`) are checked against IEEE bit patterns independently generated from exact rational arithmetic, and canonical π digits are cross-verified with independent algorithms.

Phase-4 hardening now adds exhaustive bounded-state buffer verification, sanitizer-backed libFuzzer targets, mutation testing with zero surviving viable mutants, enforced source-coverage floors, and byte-for-byte cross-host reproducibility checks for 30 representative target/probe objects.

Verification dependencies never become runtime dependencies of the critical core. See [Phase 4 Verification Hardening](docs/PHASE4_VERIFICATION.md) for the retained evidence and methodology.

## Branding

The official Perfectπ visual identity is a geometric **π** inside a segmented bounded-precision ring. Brand assets are maintained in [`docs/assets/brand`](docs/assets/brand), with usage rules in [Branding](docs/BRANDING.md).

Primary repository assets:

- GitHub avatar / square mark: `docs/assets/brand/perfect-pi-icon-dark-512.png`
- Vector master: `docs/assets/brand/perfect-pi-icon-dark.svg`
- Transparent vector: `docs/assets/brand/perfect-pi-icon-transparent.svg`
- Monochrome vector: `docs/assets/brand/perfect-pi-icon-monochrome.svg`
- Wordmark: `docs/assets/brand/perfect-pi-wordmark-dark.svg`
- GitHub social preview: `docs/assets/brand/perfect-pi-github-social-preview-1280x640.png`

## Project documents

- [Normative Specification v1](SPECIFICATION.md)
- [Universal Support Contract](docs/UNIVERSAL_SUPPORT.md)
- [Universal 1.0 Traceability](docs/TRACEABILITY.md)
- [Critical-System Qualification Guide](docs/QUALIFICATION.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Precision and Numerical Semantics](docs/PRECISION.md)
- [Optional IEEE Float Formats](docs/FLOAT_FORMATS.md)
- [Optional Interoperability](docs/INTEROPERABILITY.md)
- [Runtime Generation and Arbitrary Precision](docs/RUNTIME_GENERATION.md)
- [API Stability and Compatibility](docs/API_STABILITY.md)
- [Rust and Dependency Currency Policy](docs/RUST_POLICY.md)
- [Safety and Reliability](docs/SAFETY.md)
- [Resource Budget](docs/RESOURCE_BUDGET.md)
- [Resource Measurements](docs/MEASUREMENTS.md)
- [Benchmark Report](docs/BENCHMARKS.md)
- [Startup Integrity Self-Test Evaluation](docs/INTEGRITY_SELF_TEST.md)
- [Domain Scope](docs/DOMAIN_SCOPE.md)
- [Verification Strategy](docs/VERIFICATION.md)
- [Phase 4 Verification Hardening](docs/PHASE4_VERIFICATION.md)
- [Release-Candidate Critic Review](docs/FINAL_REVIEW.md)
- [Release Criteria](docs/RELEASE_CRITERIA.md)
- [Roadmap](ROADMAP.md)
- [Support](SUPPORT.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)
- [Branding](docs/BRANDING.md)

## Contributing

Perfectπ accepts community participation and implementation contributions. Copyrightable contributions require agreement to the [Perfectπ Contributor Agreement](CONTRIBUTOR_AGREEMENT.md) and the pull-request attestation described in [CONTRIBUTING.md](CONTRIBUTING.md). Contributors retain ownership while granting the project owner the rights required to maintain the dual-licensing model.

## Rights and licensing

**Copyright © 2026 DrTomLLC.**

Perfectπ is source-available under a dual-licensing model. Community and noncommercial use, modification, and redistribution are licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE). Commercial use is encouraged but requires a separate paid commercial license from DrTomLLC; no fixed royalty percentage is imposed by the repository because commercial scope and consideration are agreed for the specific deployment.

See [RIGHTS.md](RIGHTS.md) and [Commercial Licensing](COMMERCIAL_LICENSING.md). The Cargo package remains `publish = false` until a deliberate package-release action is approved.

---

**Perfectπ** — the smallest correct π representation for the job, with heavier precision available only when deliberately requested.
