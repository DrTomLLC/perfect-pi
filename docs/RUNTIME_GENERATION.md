# Runtime Generation and Arbitrary Precision

Perfectπ keeps runtime generation outside the bounded core. Enable `runtime-generation` for variable-cost computation or `arbitrary-precision` as the semantic alias for precision above 40 decimal places.

## Contract

- The default feature set remains dependency-free.
- The feature uses `num-bigint 0.5.1` with default features disabled.
- The crate remains `no_std`; the optional big-integer path requires allocation support from the downstream environment when executed.
- `generate_pi_ascii(decimal_places, output)` writes truncated decimal π into caller-provided output storage.
- `generate_pi_ascii_round_nearest_even(decimal_places, output)` retains the explicit nearest-even compatibility API.
- `generate_pi_ascii_with_rounding(decimal_places, rounding, output)` supports all six public `RoundingMode` policies.
- `generate_pi_ascii_with_limit(decimal_places, max_decimal_places, rounding, output)` rejects requests above a caller-selected ceiling before expensive generation or output mutation.
- Perfectπ keeps final output storage caller-owned; the runtime implementation may allocate internally through its opt-in big-integer dependency, but production source does not expose a direct allocation convenience.
- Output is `3` at zero places and `3.<digits>` otherwise.
- Too-small output is rejected before output bytes are modified.
- Runtime and memory cost increase with requested precision.
- Callers forwarding untrusted or externally supplied precision values must use an application-appropriate upper bound. The limit-taking APIs enforce this at the library boundary; Perfectπ deliberately does not guess a universal maximum that would be unsuitable across different machines.

## Numerical method

The production generator uses the Chudnovsky series with recursive binary splitting over arbitrary-precision integers. Consecutive partial sums of the alternating, monotonically decreasing series bracket the reciprocal-π series, and an integer square-root bound brackets `sqrt(10005)`. Perfectπ combines those rational intervals and increases guard precision until both bounds prove the same requested decimal truncation.

The previous certified Machin implementation remains in the test configuration as an algorithmically independent reference. Unit tests compare the production Chudnovsky result against Machin at representative precisions through 1,000 places, while the external verifier independently checks Chudnovsky and Gauss-Legendre references.

This is intentionally different from assuming a fixed number of guard digits is sufficient.

## Verification

`tests/runtime_generation.rs` and `tests/runtime_rounding.rs` check zero, bounded, 100-place, six-mode rounding, precision-limit, and failure behavior. `scripts/verify_runtime_generation.py` independently computes π with Chudnovsky and Gauss-Legendre and requires all six public runtime rounding modes to match the independent reference through 10,000 fractional digits at ten checkpoints. The runtime libFuzzer target mutates precision, rounding policy, output capacity, and caller limits under AddressSanitizer.

## Example

```text
cargo run --example runtime_generate --features runtime-generation -- 256 trunc
cargo run --example runtime_generate --features runtime-generation -- 256 ceil
cargo run --example runtime_generate --features runtime-generation -- 256 nearest-even
```

For `0..=40` places where runtime generation is unnecessary, prefer `Pi<D>` because it has fixed resource bounds and no allocation.
