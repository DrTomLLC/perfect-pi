# Perfectπ

<p align="center">
  <img src="docs/assets/brand/perfect-pi-icon-dark-512.png" alt="Perfectπ logo" width="160">
</p>

**Perfect Pi** — deterministic, bounded, resource-efficient π infrastructure for Rust.

> One π ecosystem from tiny `no_std` targets to high-precision scientific computing, while making every expensive capability explicit and opt-in.

## Status

**Initial bounded core implemented and verified; minimum supported Rust is the current stable release, Rust 1.98.1; no public crate release yet.**

The repository now contains a working `no_std` bounded core: native `f32` / `f64` constants, `Pi<D>` support for every `D = 0..=40`, explicit truncation and round-to-nearest-even behavior, checked and explicitly lossy native-float conversions, allocation-free caller-buffer output, and independent canonical/conversion verification. It is still pre-release and must not yet be treated as a production-validated or safety-certified dependency.

## Why Perfectπ

Rust already provides π for native floating-point types, and arbitrary-precision crates can calculate far more digits than physical systems normally need. Perfectπ is designed to fill the space between those extremes:

- native `f32` / `f64` fast paths with effectively no library overhead;
- a bounded, deterministic decimal precision model from 0 through **40 places after the decimal point**;
- `#![no_std]` and no-allocation operation for the critical bounded core;
- explicit rounding plus checked precision-preserving and explicitly named lossy conversions — never silent precision claims;
- fixed and documented memory / execution bounds;
- optional stable IEEE `binary16` / `binary128` interchange adapters, plus future interoperability, calculation, and arbitrary-precision layers;
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

## Resource philosophy

`PI_F32` and `PI_F64` are direct aliases of Rust `core` constants. `Pi<D>` is verified as a zero-sized type, while the current straightforward `DecimalPi<40>` materialization occupies 41 bytes (one integer byte plus 40 fractional digit bytes).

The production source holds exactly 41 fractional digits: 40 public digits plus the one additional digit required to round `D=40`. Independent verification computes at least 64 fractional digits using two separate algorithms, so audit depth does not become runtime data. Native conversion uses independently verified IEEE bit patterns only where a bounded decimal value differs from native π; higher precisions collapse directly to `PI_F32` or `PI_F64`.

Optional-format probes remain tiny: binary16 π+τ measures 16–24 bytes of text across the measured constrained targets, while binary128 π+τ measures 62–80 bytes, with 0 measured rodata in those probes. Object-level Cortex-M/RISC-V measurements are published, while stack, WCET/cycle, power, and final linked-binary claims remain pending. See [Resource Budget](docs/RESOURCE_BUDGET.md).

## Reliability scope

Perfectπ is being designed for use in high-reliability software, including resource-constrained and fault-aware systems. That does **not** by itself make a downstream system safety-certified, radiation-hardened, or immune to hardware, compiler, memory, power, or integration faults.

See [Safety and Reliability](docs/SAFETY.md).

## Verification strategy

The bounded precision space is intentionally finite. Every supported decimal precision from `Pi<0>` through `Pi<40>` is covered by known-answer tests. All 164 bounded conversion outcomes (41 precisions × truncation/rounding × `f32`/`f64`) are checked against IEEE bit patterns independently generated from exact rational arithmetic, and canonical π digits are cross-verified with independent algorithms.

Verification dependencies will never become runtime dependencies of the critical core.

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
- [Architecture](docs/ARCHITECTURE.md)
- [Precision and Numerical Semantics](docs/PRECISION.md)
- [Optional IEEE Float Formats](docs/FLOAT_FORMATS.md)
- [Rust and Dependency Currency Policy](docs/RUST_POLICY.md)
- [Safety and Reliability](docs/SAFETY.md)
- [Resource Budget](docs/RESOURCE_BUDGET.md)
- [Initial Resource Measurements](docs/MEASUREMENTS.md)
- [Domain Scope](docs/DOMAIN_SCOPE.md)
- [Verification Strategy](docs/VERIFICATION.md)
- [Release Criteria](docs/RELEASE_CRITERIA.md)
- [Roadmap](ROADMAP.md)
- [Support](SUPPORT.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)
- [Branding](docs/BRANDING.md)

## Contributing

Perfectπ is currently **not accepting external code contributions** while ownership, licensing, and contributor-rights policy remain intentionally undecided. Design discussion, bug reports, numerical-correctness reports, and feature proposals are welcome.

Read [CONTRIBUTING.md](CONTRIBUTING.md) before participating.

## Rights and licensing

**Copyright © 2026 DrTomLLC. All rights reserved.**

Perfectπ is publicly viewable, but it is **not currently open source and no software license is granted**. No permission to copy, modify, redistribute, sublicense, publish, sell, or create derivative works should be inferred from public availability. Any permissions required solely for GitHub to host and display the repository are governed by GitHub's platform terms.

No patent, trademark, or branding rights are granted. Licensing and contributor-rights policy will be decided deliberately before any release that grants broader reuse rights.

See [RIGHTS.md](RIGHTS.md).

---

**Perfectπ** — the smallest correct π representation for the job, with heavier precision available only when deliberately requested.
