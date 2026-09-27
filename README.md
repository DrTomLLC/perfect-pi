# Perfectπ

**Perfect Pi** — deterministic, bounded, resource-efficient π infrastructure for Rust.

> One π ecosystem from tiny `no_std` targets to high-precision scientific computing, while making every expensive capability explicit and opt-in.

## Status

**Architecture baseline defined; implementation not yet released.**

This repository currently defines the public contract, numerical model, safety goals, resource goals, and implementation roadmap. It must not yet be treated as a validated numerical dependency for production or safety-critical systems.

## Why Perfectπ

Rust already provides π for native floating-point types, and arbitrary-precision crates can calculate far more digits than physical systems normally need. Perfectπ is designed to fill the space between those extremes:

- native `f32` / `f64` fast paths with effectively no library overhead;
- a bounded, deterministic decimal precision model from 0 through **40 places after the decimal point**;
- `#![no_std]` and no-allocation operation for the critical bounded core;
- explicit rounding and explicit lossy conversions — never silent precision claims;
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

## Planned API direction

The following illustrates the intended ergonomics; it is **not yet released API**:

```rust
use perfect_pi::PI_F64;

let circumference = 2.0 * PI_F64 * radius;
```

Bounded precision will be explicit:

```rust
let p = perfect_pi::Pi::<40>::new();
```

Conversions that can lose precision will be explicitly named or checked rather than silently narrowing.

## Resource philosophy

The `PI_F32` / `PI_F64` path should compile down to essentially the cost of using the corresponding native constant. The bounded 40-place representation is expected to require only a few dozen bytes of read-only canonical data plus bounded fixed-width working storage.

Actual ROM, RAM, stack, instruction-count, and timing claims will be published only after they are measured on representative targets. See [Resource Budget](docs/RESOURCE_BUDGET.md).

## Reliability scope

Perfectπ is being designed for use in high-reliability software, including resource-constrained and fault-aware systems. That does **not** by itself make a downstream system safety-certified, radiation-hardened, or immune to hardware, compiler, memory, power, or integration faults.

See [Safety and Reliability](docs/SAFETY.md).

## Verification strategy

The bounded precision space is intentionally finite. Every supported decimal precision from `Pi<0>` through `Pi<40>` can be exhaustively covered by known-answer tests. Native representations will be checked by exact bit pattern, and canonical values will be cross-verified against independent references and algorithms.

Verification dependencies will never become runtime dependencies of the critical core.

## Project documents

- [Architecture](docs/ARCHITECTURE.md)
- [Precision and Numerical Semantics](docs/PRECISION.md)
- [Safety and Reliability](docs/SAFETY.md)
- [Resource Budget](docs/RESOURCE_BUDGET.md)
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
