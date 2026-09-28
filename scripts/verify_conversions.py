#!/usr/bin/env python3
"""Independently verify Perfectπ decimal-to-IEEE conversion vectors."""

from fractions import Fraction
from pathlib import Path
import argparse
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
BOUNDED_TEST = ROOT / "tests" / "bounded_precision.rs"
CONVERSION_TEST = ROOT / "tests" / "conversion_bits.rs"
EXPECTED_PRECISIONS = 41


def decimal_fraction(text: str) -> Fraction:
    if "." not in text:
        return Fraction(int(text), 1)
    integer, fractional = text.split(".", maxsplit=1)
    denominator = 10 ** len(fractional)
    numerator = int(integer) * denominator + int(fractional)
    return Fraction(numerator, denominator)


def ieee_bits(text: str, precision_bits: int, fraction_bits: int, bias: int) -> int:
    value = decimal_fraction(text)
    if not (Fraction(2, 1) <= value < Fraction(4, 1)):
        raise ValueError(f"value outside Perfectπ conversion domain: {text}")

    exponent = 1
    shift = precision_bits - 1 - exponent
    scaled_numerator = value.numerator << shift
    quotient, remainder = divmod(scaled_numerator, value.denominator)
    doubled = remainder * 2
    if doubled > value.denominator or (
        doubled == value.denominator and quotient % 2 != 0
    ):
        quotient += 1

    leading = 1 << (precision_bits - 1)
    if quotient >= (leading << 1):
        quotient >>= 1
        exponent += 1

    fraction_field = quotient - leading
    exponent_field = exponent + bias
    return (exponent_field << fraction_bits) | fraction_field


def bounded_cases() -> list[tuple[int, str, str]]:
    text = BOUNDED_TEST.read_text(encoding="utf-8")
    matches = re.findall(
        r'assert_case!\(\s*(\d+)\s*,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)',
        text,
        re.DOTALL,
    )
    cases = [(int(d), truncated, rounded) for d, truncated, rounded in matches]
    if len(cases) != EXPECTED_PRECISIONS:
        raise RuntimeError(
            f"expected {EXPECTED_PRECISIONS} bounded cases, found {len(cases)}"
        )
    if [d for d, _, _ in cases] != list(range(EXPECTED_PRECISIONS)):
        raise RuntimeError("bounded cases do not cover D=0..40 in order")
    return cases


def float_fraction(
    bits: int, precision_bits: int, fraction_bits: int, bias: int
) -> Fraction:
    exponent_field = bits >> fraction_bits
    fraction_field = bits & ((1 << fraction_bits) - 1)
    exponent = exponent_field - bias
    significand = (1 << (precision_bits - 1)) + fraction_field
    shift = precision_bits - 1 - exponent
    return Fraction(significand, 1 << shift)


def expected_vectors() -> list[tuple[int, int, int, int, int]]:
    vectors = []
    for d, truncated, rounded in bounded_cases():
        vectors.append(
            (
                d,
                ieee_bits(truncated, 24, 23, 127),
                ieee_bits(truncated, 53, 52, 1023),
                ieee_bits(rounded, 24, 23, 127),
                ieee_bits(rounded, 53, 52, 1023),
            )
        )
    return vectors


def emitted_lines() -> list[str]:
    return [
        (
            f"    assert_bits!({d}, 0x{tf32:08x}, 0x{tf64:016x}, "
            f"0x{rf32:08x}, 0x{rf64:016x});"
        )
        for d, tf32, tf64, rf32, rf64 in expected_vectors()
    ]


def verify_file() -> list[str]:
    text = CONVERSION_TEST.read_text(encoding="utf-8")
    matches = re.findall(
        r"assert_bits!\(\s*(\d+)\s*,\s*(0x[0-9a-fA-F]+)\s*,\s*"
        r"(0x[0-9a-fA-F]+)\s*,\s*(0x[0-9a-fA-F]+)\s*,\s*"
        r"(0x[0-9a-fA-F]+)\s*\);",
        text,
    )
    actual = [
        tuple([int(d), *(int(value, 16) for value in values)])
        for d, *values in matches
    ]
    expected = expected_vectors()
    failures: list[str] = []

    if len(actual) != EXPECTED_PRECISIONS:
        failures.append(
            f"expected {EXPECTED_PRECISIONS} conversion vectors, found {len(actual)}"
        )
        return failures

    for expected_row, actual_row in zip(expected, actual, strict=True):
        if expected_row != actual_row:
            failures.append(
                f"conversion vector mismatch: expected={expected_row}, actual={actual_row}"
            )

    return failures


def verify_preservation_bounds() -> list[str]:
    failures: list[str] = []
    f32_first_failure = False
    f64_first_failure = False

    for d, truncated, rounded in bounded_cases():
        for label, text in (("truncated", truncated), ("rounded", rounded)):
            exact = decimal_fraction(text)
            f32_bits = ieee_bits(text, 24, 23, 127)
            f64_bits = ieee_bits(text, 53, 52, 1023)
            f32_error = abs(float_fraction(f32_bits, 24, 23, 127) - exact)
            f64_error = abs(float_fraction(f64_bits, 53, 52, 1023) - exact)
            tolerance = Fraction(1, 2 * (10**d))

            if d <= 6 and not f32_error < tolerance:
                failures.append(f"f32 preservation failed at D={d} {label}")
            if d <= 15 and not f64_error < tolerance:
                failures.append(f"f64 preservation failed at D={d} {label}")
            if d == 7 and f32_error >= tolerance:
                f32_first_failure = True
            if d == 16 and f64_error >= tolerance:
                f64_first_failure = True

    if not f32_first_failure:
        failures.append("D=7 did not demonstrate why f32 guarantee stops at D=6")
    if not f64_first_failure:
        failures.append("D=16 did not demonstrate why f64 guarantee stops at D=15")

    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--emit", action="store_true", help="print independently generated vector lines"
    )
    args = parser.parse_args()

    if args.emit:
        print("\n".join(emitted_lines()))
        return 0

    failures = verify_file()
    failures.extend(verify_preservation_bounds())
    if failures:
        print("FAIL: conversion verification failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("PASS: all 41 decimal conversion vectors independently verified")
    print("PASS: each vector covers truncated/rounded × f32/f64 exact IEEE bits")
    print("PASS: f32 preservation guarantee verified through D=6; D=7 has a failing case")
    print("PASS: f64 preservation guarantee verified through D=15; D=16 has a failing case")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
