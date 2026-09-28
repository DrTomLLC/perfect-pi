# Initial Resource Measurements

These measurements characterize the first Perfectπ core baseline. They are **object-level probes**, not final linked-firmware size claims.

## Reproduce

From the repository root:

```text
python scripts/measure_resources.py
```

The script uses only the Python standard library plus the installed Rust toolchain and `llvm-tools`. It obtains the exact `.rlib` path from Cargo's JSON artifact output rather than selecting files by timestamp, preventing stale build artifacts from contaminating measurements.

## Method

- Rust compiler: `rustc 1.98.1` for the recorded measurement pass;
- Perfectπ library: release build;
- probe compilation: `-C opt-level=z -C panic=abort`;
- measurement tool: LLVM `llvm-size -A`;
- native probe forces both `PI_F32` and `PI_F64` bit-return paths;
- bounded probe forces both `Pi<40>::truncated()` and `Pi<40>::round_nearest_even()`;
- `conversion40` forces truncated/rounded `Pi<40>` conversion to both `f32` and `f64`;
- `conversion_low` forces the highest distinct table-backed conversions at `D=7` for `f32` and `D=14` for `f64`;
- `binary16` forces optional `PI_BINARY16` and `TAU_BINARY16` bit-return paths with feature `binary16`;
- `binary128` forces optional `PI_BINARY128` and `TAU_BINARY128` bit-return paths with feature `binary128`;
- results are measured before final application linking, LTO, dead stripping, and target-specific firmware runtime integration.

## Object-level contribution

| Target | Probe | Text bytes | Read-only π digits | Target unwind/metadata relevant to loaded image* |
| --- | --- | ---: | ---: | ---: |
| Cortex-M0 `thumbv6m-none-eabi` | native f32+f64 | 32 | 0 | 16 ARM exidx |
| Cortex-M0 `thumbv6m-none-eabi` | bounded Pi<40> trunc+round | 162 | 41 | 32 ARM exidx |
| Cortex-M0 `thumbv6m-none-eabi` | Pi<40> conversion f32+f64 | 328 | 41 | 72 ARM exidx |
| Cortex-M0 `thumbv6m-none-eabi` | low-precision conversion tables | 386 | 41 | 88 ARM exidx |
| Cortex-M0 `thumbv6m-none-eabi` | optional binary16 π+τ | 24 | 0 | 16 ARM exidx |
| Cortex-M0 `thumbv6m-none-eabi` | optional binary128 π+τ | 80 | 0 | 16 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | native f32+f64 | 32 | 0 | 16 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | bounded Pi<40> trunc+round | 158 | 41 | 32 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | Pi<40> conversion f32+f64 | 340 | 41 | 72 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | low-precision conversion tables | 398 | 41 | 88 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | optional binary16 π+τ | 20 | 0 | 16 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | optional binary128 π+τ | 74 | 0 | 24 ARM exidx |
| RISC-V `riscv32imac-unknown-none-elf` | native f32+f64 | 28 | 0 | 0 |
| RISC-V `riscv32imac-unknown-none-elf` | bounded Pi<40> trunc+round | 250 | 41 | 0 |
| RISC-V `riscv32imac-unknown-none-elf` | Pi<40> conversion f32+f64 | 440 | 41 | 0 |
| RISC-V `riscv32imac-unknown-none-elf` | low-precision conversion tables | 534 | 41 | 0 |
| RISC-V `riscv32imac-unknown-none-elf` | optional binary16 π+τ | 16 | 0 | 0 |
| RISC-V `riscv32imac-unknown-none-elf` | optional binary128 π+τ | 62 | 0 | 0 |

`*` Object files also contain non-runtime bookkeeping sections such as compiler comments and architecture attributes. Those are intentionally excluded from the text/π-data columns.

## Writable memory and static instruction counts

The same exact probe objects are also inspected with `llvm-size` for writable sections and `llvm-objdump -d` for static instruction counts. All 18 probes measured **0 bytes of `.data` and 0 bytes of `.bss`**.

| Target | Probe | Instructions |
| --- | --- | ---: |
| Cortex-M0 `thumbv6m-none-eabi` | native f32+f64 | 13 |
| Cortex-M0 `thumbv6m-none-eabi` | bounded Pi<40> trunc+round | 74 |
| Cortex-M0 `thumbv6m-none-eabi` | Pi<40> conversion f32+f64 | 144 |
| Cortex-M0 `thumbv6m-none-eabi` | low-precision conversion tables | 171 |
| Cortex-M0 `thumbv6m-none-eabi` | optional binary16 π+τ | 10 |
| Cortex-M0 `thumbv6m-none-eabi` | optional binary128 π+τ | 32 |
| Cortex-M `thumbv7em-none-eabihf` | native f32+f64 | 13 |
| Cortex-M `thumbv7em-none-eabihf` | bounded Pi<40> trunc+round | 65 |
| Cortex-M `thumbv7em-none-eabihf` | Pi<40> conversion f32+f64 | 134 |
| Cortex-M `thumbv7em-none-eabihf` | low-precision conversion tables | 152 |
| Cortex-M `thumbv7em-none-eabihf` | optional binary16 π+τ | 8 |
| Cortex-M `thumbv7em-none-eabihf` | optional binary128 π+τ | 25 |
| RISC-V `riscv32imac-unknown-none-elf` | native f32+f64 | 8 |
| RISC-V `riscv32imac-unknown-none-elf` | bounded Pi<40> trunc+round | 87 |
| RISC-V `riscv32imac-unknown-none-elf` | Pi<40> conversion f32+f64 | 143 |
| RISC-V `riscv32imac-unknown-none-elf` | low-precision conversion tables | 188 |
| RISC-V `riscv32imac-unknown-none-elf` | optional binary16 π+τ | 6 |
| RISC-V `riscv32imac-unknown-none-elf` | optional binary128 π+τ | 18 |

These are static object instruction counts, not cycle counts or WCET. A single instruction can have target- and state-dependent latency, so the project does not convert these counts into timing claims without target-specific execution evidence.

## Type storage

- `Pi<0>`: 0 bytes in the tested builds;
- `Pi<40>`: 0 bytes in the tested builds;
- `DecimalPi<0>`: 1 byte;
- `DecimalPi<40>`: 41 bytes;
- `Binary16`: 2 bytes when the feature is enabled and a value is materialized;
- `Binary128`: 16 bytes when the feature is enabled and a value is materialized.

## Interpretation

The native path does not pull the 41-byte decimal digit table into the forced native probe. A target that only needs native π therefore does not pay for bounded decimal data in this measurement.

The bounded representation deliberately stores one decimal digit per byte. Packing could reduce the 41-byte materialized value, but would add decode logic and verification complexity. Perfectπ will not make that trade without measured evidence that it is beneficial on a real target.

The conversion implementation uses compact independently verified IEEE bit tables only for bounded values whose native result differs from the corresponding native π constant. The `conversion40` probe therefore measures the high-precision/native-collapse path, while `conversion_low` measures the distinct low-precision table-backed path. These are object-level forced-use probes; final linked applications may remove additional code/data through LTO and section garbage collection.

The conversion implementation was deliberately reduced from an exact generic fixed-width rational converter after measurement showed roughly 0.9-1.1 KiB of forced conversion text on constrained targets. The finite-domain verified lookup/native-constant design lowers the measured conversion probes to 328-398 bytes of text across the tested Cortex-M paths and 440-534 bytes across the tested RISC-V paths while preserving identical independently verified IEEE results.

The optional binary16/binary128 adapters are bit-format wrappers only; they do not emulate arithmetic. In the forced π+τ probes, binary16 contributes 16-24 bytes of text and binary128 contributes 62-80 bytes across the measured constrained targets, with 0 measured rodata. Because these are opt-in features and ordinary constants, unused adapters remain absent from the default build and may be further eliminated by final linking.

## Static stack-frame measurements

Current nightly `rustc 1.101.0-nightly (d080e7dff 2026-09-27)` with `-Z emit-stack-sizes` reports the following **individual probe-function frames**:

| Target | bounded round-40 + ASCII | round-40 → f64 |
| --- | ---: | ---: |
| `thumbv6m-none-eabi` | 92 B | 8 B |
| `thumbv7em-none-eabihf` | 8 B | 8 B |
| `riscv32imac-unknown-none-elf` | 48 B | 0 B |

Reproduce with `python scripts/measure_stack_frames.py`. These are compiler-emitted frame sizes, not transitive call-chain high-water marks.

## Linked host size and timing

A stripped fat-LTO Windows x86-64 linked probe measured a +256-byte `.text` delta for forced bounded round-40 + ASCII relative to the baseline, with unchanged `.rdata` and `.data`. The native constant probe showed no measurable positive linked-size cost. Reproduce with `python scripts/measure_linked_size.py`.

Host timing methodology and five-run medians are published in [Benchmark Report](BENCHMARKS.md).

## Still required for hardware qualification

The software-side Phase 4 measurements do not establish:

- transitive worst-case stack high-water on a final application/firmware image;
- target-hardware worst-case execution time and cycle counts;
- power impact;
- end-to-end software-float versus hardware-float arithmetic cost on selected hardware;
- cross-compiler reproducibility of final machine code.

Those require a specific MCU/board, clock/memory configuration, compiler/linker setup, and measurement method. Perfectπ does not infer those values from object bytes or host timing.
