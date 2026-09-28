#!/usr/bin/env python3
"""Independently verify Perfectπ canonical source digits.

This script uses only Python's standard-library Decimal implementation and a
Chudnovsky series. It does not import or execute Perfectπ.
"""

from decimal import Decimal, ROUND_HALF_EVEN, localcontext
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
BOUNDED_RS = ROOT / "src" / "bounded.rs"
BOUNDED_TEST = ROOT / "tests" / "bounded_precision.rs"
SOURCE_DIGITS = 41
VERIFICATION_DIGITS = 64
EXPECTED_PRECISIONS = 41


def chudnovsky_pi(decimal_places: int) -> Decimal:
    with localcontext() as ctx:
        ctx.prec = decimal_places + 30
        c = Decimal(426880) * Decimal(10005).sqrt()
        m = 1
        l = 13_591_409
        x = 1
        k = 6
        total = Decimal(l)

        # Each term contributes roughly 14 decimal digits.
        terms = (decimal_places // 14) + 3
        for i in range(1, terms):
            m = (m * (k**3 - 16 * k)) // (i**3)
            l += 545_140_134
            x *= -262_537_412_640_768_000
            total += Decimal(m * l) / Decimal(x)
            k += 12

        return +(c / total)


def gauss_legendre_pi(decimal_places: int) -> Decimal:
    with localcontext() as ctx:
        ctx.prec = decimal_places + 30
        one = Decimal(1)
        two = Decimal(2)
        four = Decimal(4)
        a = one
        b = one / two.sqrt()
        t = Decimal(1) / four
        p = one

        # Quadratic convergence roughly doubles correct digits per iteration.
        # Use a precision-scaled count with a wide safety margin instead of
        # relying on exact Decimal equality at the final rounded ulp.
        iterations = max(16, ctx.prec.bit_length() + 8)
        for _ in range(iterations):
            next_a = (a + b) / two
            b = (a * b).sqrt()
            t -= p * (a - next_a) ** 2
            a = next_a
            p *= two

        return +((a + b) ** 2 / (four * t))


def source_digits() -> str:
    text = BOUNDED_RS.read_text(encoding="utf-8")
    match = re.search(
        r"CANONICAL_FRACTIONAL_DIGITS:\s*\[u8;\s*(\d+)\]\s*=\s*\[(.*?)\];",
        text,
        re.DOTALL,
    )
    if match is None:
        raise RuntimeError("canonical digit table not found")

    declared = int(match.group(1))
    digits = "".join(re.findall(r"\b\d\b", match.group(2)))
    if declared != SOURCE_DIGITS or len(digits) != SOURCE_DIGITS:
        raise RuntimeError(
            f"expected {SOURCE_DIGITS} source digits, declared={declared}, parsed={len(digits)}"
        )
    return digits


def verify_test_vectors(pi_value: Decimal) -> list[str]:
    text = BOUNDED_TEST.read_text(encoding="utf-8")
    cases = re.findall(
        r'assert_case!\(\s*(\d+)\s*,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)',
        text,
        re.DOTALL,
    )
    failures: list[str] = []

    if len(cases) != EXPECTED_PRECISIONS:
        failures.append(
            f"expected {EXPECTED_PRECISIONS} precision vectors, found {len(cases)}"
        )
        return failures

    seen: set[int] = set()
    with localcontext() as ctx:
        ctx.prec = VERIFICATION_DIGITS + 30
        pi_text = format(pi_value, "f")
        _, fraction = pi_text.split(".", maxsplit=1)

        for d_text, trunc_actual, rounded_actual in cases:
            d = int(d_text)
            seen.add(d)

            trunc_expected = "3" if d == 0 else f"3.{fraction[:d]}"
            quantum = Decimal(1).scaleb(-d)
            rounded_value = pi_value.quantize(quantum, rounding=ROUND_HALF_EVEN)
            rounded_expected = format(rounded_value, f".{d}f")

            if trunc_actual != trunc_expected:
                failures.append(
                    f"D={d} truncation mismatch: test={trunc_actual}, expected={trunc_expected}"
                )
            if rounded_actual != rounded_expected:
                failures.append(
                    f"D={d} rounding mismatch: test={rounded_actual}, expected={rounded_expected}"
                )

    if seen != set(range(EXPECTED_PRECISIONS)):
        failures.append("precision vectors do not cover every D=0..40 exactly")

    return failures


def main() -> int:
    actual = source_digits()
    pi_value = chudnovsky_pi(VERIFICATION_DIGITS + 8)
    independent_pi = gauss_legendre_pi(VERIFICATION_DIGITS + 8)

    chudnovsky_text = format(pi_value, "f")
    gauss_legendre_text = format(independent_pi, "f")
    comparison_digits = VERIFICATION_DIGITS
    if chudnovsky_text[: comparison_digits + 2] != gauss_legendre_text[: comparison_digits + 2]:
        print("FAIL: independent π algorithms disagree", file=sys.stderr)
        print(f"Chudnovsky:     {chudnovsky_text}", file=sys.stderr)
        print(f"Gauss-Legendre: {gauss_legendre_text}", file=sys.stderr)
        return 1

    integer, fractional = chudnovsky_text.split(".", maxsplit=1)
    expected = fractional[:SOURCE_DIGITS]

    if integer != "3":
        print(f"FAIL: unexpected integer part {integer}", file=sys.stderr)
        return 1
    if actual != expected:
        print("FAIL: canonical digit mismatch", file=sys.stderr)
        print(f"source:   {actual}", file=sys.stderr)
        print(f"computed: {expected}", file=sys.stderr)
        return 1

    vector_failures = verify_test_vectors(pi_value)
    if vector_failures:
        print("FAIL: bounded precision vector verification failed", file=sys.stderr)
        for failure in vector_failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print(
        f"PASS: Chudnovsky and Gauss-Legendre agree through "
        f"{VERIFICATION_DIGITS} fractional digits"
    )
    print(f"PASS: all {SOURCE_DIGITS} production source digits independently verified")
    print(f"PASS: all {EXPECTED_PRECISIONS} truncation and rounding vectors independently verified")
    print(f"3.{actual}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
