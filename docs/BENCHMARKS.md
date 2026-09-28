# Benchmark Report

This report records host measurements for Perfectπ. It is evidence for the measured host only; it is **not** a cycle-count or WCET claim for embedded targets.

## Environment

- CPU: AMD Ryzen AI 9 365 with Radeon 880M
- OS: Windows x86-64
- Rust: stable 1.98.1
- profile: Cargo `--release`
- harness: `examples/host_benchmark.rs`
- timing source: `std::time::Instant`
- anti-elision boundary: `std::hint::black_box`
- samples: five post-build process runs on 2026-09-28; values below are medians

## Host timing medians

| Operation | Iterations per run | Median ns/op | Interpretation |
| --- | ---: | ---: | --- |
| native `PI_F64.to_bits()` | 20,000,000 | 0.359 | compiler-optimized constant path; timer/loop microbenchmark |
| `Pi<40>::round_nearest_even()` | 5,000,000 | 0.600 | compiler-optimized bounded materialization |
| rounded `Pi<40>` → lossy `f64` | 5,000,000 | 0.288 | optimized finite-table/native-collapse path |
| runtime generation, 100 fractional places | 100 | 31,471 | about 31.471 µs per generated value |
| runtime generation, 1,000 fractional places | 10 | 1,184,200 | about 1.184 ms per generated value |

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
