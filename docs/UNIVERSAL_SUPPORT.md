# Universal Support Contract

## Mission

Perfectπ is one canonical Rust π infrastructure crate from allocation-free bare-metal
software through ordinary applications and arbitrary-precision scientific work. A
caller selects the smallest capability tier that satisfies its requirements; it
should not need a second π crate merely to change precision, platform, or resource
model.

Universal support is an engineering compatibility goal, not a claim that one
library build is pre-certified for every downstream safety standard or device.

## Capability tiers

| Tier | Feature surface | Resource model | Intended use |
| --- | --- | --- | --- |
| Core | native constants, Pi<D>, DecimalPi<D>, RoundingMode | no_std, no allocator, dependency-free | bare metal, RTOS, deterministic/high-assurance code |
| IEEE | binary16, binary128 interchange | no allocation, dependency-free | wire/file formats and non-native IEEE widths |
| Ecosystem | complex, fixed-point, rust_decimal | opt-in dependencies | application/numerical interoperability |
| Precision | runtime-generation / arbitrary-precision | verified read-only prefix through 10k; allocation-backed internal arithmetic above 10k; zero normal bigint dependency | research, validation, high-precision computation |
| Performance | parallel-runtime | optional scoped host threading above a measured crossover | large high-precision host workloads |
| Full | full | all numerical capability tiers; parallel policy remains separate | hosts or applications that deliberately want every numerical feature |
| Assurance | Core plus retained verification evidence | application-qualified | safety/security/reliability cases |

The default feature set is empty. Enabling heavier tiers never changes the
dependency or allocation contract of a default build.

## Precision

The bounded tier supports exactly 0 through 40 places after the decimal point.
This finite domain is deliberately exhaustively testable and allocation-free.

The arbitrary-precision tier accepts a runtime precision limited by address
space, Perfectπ's internal integer arithmetic, execution resources, and caller policy.
For untrusted inputs, use a caller-selected maximum through
generate_pi_ascii_with_limit. Perfectπ does not invent a global maximum that
would be wrong for either a microcontroller or a workstation. Final output
storage remains caller-owned.

## Rounding

Rounding is explicit through RoundingMode:

- TowardZero;
- AwayFromZero;
- TowardNegativeInfinity;
- TowardPositiveInfinity;
- NearestTiesToEven;
- NearestTiesAwayFromZero.

π is positive and irrational. Therefore truncation equals rounding toward zero
and toward negative infinity; ceiling equals away from zero; and π can never be
an exact finite-decimal halfway tie. The two nearest tie policies consequently
produce the same π result while preserving conventional policy names.

Existing truncation and nearest-even APIs remain compatibility conveniences.

## Platform model

The dependency-free core contains no OS API, I/O, allocation, global
initialization, environment inspection, randomness, or unsafe code. Universal
1.0 CI checks the core across representative Rust target classes including:

- ARM Cortex-M v6/v7/v8, soft- and hardware-float;
- RISC-V 32-bit and 64-bit;
- x86 32-bit and x86-64;
- AArch64 and ARMv7 Linux;
- little- and big-endian PowerPC plus s390x;
- LoongArch64;
- WebAssembly unknown and WASI;
- Windows GNU;
- Android AArch64/x86-64;
- Apple macOS/iOS/iOS Simulator;
- FreeBSD, NetBSD, and illumos;
- glibc and musl Linux variants.

Selected bare-metal and general targets additionally run all-feature checks.
Host behavior is tested on Linux, Windows, and macOS.

A target listed by rustc but lacking a distributable standard/core component,
or an optional third-party dependency that does not support that target, is not
silently represented as verified. Such cases use the dependency-free core or
require target-specific qualification.

## Critical systems

Perfectπ is designed to be qualification-friendly. The bounded Core tier offers
the properties normally needed from a small numerical primitive: deterministic
semantics, no hidden allocation, bounded state, explicit conversions, explicit
rounding, no unsafe code, and retained verification evidence.

A library cannot pre-certify an aircraft, medical device, vehicle, reactor,
industrial controller, or other critical system. Qualification must bind the
exact Perfectπ commit, Rust compiler, target, feature set, linker/application,
hardware, fault model, and governing standard. See QUALIFICATION.md.

## Compatibility

A future stable 1.0 release freezes documented public semantics under SemVer.
Before 1.0 publication, incompatible changes still require deliberate
specification review. The Cargo package remains publish = false until the owner
explicitly authorizes a package/release action.

## Non-goals

Perfectπ does not become a general trigonometry, linear-algebra, units,
statistics, or domain-framework crate. It owns π, closely related constants,
their finite representations, conversions, rounding semantics, generation, and
the evidence needed to trust those results.
