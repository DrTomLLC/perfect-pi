#!/usr/bin/env python3
"""Independently verify optional IEEE binary16/binary128 Perfectπ constants."""

from decimal import Decimal, localcontext
from fractions import Fraction
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
BINARY16_RS = ROOT / "src" / "binary16.rs"
BINARY128_RS = ROOT / "src" / "binary128.rs"
PRECISION = 260

NAMES = (
    "PI",
    "TAU",
    "FRAC_PI_2",
    "FRAC_PI_3",
    "FRAC_PI_4",
    "FRAC_PI_6",
    "FRAC_PI_8",
    "INV_PI",
    "TWO_INV_PI",
    "TWO_INV_SQRT_PI",
)


def chudnovsky_pi() -> Decimal:
    with localcontext() as ctx:
        ctx.prec = PRECISION
        c = Decimal(426880) * Decimal(10005).sqrt()
        m = 1
        l = 13_591_409
        x = 1
        k = 6
        total = Decimal(l)

        for i in range(1, 22):
            m = (m * (k**3 - 16 * k)) // (i**3)
            l += 545_140_134
            x *= -262_537_412_640_768_000
            total += Decimal(m * l) / Decimal(x)
            k += 12

        return +(c / total)


def gauss_legendre_pi() -> Decimal:
    with localcontext() as ctx:
        ctx.prec = PRECISION
        one = Decimal(1)
        two = Decimal(2)
        four = Decimal(4)
        a = one
        b = one / two.sqrt()
        t = one / four
        p = one

        for _ in range(10):
            next_a = (a + b) / two
            b = (a * b).sqrt()
            t -= p * (a - next_a) ** 2
            a = next_a
            p *= two

        return +((a + b) ** 2 / (four * t))


def constants(pi_value: Decimal) -> dict[str, Decimal]:
    with localcontext() as ctx:
        ctx.prec = PRECISION
        return {
            "PI": +pi_value,
            "TAU": +(pi_value * 2),
            "FRAC_PI_2": +(pi_value / 2),
            "FRAC_PI_3": +(pi_value / 3),
            "FRAC_PI_4": +(pi_value / 4),
            "FRAC_PI_6": +(pi_value / 6),
            "FRAC_PI_8": +(pi_value / 8),
            "INV_PI": +(Decimal(1) / pi_value),
            "TWO_INV_PI": +(Decimal(2) / pi_value),
            "TWO_INV_SQRT_PI": +(Decimal(2) / pi_value.sqrt()),
        }


def ieee_bits(value: Fraction, precision_bits: int, exponent_bits: int, bias: int) -> int:
    if value <= 0:
        raise ValueError("verification only supports positive finite values")

    numerator = value.numerator
    denominator = value.denominator
    exponent = numerator.bit_length() - denominator.bit_length()

    if exponent >= 0:
        if numerator < denominator << exponent:
            exponent -= 1
    elif numerator << (-exponent) < denominator:
        exponent -= 1

    shift = (precision_bits - 1) - exponent
    if shift >= 0:
        scaled_numerator = numerator << shift
        scaled_denominator = denominator
    else:
        scaled_numerator = numerator
        scaled_denominator = denominator << (-shift)

    significand, remainder = divmod(scaled_numerator, scaled_denominator)
    doubled = remainder << 1
    if doubled > scaled_denominator or (
        doubled == scaled_denominator and significand & 1
    ):
        significand += 1

    if significand == 1 << precision_bits:
        significand >>= 1
        exponent += 1

    fraction_bits = precision_bits - 1
    fraction_field = significand - (1 << fraction_bits)
    exponent_field = exponent + bias

    if not 0 < exponent_field < (1 << exponent_bits) - 1:
        raise ValueError("verification value unexpectedly outside normal IEEE range")

    return (exponent_field << fraction_bits) | fraction_field


def parse_source(path: Path, suffix: str) -> dict[str, int]:
    text = path.read_text(encoding="utf-8")
    parsed: dict[str, int] = {}

    for name in NAMES:
        pattern = (
            rf"pub const {name}_{suffix}: [A-Za-z0-9]+\s*=\s*"
            rf"[A-Za-z0-9]+::from_bits\((0x[0-9a-fA-F_]+)\);"
        )
        match = re.search(pattern, text, re.DOTALL)
        if match is None:
            raise RuntimeError(f"{name}_{suffix} not found in {path}")
        parsed[name] = int(match.group(1).replace("_", ""), 16)

    return parsed


def main() -> int:
    first = constants(chudnovsky_pi())
    second = constants(gauss_legendre_pi())
    source16 = parse_source(BINARY16_RS, "BINARY16")
    source128 = parse_source(BINARY128_RS, "BINARY128")
    failures: list[str] = []

    for name in NAMES:
        first16 = ieee_bits(Fraction(first[name]), 11, 5, 15)
        second16 = ieee_bits(Fraction(second[name]), 11, 5, 15)
        first128 = ieee_bits(Fraction(first[name]), 113, 15, 16_383)
        second128 = ieee_bits(Fraction(second[name]), 113, 15, 16_383)

        if first16 != second16:
            failures.append(f"{name}: independent binary16 computations disagree")
        if first128 != second128:
            failures.append(f"{name}: independent binary128 computations disagree")
        if source16[name] != first16:
            failures.append(
                f"{name}_BINARY16: source=0x{source16[name]:04x}, expected=0x{first16:04x}"
            )
        if source128[name] != first128:
            failures.append(
                f"{name}_BINARY128: source=0x{source128[name]:032x}, "
                f"expected=0x{first128:032x}"
            )

    if failures:
        print("FAIL: optional float-format verification failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("PASS: Chudnovsky and Gauss-Legendre agree for all optional float constants")
    print("PASS: all 10 IEEE binary16 constants independently verified")
    print("PASS: all 10 IEEE binary128 constants independently verified")
    print(f"PASS: PI_BINARY16 = 0x{source16['PI']:04x}")
    print(f"PASS: PI_BINARY128 = 0x{source128['PI']:032x}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
