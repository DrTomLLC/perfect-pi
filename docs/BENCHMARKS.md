# Benchmark Report

This report records host measurements for Perfectπ. It is evidence for the measured host only; it is **not** a cycle-count or WCET claim for embedded targets.

## Environment

- CPU: AMD Ryzen AI 9 365 with Radeon 880M
- OS: Windows x86-64
- Rust: stable 1.98.1
- production source measured: `f11b0ab21b5e820777afd5f0b04b6732b9dbc349`
- profile: Cargo `--release`
- harness: `examples/host_benchmark.rs`
- timing source: `std::time::Instant`
- anti-elision boundary: `std::hint::black_box`
- samples: five post-build process runs on 2026-09-28; values below are medians

## Host timing medians

| Operation | Iterations per run | Median ns/op | Interpretation |
| --- | ---: | ---: | --- |
| native `PI_F64.to_bits()` | 20,000,000 | 0.233 | compiler-optimized constant path; timer/loop microbenchmark |
| `Pi<40>::round_nearest_even()` | 5,000,000 | 0.452 | compiler-optimized bounded materialization |
| rounded `Pi<40>` → lossy `f64` | 5,000,000 | 0.226 | optimized finite-table/native-collapse path |
| runtime generation, 100 fractional places | 100 | 26,594 | about 26.594 µs per generated value |
| runtime generation, 1,000 fractional places | 10 | 1,024,520 | about 1.025 ms per generated value |

The sub-nanosecond core figures must not be generalized to other CPUs or used as hardware latency claims. They mainly demonstrate that the optimized native/bounded paths are tiny relative to host timer-scale work. Runtime generation has deliberately variable cost and should be benchmarked at the precision actually used by an application.

## Reproduce

```text
python scripts/measure_host_timing.py
```

The script builds the release benchmark once, runs the resulting executable five times, and prints both the five samples and their median for every operation. `cargo run --release --example host_benchmark --features runtime-generation` remains useful for a single raw run. Do not mix compile time or process startup with the internally measured operation intervals.

## Linked x86-64 Windows size probe

Three examples were built with release optimization, fat LTO, one codegen unit, and symbol stripping. `llvm-size -A` measured linked PE sections.

| Probe | `.text` | `.rdata` | `.data` | Total measured sections | Delta vs baseline |
| --- | ---: | ---: | ---: | ---: | ---: |
| baseline | 78,403 | 27,110 | 512 | 109,449 | — |
| native `PI_F64` bits | 78,355 | 27,110 | 512 | 109,401 | -48 B layout variation |
| bounded round-40 + ASCII | 78,659 | 27,110 | 512 | 109,705 | +256 B |

The native probe's 48-byte reduction is treated as linker/layout variation, not a claim that Perfectπ makes a program smaller. At this measurement granularity, native π adds no measurable positive linked-size cost. The bounded forced-use probe adds 256 bytes of `.text`; `.rdata` and `.data` are unchanged from baseline.

## Limits

These measurements do not establish embedded WCET, cycle counts, interrupt-driven stack high-water, power impact, cache behavior, or final firmware deltas. Those require target-specific hardware/toolchain integration and must be reported separately when measured.


## Certified Chudnovsky runtime backend — 2026-09-30

The Universal 1.0 acceptance campaign exposed a high-precision scaling defect in the original Machin-series runtime backend. Production runtime generation was replaced with certified Chudnovsky binary splitting while retaining the same public APIs, six rounding modes, caller-owned output contract, precision-limit behavior, and `num-bigint 0.5.1` dependency.

The optimized production source was independently checked against the retained Machin reference in unit tests and against independent Chudnovsky plus Gauss–Legendre calculations through 10,000 fractional digits at ten checkpoints.

### Final host timing medians

Measured on the same Windows x86-64 / Ryzen AI 9 365 / Rust 1.98.1 host using the extended `scripts/measure_host_timing.py` harness:

| Operation | Iterations per run | Median ns/op | Approximate wall time |
| --- | ---: | ---: | ---: |
| native `PI_F64.to_bits()` | 20,000,000 | 0.229 | sub-ns optimized constant path |
| `Pi<40>::round_nearest_even()` | 5,000,000 | 0.437 | sub-ns bounded path |
| rounded `Pi<40>` → lossy `f64` | 5,000,000 | 0.218 | sub-ns conversion path |
| runtime generation, 100 places | 100 | 11,524 | 11.524 µs |
| runtime generation, 1,000 places | 10 | 114,670 | 0.115 ms |
| runtime generation, 10,000 places | 3 | 4,517,300 | 4.517 ms |
| runtime generation, 100,000 places | 1 | 182,949,800 | 182.950 ms |

Compared with the pre-optimization acceptance measurements, runtime generation improved by roughly 2.7× at 100 places, 10× at 1,000 places, 200× at 10,000 places, and about 1,900× at 100,000 places.

An isolated 100,000-place run produced 100,002 output bytes and the same FNV-1a digest (`6b16a6390067574d`) as the independently generated Dashu decimal result.

### Comparative high-precision context

The same acceptance campaign measured end-to-end decimal output at 100,000 places as approximately:

| Method | 100,000-place time |
| --- | ---: |
| Dashu | 102.459 ms |
| astro-float | 171.497 ms |
| Perfectπ certified Chudnovsky | 182.950 ms |

At 10,000 places Perfectπ measured 4.517 ms versus 6.583 ms for Dashu and 8.497 ms for astro-float on this host. These are host-specific measurements, not universal performance guarantees.

### Runtime footprint tradeoff

The optimized Windows runtime-generation linked probe measured 136,477 bytes across `.text`, `.rdata`, `.data`, and `.bss`, or +37,304 bytes over the 99,173-byte baseline. The prior Machin runtime probe added +34,896 bytes, so the speedup costs about 2.4 KiB of additional measured linked sections.

At a 10,000-place workload, the optimized runtime probe peaked at about 4.53 MB working set and 1.25 MB private memory, improved from approximately 6.00 MB / 2.55 MB for the prior backend. The bounded tier remains effectively at process-baseline memory.

The high-precision backend therefore trades a small linked-size increase for orders-of-magnitude better scaling and lower measured runtime memory while leaving the dependency-free native/bounded tiers unchanged.
