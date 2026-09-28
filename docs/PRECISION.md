# Precision and Numerical Semantics

## Canonical reference

π begins:

```text
3.14159265358979323846264338327950288419716939937510...
```

The public bounded API is limited to **40 places after the decimal point**.

## Terminology

`Pi<D>` means `D` decimal places after the decimal separator. It does not mean `D` significant digits.

Perfectπ must distinguish mathematical precision from the precision of a concrete binary floating-point representation.

## Forty-place boundary

The first 40 fractional digits are:

```text
1415926535897932384626433832795028841971
```

The next digit is `6`. Therefore:

- truncation at 40 places gives `3.1415926535897932384626433832795028841971`;
- round-to-nearest at 40 places gives `3.1415926535897932384626433832795028841972`.

Perfectπ must never leave that distinction implicit.

## Native floating point

`f32`, `f64`, and any optional `f16` / `f128` support represent binary floating-point approximations. A long decimal string cannot be treated as proof that the binary value contains that many exact decimal places.

Native constants will therefore be documented using their representation properties and, where useful, their exact stored bit patterns and numerical error.

## Rounding

Every conversion or fixed-precision construction must define its rounding behavior. Truncation must be explicitly requested and must not masquerade as rounding.

Perfectπ exposes six conventional decimal policies through `RoundingMode`: toward zero, away from zero, toward negative infinity, toward positive infinity, nearest ties-to-even, and nearest ties-away-from-zero. Because π is positive and irrational, toward zero equals toward negative infinity, away from zero equals toward positive infinity, and an exact finite-decimal halfway tie is impossible. The two nearest tie policies therefore produce the same π result while remaining semantically distinct public policies.

## Native-float conversion

Perfectπ converts the finite stored decimal value, not an imagined higher-precision `f32` or `f64` value. Callers that want mathematical π directly in a native float should use `PI_F32` or `PI_F64`; converting a `DecimalPi<D>` instead preserves the semantics of that finite decimal source before binary rounding.

`to_f32_lossy()` and `to_f64_lossy()` explicitly permit decimal-place loss. Their results are the independently verified IEEE-754 round-to-nearest, ties-to-even encodings of the canonical finite decimal source.

`try_to_f32_preserving_places()` succeeds only for `D <= 6`; `try_to_f64_preserving_places()` succeeds only for `D <= 15`. For the bounded π domain, these are the maximum conservative guarantees for recovering every requested decimal place. `D=7` already contains an `f32` case that cannot preserve all seven places, and `D=16` already contains an `f64` case that cannot preserve all sixteen.

A checked preservation guarantee means the binary result lies within half of one unit in the source's last requested decimal place, so rounding that binary result back to `D` decimal places recovers the source value. Higher-precision sources must opt into the explicitly lossy methods.

## Error budgets

Documentation should let callers choose the lowest-cost representation that satisfies their required error bound. Higher precision is not automatically better when sensors, models, input data, or downstream arithmetic dominate the total error.
