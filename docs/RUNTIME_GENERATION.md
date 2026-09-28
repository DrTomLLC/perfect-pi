# Runtime Generation and Arbitrary Precision

Perfectπ keeps runtime generation outside the bounded core. Enable `runtime-generation` for variable-cost computation or `arbitrary-precision` as the semantic alias for precision above 40 decimal places.

## Contract

- The default feature set remains dependency-free.
- The feature uses `num-bigint 0.5.1` with default features disabled.
- The crate remains `no_std`; the optional big-integer path requires allocation support from the downstream environment when executed.
- `generate_pi_ascii(decimal_places, output)` writes truncated decimal π into caller-provided output storage.
- `generate_pi_ascii_round_nearest_even(decimal_places, output)` provides explicit decimal nearest-even rounding.
- Output is `3` at zero places and `3.<digits>` otherwise.
- Too-small output is rejected before output bytes are modified.
- Runtime and memory cost increase with requested precision.
- Callers must impose an application-appropriate upper bound before forwarding untrusted or externally supplied precision values; the runtime tier is not a fixed-cost parser and deliberately does not guess a universal limit.

## Numerical method

The generator uses Machin's identity, `π = 16 atan(1/5) - 4 atan(1/239)`, evaluated with arbitrary-precision integers. Every integer division contributes conservative lower and upper bounds. The implementation increases guard precision until both bounds prove the same requested decimal truncation.

This is intentionally different from assuming a fixed number of guard digits is sufficient.

## Verification

`tests/runtime_generation.rs` checks zero, bounded, 100-place, rounded, and failure behavior. `scripts/verify_runtime_generation.py` independently computes π with Chudnovsky and Gauss-Legendre and requires truncation and nearest-even generation to match both through 1,000 fractional digits at nine checkpoints.

## Example

```text
cargo run --example runtime_generate --features runtime-generation -- 256
cargo run --example runtime_generate --features runtime-generation -- 256 round
```

For `0..=40` places where runtime generation is unnecessary, prefer `Pi<D>` because it has fixed resource bounds and no allocation.
