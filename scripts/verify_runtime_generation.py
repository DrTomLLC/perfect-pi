#!/usr/bin/env python3
"""Independently verify the optional runtime pi generator."""

from decimal import Decimal, ROUND_HALF_EVEN, localcontext
import subprocess
import sys

from verify_reference import chudnovsky_pi, gauss_legendre_pi

VERIFICATION_DIGITS = 1000
CHECKPOINTS = (0, 1, 2, 3, 10, 40, 100, 256, 1000)


def generated_pi(decimal_places: int, rounded: bool = False) -> str:
    command = [
        "cargo",
        "run",
        "--quiet",
        "--example",
        "runtime_generate",
        "--features",
        "runtime-generation",
        "--",
        str(decimal_places),
    ]
    if rounded:
        command.append("round")
    process = subprocess.run(
        command,
        check=False,
        capture_output=True,
        text=True,
    )
    if process.returncode != 0:
        mode = "rounded" if rounded else "truncated"
        print(f"FAIL: {mode} runtime generator failed at D={decimal_places}", file=sys.stderr)
        print(process.stderr, file=sys.stderr)
        raise RuntimeError("runtime generator example failed")
    return process.stdout.strip()


def main() -> int:
    chudnovsky_value = chudnovsky_pi(VERIFICATION_DIGITS + 8)
    gauss_legendre_value = gauss_legendre_pi(VERIFICATION_DIGITS + 8)
    chudnovsky = format(chudnovsky_value, "f")
    gauss_legendre = format(gauss_legendre_value, "f")
    length = VERIFICATION_DIGITS + 2

    if chudnovsky[:length] != gauss_legendre[:length]:
        print("FAIL: independent algorithms disagree at runtime verification depth", file=sys.stderr)
        return 1

    with localcontext() as ctx:
        ctx.prec = VERIFICATION_DIGITS + 30
        for decimal_places in CHECKPOINTS:
            generated = generated_pi(decimal_places)
            expected_length = 1 if decimal_places == 0 else decimal_places + 2
            expected = "3" if decimal_places == 0 else chudnovsky[:expected_length]
            if generated != expected:
                print(
                    f"FAIL: truncated runtime generator disagrees at D={decimal_places}",
                    file=sys.stderr,
                )
                return 1

            quantum = Decimal(1).scaleb(-decimal_places)
            rounded_expected = format(
                chudnovsky_value.quantize(quantum, rounding=ROUND_HALF_EVEN),
                f".{decimal_places}f",
            )
            rounded_generated = generated_pi(decimal_places, rounded=True)
            if rounded_generated != rounded_expected:
                print(
                    f"FAIL: rounded runtime generator disagrees at D={decimal_places}",
                    file=sys.stderr,
                )
                return 1

    print(
        f"PASS: truncated and nearest-even runtime generation match Chudnovsky and "
        f"Gauss-Legendre through {VERIFICATION_DIGITS} fractional digits at "
        f"{len(CHECKPOINTS)} checkpoints"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
