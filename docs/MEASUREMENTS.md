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
- results are measured before final application linking, LTO, dead stripping, and target-specific firmware runtime integration.

## Object-level contribution

| Target | Probe | Text bytes | Read-only π digits | Target unwind/metadata relevant to loaded image* |
| --- | --- | ---: | ---: | ---: |
| Cortex-M0 `thumbv6m-none-eabi` | native f32+f64 | 32 | 0 | 16 ARM exidx |
| Cortex-M0 `thumbv6m-none-eabi` | bounded Pi<40> trunc+round | 162 | 41 | 32 ARM exidx |
| Cortex-M0 `thumbv6m-none-eabi` | Pi<40> conversion f32+f64 | 328 | 41 | 72 ARM exidx |
| Cortex-M0 `thumbv6m-none-eabi` | low-precision conversion tables | 492 | 41 | 88 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | native f32+f64 | 32 | 0 | 16 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | bounded Pi<40> trunc+round | 158 | 41 | 32 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | Pi<40> conversion f32+f64 | 340 | 41 | 72 ARM exidx |
| Cortex-M `thumbv7em-none-eabihf` | low-precision conversion tables | 444 | 41 | 88 ARM exidx |
| RISC-V `riscv32imac-unknown-none-elf` | native f32+f64 | 28 | 0 | 0 |
| RISC-V `riscv32imac-unknown-none-elf` | bounded Pi<40> trunc+round | 250 | 41 | 0 |
| RISC-V `riscv32imac-unknown-none-elf` | Pi<40> conversion f32+f64 | 440 | 41 | 0 |
| RISC-V `riscv32imac-unknown-none-elf` | low-precision conversion tables | 694 | 41 | 0 |

`*` Object files also contain non-runtime bookkeeping sections such as compiler comments and architecture attributes. Those are intentionally excluded from the text/π-data columns.

## Type storage

- `Pi<0>`: 0 bytes in the tested builds;
- `Pi<40>`: 0 bytes in the tested builds;
- `DecimalPi<0>`: 1 byte;
- `DecimalPi<40>`: 41 bytes.

## Interpretation

The native path does not pull the 41-byte decimal digit table into the forced native probe. A target that only needs native π therefore does not pay for bounded decimal data in this measurement.

The bounded representation deliberately stores one decimal digit per byte. Packing could reduce the 41-byte materialized value, but would add decode logic and verification complexity. Perfectπ will not make that trade without measured evidence that it is beneficial on a real target.

The conversion implementation uses compact independently verified IEEE bit tables only for bounded values whose native result differs from the corresponding native π constant. The `conversion40` probe therefore measures the high-precision/native-collapse path, while `conversion_low` measures the distinct low-precision table-backed path. These are object-level forced-use probes; final linked applications may remove additional code/data through LTO and section garbage collection.

The conversion implementation was deliberately reduced from an exact generic fixed-width rational converter after measurement showed roughly 0.9-1.1 KiB of forced conversion text on constrained targets. The finite-domain verified lookup/native-constant design lowers the measured conversion probes to 328-492 bytes of text across the tested Cortex-M paths and 440-694 bytes across the tested RISC-V paths while preserving identical independently verified IEEE results.

## Still required

These measurements do not yet establish:

- final linked application/firmware delta with LTO and section garbage collection;
- worst-case stack usage;
- worst-case execution time;
- cycle counts;
- power impact;
- software-float versus hardware-float arithmetic costs;
- cross-compiler reproducibility of machine code.

Those remain Phase 4 work before production-readiness claims.
