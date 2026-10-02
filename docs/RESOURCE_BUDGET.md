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
| optional `Binary16` | 2-byte value when materialized | constants only when referenced | bit/byte interchange only |
| optional `Binary128` | 16-byte value when materialized | constants only when referenced | bit/byte interchange only |
| optional complex interop | external type only when feature enabled | pulls `num-complex` only when requested | wraps verified native conversion |
| optional fixed-point interop | destination-defined | pulls `fixed` only when requested | decimal nearest-even parse |
| optional decimal interop | 16-byte `rust_decimal::Decimal` destination | pulls `rust_decimal` only when requested | exact to 28 places / explicit nearest-even above |
| optional runtime/arbitrary precision | no big-int working allocation through 10,000 places; variable above that | ~10 KB independently verified π prefix plus internal specialized arithmetic; no normal bigint dependency | direct copy/round through 10,000 places; variable certified arithmetic above that |

## Verified baseline measurements

The Universal 1.0 release candidate has established these concrete properties:

- `Pi<0>` and `Pi<40>` are zero-sized types on the tested Rust toolchains;
- `DecimalPi<0>` currently occupies 1 byte;
- `DecimalPi<40>` currently occupies 41 bytes;
- the bounded-core canonical table contains 41 fractional decimal digits as `u8` values (40 public + one rounding digit);
- the default feature set has zero normal Rust dependencies and does not import `alloc`;
- bare-metal library checks pass for `thumbv6m-none-eabi`, `thumbv7em-none-eabihf`, and `riscv32imac-unknown-none-elf`;
- library checks also pass for `wasm32-unknown-unknown` and `aarch64-unknown-linux-gnu`;
- forced `Pi<40>` conversion probes measure 394 bytes of text on Cortex-M0, 408 bytes on Cortex-M hardware-float, and 534 bytes on bare-metal RISC-V;
- table-backed low-precision conversion probes measure 524 bytes of text on Cortex-M0, 580 bytes on Cortex-M hardware-float, and 722 bytes on bare-metal RISC-V;
- optional binary16 π+τ probes measure 24 bytes of text on Cortex-M0, 20 bytes on Cortex-M hardware-float, and 16 bytes on bare-metal RISC-V, with 0 measured rodata;
- optional binary128 π+τ probes measure 80 bytes of text on Cortex-M0, 74 bytes on Cortex-M hardware-float, and 62 bytes on bare-metal RISC-V, with 0 measured rodata;
- enabling all optional float-format features adds no Rust dependency and continues to pass the existing `no_std` target matrix;
- enabling interoperability pulls only the explicitly selected latest-stable optional ecosystem dependencies, while the default dependency graph remains empty;
- runtime/arbitrary precision uses an independently verified 10,032-place read-only prefix through 10,000 requested places and internal specialized arithmetic above that; `num-bigint` is development-only, and production runtime/parallel dependency trees contain only Perfectπ;
- all 18 constrained-target object probes contain 0 bytes of measured `.data` and 0 bytes of measured `.bss`;
- static instruction counts are recorded for all constrained-target probes;
- compiler-emitted stack frames are measured for representative bounded/conversion paths on Cortex-M0, Cortex-M hardware-float, and RISC-V;
- a stripped fat-LTO Windows x86-64 linked probe records no measurable positive native-π size cost and a +256-byte `.text` delta for bounded round-40 + ASCII;
- fresh Rust 1.99.0 stripped Windows dynamic probes measure +36,880 bytes of sections for the specialized serial runtime with verified prefix and +59,532 bytes for `parallel-runtime` over a 104,814-byte baseline, versus +84,180 bytes for astro-float and +94,680 bytes for Dashu in the same probe methodology;
- at 1,000,000 requested fractional places, the exact-current-tree parallel runtime peaked at 34,504,704 bytes working set and 31,944,704 bytes private memory on the benchmark host, below the measured Dashu and astro-float working sets in the same campaign;
- host release timings are published through 1,000,000 requested fractional places, with serial and opt-in parallel runtime measurements separated.

These are verified software-side implementation facts. The checked/lossy bounded conversion layer remains dependency-free and allocation-free. Runtime arbitrary precision is explicitly opt-in: the verified-prefix path through 10,000 places uses no internal big-integer working allocation, while deeper certified arithmetic is allocation-backed. See [Measurements](MEASUREMENTS.md) and [Benchmark Report](BENCHMARKS.md).

## Remaining hardware qualification

- transitive stack high-water in a final firmware/application image;
- target-hardware WCET and cycle counts;
- power impact;
- end-to-end software-float versus hardware-float cost on selected hardware;
- final target-specific firmware/application deltas where a concrete board and linker script are available;
- continued `no_std` validation as the API grows.

## Optimization policy

Do not add compression, table indirection, generic duplication, dynamic allocation, or runtime calculation merely to optimize an unmeasured bottleneck.

Twenty or thirty saved bytes are not valuable if they materially increase code complexity or verification burden.
