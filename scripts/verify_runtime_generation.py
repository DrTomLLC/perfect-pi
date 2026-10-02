#!/usr/bin/env python3
"""Independently verify the optional runtime pi generator."""

from decimal import (
    Decimal,
    ROUND_CEILING,
    ROUND_DOWN,
    ROUND_FLOOR,
    ROUND_HALF_EVEN,
    ROUND_HALF_UP,
    localcontext,
)
import subprocess
import sys

from verify_reference import chudnovsky_pi, gauss_legendre_pi

VERIFICATION_DIGITS = 10_001
CHECKPOINTS = (0, 1, 2, 3, 10, 40, 100, 256, 1_000, 10_000, 10_001)
MODES = {
    "trunc": ROUND_DOWN,
    "away": ROUND_CEILING,
    "floor": ROUND_FLOOR,
    "ceil": ROUND_CEILING,
    "nearest-even": ROUND_HALF_EVEN,
    "nearest-away": ROUND_HALF_UP,
}


def generated_pi(decimal_places: int, mode: str) -> str:
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
        mode,
    ]
    process = subprocess.run(
        command,
        check=False,
        capture_output=True,
        text=True,
    )
    if process.returncode != 0:
        print(
            f"FAIL: {mode} runtime generator failed at D={decimal_places}",
            file=sys.stderr,
        )
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
        print(
            "FAIL: independent algorithms disagree at runtime verification depth",
            file=sys.stderr,
        )
        return 1

    with localcontext() as ctx:
        ctx.prec = VERIFICATION_DIGITS + 30
        for decimal_places in CHECKPOINTS:
            quantum = Decimal(1).scaleb(-decimal_places)
            for mode, decimal_rounding in MODES.items():
                expected = format(
                    chudnovsky_value.quantize(quantum, rounding=decimal_rounding),
                    f".{decimal_places}f",
                )
                generated = generated_pi(decimal_places, mode)
                if generated != expected:
                    print(
                        f"FAIL: {mode} runtime generator disagrees at "
                        f"D={decimal_places}",
                        file=sys.stderr,
                    )
                    return 1

    print(
        f"PASS: all six runtime rounding modes match independently computed "
        f"Chudnovsky and Gauss-Legendre pi through {VERIFICATION_DIGITS} "
        f"fractional digits at {len(CHECKPOINTS)} checkpoints"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
