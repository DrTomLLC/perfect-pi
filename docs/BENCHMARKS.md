# Benchmark Report

This report records host measurements for Perfectπ. It is evidence for the measured host only; it is **not** a cycle-count or WCET claim for embedded targets.

## Environment

- CPU: AMD Ryzen AI 9 365 with Radeon 880M
- OS: Windows x86-64
- Rust: stable 1.98.1
- production runtime source measured: `a2c9adfdf2470f29062a979e9c4ae6407e014b7d` (subsequent production-source change is a final-newline-only formatting repair)
- profile: Cargo `--release`
- harness: `examples/host_benchmark.rs`
- timing source: `std::time::Instant`
- anti-elision boundary: `std::hint::black_box`
- samples: five post-build process runs on 2026-09-30; values below are medians

## Host timing medians

| Operation | Iterations per run | Median ns/op | Interpretation |
| --- | ---: | ---: | --- |
| native `PI_F64.to_bits()` | 20,000,000 | 0.284 | compiler-optimized constant path; timer/loop microbenchmark |
| `Pi<40>::round_nearest_even()` | 5,000,000 | 0.559 | compiler-optimized bounded materialization |
| rounded `Pi<40>` → lossy `f64` | 5,000,000 | 0.277 | optimized finite-table/native-collapse path |
| runtime generation, 100 fractional places | 100 | 14,221 | about 14.221 µs per generated value |
| runtime generation, 1,000 fractional places | 10 | 142,170 | about 142.170 µs per generated value |
| runtime generation, 10,000 fractional places | 5 | 5,343,420 | about 5.343 ms per generated value |
| runtime generation, 100,000 fractional places | 1 | 215,697,100 | about 215.697 ms per generated value |

The sub-nanosecond core figures must not be generalized to other CPUs or used as hardware latency claims. They mainly demonstrate that the optimized native/bounded paths are tiny relative to host timer-scale work. Runtime generation has deliberately variable cost and should be benchmarked at the precision actually used by an application.

## Runtime backend optimization

The Universal 1.0 acceptance benchmark exposed poor high-precision scaling in the former Machin-series production backend. Perfectπ replaced that production algorithm with certified Chudnovsky binary splitting while keeping the public API, caller-owned output, six rounding policies, precision-limit contract, and `num-bigint` dependency boundary unchanged.

Under the same external fat-LTO comparison harness used to identify the problem:

| Precision | Former Machin backend | Chudnovsky backend | Improvement |
| ---: | ---: | ---: | ---: |
| 10,000 fractional places | 920.951 ms | 4.716 ms | about 195× |
| 100,000 fractional places | 345.641 s | 204.105 ms | about 1,693× |

The 100,000-place Chudnovsky run wrote the expected 100,002-byte decimal result and retained the independently verified π prefix. The former Machin implementation remains test-only as an algorithmically independent cross-check rather than a production path.

The optimized 10,000-place process-memory probe measured 4,595,712 bytes peak working set and 1,421,312 bytes private memory, compared with 6,000,640 and 2,551,808 bytes for the former production backend in the same benchmark campaign.

The dedicated stripped runtime linked probe increased from a 34,896-byte measured-section delta over baseline to 37,560 bytes, a 2,664-byte increase. This is the measured code-size cost of the production algorithm change on that Windows x86-64 probe; the dependency set did not change.

## Reproduce

```text
python scripts/measure_host_timing.py
```

The script builds the release benchmark once, runs the resulting executable five times, and prints both the five samples and their median for every operation. It now includes 100, 1,000, 10,000, and 100,000 fractional-place runtime points. `cargo run --release --example host_benchmark --features runtime-generation` remains useful for a single raw run. Do not mix compile time or process startup with the internally measured operation intervals.

## Linked x86-64 Windows size probe

Three core examples were built with release optimization, fat LTO, one codegen unit, and symbol stripping. `llvm-size -A` measured linked PE sections.

| Probe | `.text` | `.rdata` | `.data` | Total measured sections | Delta vs baseline |
| --- | ---: | ---: | ---: | ---: | ---: |
| baseline | 78,403 | 27,110 | 512 | 109,449 | — |
| native `PI_F64` bits | 78,355 | 27,110 | 512 | 109,401 | -48 B layout variation |
| bounded round-40 + ASCII | 78,659 | 27,110 | 512 | 109,705 | +256 B |

The native probe's 48-byte reduction is treated as linker/layout variation, not a claim that Perfectπ makes a program smaller. At this measurement granularity, native π adds no measurable positive linked-size cost. The bounded forced-use probe adds 256 bytes of `.text`; `.rdata` and `.data` are unchanged from baseline.

## Correctness gates attached to performance work

The optimized runtime backend is not accepted on timing evidence alone:

- all-feature tests and Clippy with warnings denied must pass;
- production Chudnovsky output is cross-checked against the retained Machin implementation through representative points up to 1,000 fractional places;
- `scripts/verify_runtime_generation.py` independently computes π with both Chudnovsky and Gauss-Legendre and checks all six public rounding modes through 10,000 fractional places at ten checkpoints;
- CI fuzz, mutation, coverage, portability, reproducibility, and security jobs remain required on the exact pull-request head.

## Limits

These measurements do not establish embedded WCET, cycle counts, interrupt-driven stack high-water, power impact, cache behavior, or final firmware deltas. Those require target-specific hardware/toolchain integration and must be reported separately when measured.
