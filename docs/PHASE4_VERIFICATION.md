# Phase 4 Verification Hardening

This document records the verification-hardening evidence for the bounded Perfectπ core and its optional numeric adapters. It does **not** make a certification claim and does not replace the separate resource/timing qualification work.

## Scope

The verification pass covers:

- bounded `Pi<D>` for every `D = 0..=40`;
- truncation and nearest-even decimal rounding;
- caller-buffer ASCII output and failure behavior;
- native `f32` / `f64` conversions;
- optional binary16 / binary128 representations;
- optional complex, fixed-point, and `rust_decimal` interoperability;
- current-stable and current-nightly toolchain compatibility;
- deterministic cross-target code generation.

## Exhaustive bounded-state model

`tests/finite_state_model.rs` checks the complete public bounded precision domain.

For every one of 41 precisions and both truncation/rounding modes it verifies the canonical decimal representation and exercises output buffers with capacities from 0 through 44 bytes.

That is 41 precisions × 2 value modes × 45 buffer capacities = **3,690 `write_ascii` buffer states**.

The model verifies exact digits and lengths, `Display` consistency, exact short-buffer errors, unchanged memory after failed writes, and that successful writes touch only the returned output range.

## Sanitizer-backed fuzzing

Three libFuzzer targets live under `fuzz/` and are isolated from the production dependency graph.

### `bounded_api`

Exercises arbitrary precision, truncation/rounding mode, caller-buffer capacity, native conversion, and checked-conversion boundaries.

A current corrected-tree WSL2/Linux AddressSanitizer campaign completed **25,000 executions with no crash, assertion failure, or AddressSanitizer finding**. The final report reached 314 coverage counters / 315 features. Corpus size is intentionally not treated as a stable metric because libFuzzer may reduce it differently between runs.

### `interop`

Exercises arbitrary bounded values through complex32/complex64, exact and rounded `rust_decimal`, and several fixed-point destinations.

A current corrected-tree WSL2/Linux AddressSanitizer campaign completed **25,000 executions with no crash, assertion failure, or AddressSanitizer finding**. The final report reached 2,172 coverage counters / 2,289 features. Corpus size is intentionally not treated as a stable metric.

### `runtime_generation`

Exercises opt-in arbitrary-precision generation across requested precisions 0..=512, caller-buffer failure/success boundaries, output immutability on failure, ASCII structure, untouched tail bytes, and deterministic repeated generation. A current corrected-tree WSL2/Linux AddressSanitizer campaign completed **10,000 executions with no crash, assertion failure, or AddressSanitizer finding**, reaching 80 coverage counters / 108 features. Corpus size is intentionally not treated as a stable metric.

Windows remains in ordinary host testing and reproducibility. Local Windows libFuzzer execution was not counted because the MSVC environment lacked the dynamic AddressSanitizer runtime. Sanitizer-backed fuzzing therefore runs on Linux/WSL and Linux CI.

CI runs **25,000 executions each** for the bounded and interoperability targets and **10,000 executions** for runtime generation on current-nightly Linux under AddressSanitizer.

## Mutation testing

The current corrected-tree WSL2/Linux `cargo-mutants 27.1.0` pass generated **586 mutants** across production code after two explicitly documented mathematically equivalent `bit_reverse_permute` mutations were excluded in `.cargo/mutants.toml`.

- **519 caught by the test suite**;
- **67 unviable** because the mutation could not compile/check;
- **0 missed**;
- **0 timed out**.

The two exclusions do not remove distinct behavior: replacing `index < reversed` with `index > reversed` performs the same disjoint bit-reversal transpositions from the opposite endpoint, while replacing it with `index <= reversed` only adds no-op self-swaps. Required Linux CI reruns the same full mutation command and requires zero missed and zero timed-out viable mutants.

## Source coverage

Measured with current `cargo-llvm-cov` and all features enabled:

| Metric | Measured |
| --- | ---: |
| Lines | **96.33%** |
| Regions | **92.43%** |
| Functions | **100.00%** |

Required CI floors are 92% lines, 91% regions, and 100% functions.

The remaining uncovered lines are defensive paths that legal Perfectπ constructors cannot reach. They remain in production rather than being removed merely to inflate coverage.

## Reproducibility

`verify_reproducibility.py` compiles six representative object probes for five targets: Cortex-M0, Cortex-M hardware-float, bare-metal RISC-V, WebAssembly, and AArch64 Linux.

The six probes are native f32/f64 constants, bounded `Pi<40>`, high-precision conversion, low-precision conversion, optional binary16, and optional binary128. This produces **30 target/probe objects**.

Under current stable Rust **1.99.0**, compiler commit `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`:

- all 30 objects were byte-identical across two clean Windows builds;
- all 30 objects were byte-identical across two clean Linux builds;
- all 30 Windows SHA-256 hashes matched the corresponding Linux hashes exactly.

The committed golden manifest is `verification/reproducibility/current-stable.json`. Current Rust 1.99.0 local evidence matches it independently on Windows and Linux; required host CI applies the same check on Linux, Windows, and macOS.

## Tool currency

Verification tooling follows the same current-software policy:

- `cargo-audit 0.22.2`;
- `cargo-fuzz 0.13.2`;
- `libfuzzer-sys 0.4.13`;
- `cargo-mutants 27.1.0`;
- `cargo-llvm-cov 0.9.1`.

The current local RustSec audit reports no known vulnerabilities in the resolved 21-crate all-feature dependency set. Required CI regenerates the lockfile and runs the same audit.

Policy scripts query crates.io and fail when a newer stable direct dependency or verification tool exists until Perfectπ is explicitly updated and revalidated.

## Resource and benchmark evidence now added

The repository now also retains reproducible software-side resource evidence:

- all 18 Cortex-M/RISC-V object probes report text, rodata, writable `.data`/`.bss`, and static instruction counts;
- all 18 measured probes contain 0 bytes of `.data` and 0 bytes of `.bss`;
- current-nightly `-Z emit-stack-sizes` probes report representative function-frame sizes on Cortex-M0, Cortex-M hardware-float, and bare-metal RISC-V;
- stripped fat-LTO Windows x86-64 linked probes report native and bounded forced-use section deltas;
- a dependency-free release benchmark harness records host timing medians for native, bounded, conversion, and runtime-generation paths;
- the optional runtime generator is independently checked against both Chudnovsky and Gauss-Legendre through 10,001 fractional digits at eleven checkpoints, including the 10,000/10,001 fast-path handoff.

See [Initial Resource Measurements](MEASUREMENTS.md) and [Benchmark Report](BENCHMARKS.md) for methods, exact results, reproduction commands, and limitations.

## Remaining hardware qualification

Software-side Phase 4 evidence is complete for the current repository scope. Hardware-specific qualification still requires a selected board/application and cannot be inferred from object files or workstation timings. Remaining target-specific work is transitive stack high-water, WCET/cycles, power, software-vs-hardware-float cost, and final firmware/application linked deltas where meaningful.

Those claims will be added only after measured hardware evidence exists.
