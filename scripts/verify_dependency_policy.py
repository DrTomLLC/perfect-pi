#!/usr/bin/env python3
"""Enforce Perfectπ's newest-stable direct dependency policy."""

from pathlib import Path
import json
import re
import sys
import tomllib
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "Cargo.toml"
USER_AGENT = "PerfectPi-dependency-policy/0.1"


def stable_key(version: str) -> tuple[int, ...] | None:
    if "-" in version or "+" in version:
        return None
    if re.fullmatch(r"\d+(?:\.\d+)*", version) is None:
        return None
    return tuple(int(part) for part in version.split("."))


def newest_stable(crate: str) -> str:
    request = Request(
        f"https://crates.io/api/v1/crates/{crate}",
        headers={"User-Agent": USER_AGENT},
    )
    with urlopen(request, timeout=20) as response:
        payload = json.load(response)

    candidates: list[tuple[tuple[int, ...], str]] = []
    for version in payload.get("versions", []):
        number = version.get("num", "")
        if version.get("yanked", False):
            continue
        key = stable_key(number)
        if key is not None:
            candidates.append((key, number))

    if not candidates:
        raise RuntimeError(f"no non-yanked stable version found for {crate}")
    return max(candidates)[1]


def declared_version(spec: object) -> str:
    if not isinstance(spec, dict):
        raise RuntimeError("Perfectπ direct dependencies must use explicit tables")
    version = spec.get("version")
    if not isinstance(version, str):
        raise RuntimeError("Perfectπ direct dependency is missing a version")
    match = re.search(r"\d+(?:\.\d+)+", version)
    if match is None:
        raise RuntimeError(f"unsupported dependency version expression: {version}")
    return match.group(0)


def main() -> int:
    with MANIFEST.open("rb") as handle:
        manifest = tomllib.load(handle)

    failures: list[str] = []
    dependencies = manifest.get("dependencies", {})
    if not dependencies:
        print("PASS: Perfectπ has no direct dependencies")
        return 0

    for crate, spec in sorted(dependencies.items()):
        declared = declared_version(spec)
        latest = newest_stable(crate)
        exact_required = f"={latest}"

        if not isinstance(spec, dict) or spec.get("version") != exact_required:
            failures.append(
                f"{crate}: version must be exact newest-stable pin {exact_required}"
            )
        if not isinstance(spec, dict) or spec.get("optional") is not True:
            failures.append(f"{crate}: direct dependency must remain optional")
        if not isinstance(spec, dict) or spec.get("default-features") is not False:
            failures.append(f"{crate}: default features must remain disabled")

        if stable_key(declared) != stable_key(latest):
            failures.append(f"{crate}: declared {declared}, newest stable {latest}")
        else:
            print(f"PASS: {crate} is current at stable {latest}")

        if isinstance(spec, dict) and spec.get("optional") is True:
            print(f"PASS: {crate} remains opt-in")
        if isinstance(spec, dict) and spec.get("default-features") is False:
            print(f"PASS: {crate} default features are disabled")

    if failures:
        print("FAIL: direct dependency currency policy violated", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
