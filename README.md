# Perfectπ

<p align="center">
  <img src="docs/assets/brand/perfect-pi-icon-dark-512.png" alt="Perfectπ logo" width="160">
</p>

**Perfect Pi** — deterministic, bounded, resource-efficient π infrastructure for Rust.

> One π ecosystem from tiny `no_std` targets to high-precision scientific computing, while making every expensive capability explicit and opt-in.

## Status

**Initial bounded core implemented and locally verified; MSRV Rust 1.85; no public crate release yet.**

The repository now contains the first working `no_std` core: native `f32` / `f64` constants, bounded `Pi<D>` support for every `D = 0..=40`, explicit truncation and round-to-nearest-even behavior, allocation-free caller-buffer output, and independent canonical-digit verification. It is still pre-release and must not yet be treated as a production-validated or safety-certified dependency.

## Why Perfectπ

Rust already provides π for native floating-point types, and arbitrary-precision crates can calculate far more digits than physical systems normally need. Perfectπ is designed to fill the space between those extremes:

- native `f32` / `f64` fast paths with effectively no library overhead;
- a bounded, deterministic decimal precision model from 0 through **40 places after the decimal point**;
- `#![no_std]` and no-allocation operation for the critical bounded core;
- explicit rounding semantics and a specification requiring explicit handling of future lossy conversions — never silent precision claims;
- fixed and documented memory / execution bounds;
- optional `f16`, `f128`, interoperability, calculation, and arbitrary-precision layers;
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
| Optional `f16` / `f128` adapters | target/toolchain-specific float support | specialized hardware and extended numerical work |
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
│   ├── f16
│   └── f128
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

`Pi<41>` cannot use the bounded-value operations. Future conversions that can lose precision are required by the normative specification to be checked or explicitly named as lossy.

## Resource philosophy

`PI_F32` and `PI_F64` are direct aliases of Rust `core` constants. `Pi<D>` is verified as a zero-sized type, while the current straightforward `DecimalPi<40>` materialization occupies 41 bytes (one integer byte plus 40 fractional digit bytes).

The production source holds exactly 41 fractional digits: 40 public digits plus the one additional digit required to round `D=40`. Independent verification computes at least 64 fractional digits using two separate algorithms, so audit depth does not become runtime data. Exact ROM, stack, instruction-count, timing, and linked-binary claims will be published only after representative measurements. See [Resource Budget](docs/RESOURCE_BUDGET.md).

## Reliability scope

Perfectπ is being designed for use in high-reliability software, including resource-constrained and fault-aware systems. That does **not** by itself make a downstream system safety-certified, radiation-hardened, or immune to hardware, compiler, memory, power, or integration faults.

See [Safety and Reliability](docs/SAFETY.md).

## Verification strategy

The bounded precision space is intentionally finite. Every supported decimal precision from `Pi<0>` through `Pi<40>` can be exhaustively covered by known-answer tests. Native representations will be checked by exact bit pattern, and canonical values will be cross-verified against independent references and algorithms.

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
