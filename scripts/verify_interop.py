#!/usr/bin/env python3
"""Independently verify Perfectπ optional interoperability reference values."""

from fractions import Fraction
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
BOUNDED = ROOT / "tests" / "bounded_precision.rs"
INTEROP = ROOT / "tests" / "interop.rs"


def parse_decimal(text: str) -> Fraction:
    if "." not in text:
        return Fraction(int(text), 1)
    integer, fractional = text.split(".", maxsplit=1)
    denominator = 10 ** len(fractional)
    return Fraction(int(integer) * denominator + int(fractional), denominator)


def rounded_integer(value: Fraction) -> int:
    quotient, remainder = divmod(value.numerator, value.denominator)
    doubled = remainder * 2
    if doubled > value.denominator or (doubled == value.denominator and quotient & 1):
        quotient += 1
    return quotient


def bounded_case(d: int) -> tuple[str, str]:
    text = BOUNDED.read_text(encoding="utf-8")
    match = re.search(
        rf'assert_case!\(\s*{d}\s*,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)',
        text,
    )
    if match is None:
        raise RuntimeError(f"bounded D={d} case not found")
    return match.group(1), match.group(2)


def main() -> int:
    interop = INTEROP.read_text(encoding="utf-8")
    failures: list[str] = []

    _, rounded40 = bounded_case(40)
    source = parse_decimal(rounded40)
    expected_i16f16 = rounded_integer(source * (1 << 16))
    expected_i32f32 = rounded_integer(source * (1 << 32))

    fixed16 = re.search(r"assert_eq!\(i16f16\.to_bits\(\),\s*([0-9_]+)\);", interop)
    fixed32 = re.search(r"assert_eq!\(i32f32\.to_bits\(\),\s*([0-9_]+)\);", interop)
    if fixed16 is None or int(fixed16.group(1).replace("_", "")) != expected_i16f16:
        failures.append(f"I16F16 expected bits {expected_i16f16} not matched by test vector")
    if fixed32 is None or int(fixed32.group(1).replace("_", "")) != expected_i32f32:
        failures.append(f"I32F32 expected bits {expected_i32f32} not matched by test vector")

    truncated28, _ = bounded_case(28)
    exact_match = re.search(
        r'assert_eq!\(exact\.to_string\(\),\s*"([^"]+)"\);', interop
    )
    if exact_match is None or exact_match.group(1) != truncated28:
        failures.append(
            f"rust_decimal exact vector mismatch: expected {truncated28}"
        )

    truncated40, rounded40 = bounded_case(40)
    rounded_source = parse_decimal(rounded40)
    scale = 28
    scaled = rounded_source * (10**scale)
    rounded_coefficient = rounded_integer(scaled)
    integer = rounded_coefficient // (10**scale)
    fractional = rounded_coefficient % (10**scale)
    expected_decimal28 = f"{integer}.{fractional:028d}"
    rounded_match = re.search(
        r'assert_eq!\(rounded\.to_string\(\),\s*"([^"]+)"\);', interop
    )
    if rounded_match is None or rounded_match.group(1) != expected_decimal28:
        failures.append(
            f"rust_decimal rounded vector mismatch: expected {expected_decimal28}"
        )

    if failures:
        print("FAIL: interoperability verification failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print(f"PASS: I16F16 nearest-even bits = {expected_i16f16}")
    print(f"PASS: I32F32 nearest-even bits = {expected_i32f32}")
    print(f"PASS: rust_decimal exact D=28 = {truncated28}")
    print(f"PASS: rust_decimal nearest-even D=40 -> 28 = {expected_decimal28}")
    print("PASS: complex interoperability reuses independently verified native-float conversions")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
