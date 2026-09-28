#!/usr/bin/env python3
"""Enforce Perfectπ's current-Rust-only toolchain policy."""

from pathlib import Path
import re
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "Cargo.toml"


def rust_version(toolchain: str) -> str:
    result = subprocess.run(
        ["rustc", f"+{toolchain}", "--version"],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)

    match = re.match(r"rustc (\d+\.\d+\.\d+)", result.stdout.strip())
    if match is None:
        raise RuntimeError(f"unexpected rustc version output: {result.stdout!r}")
    return match.group(1)


def main() -> int:
    with MANIFEST.open("rb") as handle:
        manifest = tomllib.load(handle)

    declared = manifest["package"]["rust-version"]
    stable = rust_version("stable")

    if declared != stable:
        print(
            "FAIL: Perfectπ supports only the newest stable Rust release.",
            file=sys.stderr,
        )
        print(f"Cargo.toml rust-version = {declared}", file=sys.stderr)
        print(f"current stable rustc     = {stable}", file=sys.stderr)
        print(
            "Update Cargo.toml and revalidate the project against current stable.",
            file=sys.stderr,
        )
        return 1

    print(f"PASS: Cargo.toml rust-version matches current stable Rust {stable}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
