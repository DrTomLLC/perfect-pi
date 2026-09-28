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

Two libFuzzer targets live under `fuzz/` and are isolated from the production dependency graph.

### `bounded_api`

Exercises arbitrary precision, truncation/rounding mode, caller-buffer capacity, native conversion, and checked-conversion boundaries.

A local WSL2/Linux AddressSanitizer campaign completed **20,000 executions with no crash or assertion failure**. The final report reached 345 coverage counters / 346 features with a 199-entry minimized corpus.

### `interop`

Exercises arbitrary bounded values through complex32/complex64, exact and rounded `rust_decimal`, and several fixed-point destinations.

A local WSL2/Linux AddressSanitizer campaign completed **20,000 executions with no crash or assertion failure**. The final report reached 2,172 coverage counters / 2,289 features with a 243-entry minimized corpus.

Windows remains in ordinary host testing and reproducibility. Local Windows libFuzzer execution was not counted because the MSVC environment lacked the dynamic AddressSanitizer runtime. Sanitizer-backed fuzzing therefore runs on Linux/WSL and Linux CI.

CI runs **25,000 executions per fuzz target** on current nightly Linux.

## Mutation testing

The current `cargo-mutants` pass generated **96 mutants** across production code.

- **84 caught by the test suite**;
- **12 unviable** because the mutation could not compile/check;
- **0 missed**;
- **0 timed out**.

Therefore every generated mutant that could be compiled and executed was detected by the tests.

## Source coverage

Measured with current `cargo-llvm-cov` and all features enabled:

| Metric | Measured |
| --- | ---: |
| Lines | **92.66%** |
| Regions | **91.48%** |
| Functions | **100.00%** |

Required CI floors are 92% lines, 91% regions, and 100% functions.

The remaining uncovered lines are defensive paths that legal Perfectπ constructors cannot reach. They remain in production rather than being removed merely to inflate coverage.

## Reproducibility

`verify_reproducibility.py` compiles six representative object probes for five targets: Cortex-M0, Cortex-M hardware-float, bare-metal RISC-V, WebAssembly, and AArch64 Linux.

The six probes are native f32/f64 constants, bounded `Pi<40>`, high-precision conversion, low-precision conversion, optional binary16, and optional binary128. This produces **30 target/probe objects**.

Under Rust **1.98.1**, compiler commit `48a229ceaefd4985c50990b14116b6d856af0985`:

- all 30 objects were byte-identical across two clean Windows builds;
- all 30 objects were byte-identical across two clean Linux builds;
- all 30 Windows SHA-256 hashes matched the corresponding Linux hashes exactly.

The committed golden manifest is `verification/reproducibility/current-stable.json`. Pre-merge local evidence matches it independently on Windows and Linux; required host CI applies the same check on Linux, Windows, and macOS.

## Tool currency

Verification tooling follows the same current-software policy:

- `cargo-audit 0.22.2`;
- `cargo-fuzz 0.13.2`;
- `libfuzzer-sys 0.4.13`;
- `cargo-mutants 27.1.0`;
- `cargo-llvm-cov 0.9.1`.

The current local RustSec audit reports no known vulnerabilities in the resolved 19-crate all-feature dependency set. Required CI regenerates the lockfile and runs the same audit.

Policy scripts query crates.io and fail when a newer stable direct dependency or verification tool exists until Perfectπ is explicitly updated and revalidated.

## Remaining Phase 4 work

This verification-hardening pass does not complete resource qualification. Remaining work includes final linked ROM/binary-size deltas, writable RAM and stack high-water measurements, instruction/cycle counts, WCET methodology, software-float versus hardware-float cost, benchmark methodology/reporting, and power impact where meaningful.

Those claims will be added only after measured evidence exists.
