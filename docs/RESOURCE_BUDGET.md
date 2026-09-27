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
| canonical ~50 fractional digits | 0 writable | only a few dozen bytes | no cost until used |
| bounded `Pi<40>` materialization | a few dozen bytes | fixed | bounded fixed-width work |

Exact numbers will be published only after implementation and measurement.

## Required measurements before release

- binary size deltas with link-time optimization;
- stack usage;
- writable RAM usage;
- read-only data footprint;
- instruction counts for key operations;
- latency and worst-case execution behavior;
- software-float versus hardware-float impact;
- representative x86-64, AArch64, Cortex-M, and RISC-V builds;
- `no_std` build validation.

## Optimization policy

Do not add compression, table indirection, generic duplication, dynamic allocation, or runtime calculation merely to optimize an unmeasured bottleneck.

Twenty or thirty saved bytes are not valuable if they materially increase code complexity or verification burden.
