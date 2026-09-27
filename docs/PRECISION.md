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

## Lossy conversion

Converting a higher-precision Perfectπ value into a lower-precision numeric type may lose information. Such conversions must be checked or explicitly named as lossy.

## Error budgets

Documentation should let callers choose the lowest-cost representation that satisfies their required error bound. Higher precision is not automatically better when sensors, models, input data, or downstream arithmetic dominate the total error.
