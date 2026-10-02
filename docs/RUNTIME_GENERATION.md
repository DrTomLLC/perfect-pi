# Runtime Generation and Arbitrary Precision

Perfectπ keeps runtime generation outside the bounded core. Enable `runtime-generation` for variable-cost computation or `arbitrary-precision` as the semantic alias for precision above 40 decimal places.

## Contract

- The default feature set remains dependency-free.
- Requests through 10,000 fractional places use an independently generated read-only π prefix and write directly into the caller buffer; deeper requests use Perfectπ's internal specialized arbitrary-precision integer engine. The feature adds no normal Rust dependency.
- The crate remains `no_std`; runtime generation requires allocation support from the downstream environment when executed.
- `generate_pi_ascii(decimal_places, output)` writes truncated decimal π into caller-provided output storage.
- `generate_pi_ascii_round_nearest_even(decimal_places, output)` retains the explicit nearest-even compatibility API.
- `generate_pi_ascii_with_rounding(decimal_places, rounding, output)` supports all six public `RoundingMode` policies.
- `generate_pi_ascii_with_limit(decimal_places, max_decimal_places, rounding, output)` rejects requests above a caller-selected ceiling before expensive generation or output mutation.
- Perfectπ keeps final output storage caller-owned; the runtime implementation allocates only internal working storage and does not expose a direct allocation convenience.
- Output is `3` at zero places and `3.<digits>` otherwise.
- Too-small output is rejected before output bytes are modified.
- Runtime and memory cost increase with requested precision.
- Callers forwarding untrusted or externally supplied precision values must use an application-appropriate upper bound. The limit-taking APIs enforce this at the library boundary; Perfectπ deliberately does not guess a universal maximum that would be unsuitable across different machines.

## Numerical method

For `0..=10_000` fractional places, the production generator uses a checked read-only decimal prefix generated only after independent Chudnovsky and Gauss-Legendre implementations agree. This fast path writes directly into caller-owned storage and applies the same six rounding policies without arbitrary-precision working allocation. `scripts/generate_runtime_prefix.py` deterministically regenerates or verifies the retained source data.

Above 10,000 places, the production generator uses the Chudnovsky series with binary splitting over Perfectπ's own specialized integer arithmetic. The internal engine uses base-`10^18` limbs, measured schoolbook/Karatsuba/NTT multiplication tiers, reciprocal-Newton division and square-root support, and exact certification checks. Consecutive partial sums of the alternating, monotonically decreasing Chudnovsky series bracket the exact reciprocal-π series. Perfectπ emits a requested decimal truncation only when the lower and upper π bounds prove the same scaled integer.

Guard precision begins conservatively and increases only if the proof interval still crosses a requested-scale integer boundary. Reciprocal-square-root certification may perform a bounded extra refinement when the estimate is not yet within the small exact-adjustment window; this closes precision-transition edge cases without penalizing the normal high-precision path.

The former Machin-identity implementation, `π = 16 atan(1/5) - 4 atan(1/239)`, and `num-bigint` are retained only under development/test as algorithmically independent references. Production Chudnovsky results are cross-checked against that Machin reference at representative precisions, while the external verifier also compares public output with independently implemented Chudnovsky and Gauss-Legendre references.

## Optional parallel runtime

`parallel-runtime` layers a host-performance policy on `runtime-generation` without changing numerical semantics or adding a production crate dependency. The verified-prefix path through 10,000 places does not create worker threads. For deeper arithmetic requests on supported non-WASM/non-bare-metal hosts, the current measured crossover is 512 Chudnovsky terms; those requests use scoped parallel binary splitting while constant bounds are computed concurrently. The `full` feature intentionally does not enable `parallel-runtime`; threading and its resource model remain an explicit deployment choice.

## Verification

`tests/runtime_generation.rs` and `tests/runtime_rounding.rs` check zero, bounded, 100-place, six-mode rounding, precision-limit, and failure behavior. `scripts/generate_runtime_prefix.py` independently regenerates and verifies the retained 10,032-place source prefix. `scripts/verify_runtime_generation.py` independently computes π with Chudnovsky and Gauss-Legendre and requires all six public runtime rounding modes to match through 10,001 fractional digits at eleven checkpoints, explicitly covering both the 10,000-place fast path and the 10,001-place arithmetic handoff. The runtime libFuzzer target mutates precision, rounding policy, output capacity, and caller limits under AddressSanitizer.

## Example

```text
cargo run --example runtime_generate --features runtime-generation -- 256 trunc
cargo run --example runtime_generate --features runtime-generation -- 256 ceil
cargo run --example runtime_generate --features runtime-generation -- 256 nearest-even
```

For `0..=40` places where runtime generation is unnecessary, prefer `Pi<D>` because it has fixed resource bounds and no allocation.
