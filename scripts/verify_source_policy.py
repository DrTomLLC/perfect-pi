#!/usr/bin/env python3
"""Enforce Perfectπ production-source reliability rules."""

from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"

FORBIDDEN = {
    "unsafe block": re.compile(r"\bunsafe\s*\{"),
    "unwrap-family call": re.compile(r"\bunwrap(?:_[A-Za-z0-9_]+)?\s*\("),
    "expect-family call": re.compile(r"\bexpect(?:_[A-Za-z0-9_]+)?\s*\("),
    "panic macro": re.compile(r"\bpanic!\s*\("),
    "todo macro": re.compile(r"\btodo!\s*\("),
    "unimplemented macro": re.compile(r"\bunimplemented!\s*\("),
    "alloc path": re.compile(r"\balloc::"),
    "std path": re.compile(r"\bstd::"),
}


def main() -> int:
    failures: list[str] = []

    for path in sorted(SRC.rglob("*.rs")):
        relative = path.relative_to(ROOT)
        for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            for label, pattern in FORBIDDEN.items():
                if pattern.search(line):
                    failures.append(f"{relative}:{line_number}: {label}: {line.strip()}")

    if failures:
        print("FAIL: production source policy violations found", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("PASS: production source contains no forbidden panic/unwrap/unsafe/std/alloc paths")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
