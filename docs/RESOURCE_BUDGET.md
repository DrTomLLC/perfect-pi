# Resource Budget

Perfectπ is explicitly designed for low-resource systems as well as workstations.

## Rule

**A target pays only for the representation and operations it actually uses.**

## Expected design envelope

These are architecture targets, not benchmark results:

| Capability | Persistent RAM target | Read-only data target | Runtime character |
| --- | ---: | ---: | --- |
| `PI_F32` | ~0 | one native constant | effectively native constant use |
| `PI_F64` | ~0 | one native constant | effectively native constant use |
| `Pi<D>` marker | 0 if zero-sized | none itself | compile-time metadata |
| 41 production fractional digits | 0 writable | 41 bytes before linker treatment | no cost until bounded path is used |
| bounded `Pi<40>` materialization | a few dozen bytes | fixed | bounded fixed-width work |
| bounded native conversion | no persistent allocation | finite verified tables/code only when used | bounded lookup/selection |

## Verified baseline measurements

The initial implementation has now established these concrete properties:

- `Pi<0>` and `Pi<40>` are zero-sized types on the tested Rust toolchains;
- `DecimalPi<0>` currently occupies 1 byte;
- `DecimalPi<40>` currently occupies 41 bytes;
- the production canonical table contains 41 fractional decimal digits as `u8` values (40 public + one rounding digit);
- the crate has zero normal Rust dependencies and does not import `alloc`;
- bare-metal library checks pass for `thumbv6m-none-eabi`, `thumbv7em-none-eabihf`, and `riscv32imac-unknown-none-elf`;
- library checks also pass for `wasm32-unknown-unknown` and `aarch64-unknown-linux-gnu`;
- forced `Pi<40>` conversion probes measure 328 bytes of text on Cortex-M0, 340 bytes on Cortex-M hardware-float, and 440 bytes on bare-metal RISC-V;
- table-backed low-precision conversion probes measure 492 bytes of text on Cortex-M0, 444 bytes on Cortex-M hardware-float, and 694 bytes on bare-metal RISC-V.

These are verified implementation facts, not yet complete linked-binary or worst-case timing measurements. The checked/lossy conversion layer remains dependency-free and allocation-free. See [Initial Resource Measurements](MEASUREMENTS.md) for object-level Cortex-M and RISC-V measurements, including conversion paths.

## Required measurements before release

- binary size deltas with link-time optimization;
- stack usage;
- writable RAM usage;
- read-only data footprint;
- instruction counts for key operations;
- latency and worst-case execution behavior;
- software-float versus hardware-float impact;
- final linked-size measurements on representative x86-64, AArch64, Cortex-M, and RISC-V applications;
- continued `no_std` validation as the API grows.

## Optimization policy

Do not add compression, table indirection, generic duplication, dynamic allocation, or runtime calculation merely to optimize an unmeasured bottleneck.

Twenty or thirty saved bytes are not valuable if they materially increase code complexity or verification burden.
