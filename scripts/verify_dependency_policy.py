#!/usr/bin/env python3
"""Enforce Perfectπ newest-stable direct dependency and fuzz-tooling policy."""

from pathlib import Path
import json
import re
import sys
import tomllib
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "Cargo.toml"
FUZZ_MANIFEST = ROOT / "fuzz" / "Cargo.toml"
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


def raw_version(spec: object) -> str | None:
    if isinstance(spec, str):
        return spec
    if isinstance(spec, dict):
        value = spec.get("version")
        if isinstance(value, str):
            return value
    return None


def require_exact_current(
    crate: str, spec: object, failures: list[str], context: str
) -> None:
    latest = newest_stable(crate)
    expected = f"={latest}"
    actual = raw_version(spec)
    if actual != expected:
        failures.append(f"{context}/{crate}: expected exact pin {expected}, found {actual!r}")
    else:
        print(f"PASS: {context}/{crate} is exactly pinned to newest stable {latest}")


def check_production(failures: list[str]) -> None:
    with MANIFEST.open("rb") as handle:
        manifest = tomllib.load(handle)

    dependencies = manifest.get("dependencies", {})
    for crate, spec in sorted(dependencies.items()):
        require_exact_current(crate, spec, failures, "production")
        if not isinstance(spec, dict) or spec.get("optional") is not True:
            failures.append(f"production/{crate}: dependency must remain optional")
        else:
            print(f"PASS: production/{crate} remains opt-in")
        if not isinstance(spec, dict) or spec.get("default-features") is not False:
            failures.append(f"production/{crate}: default features must remain disabled")
        else:
            print(f"PASS: production/{crate} default features are disabled")


def check_fuzz(failures: list[str]) -> None:
    with FUZZ_MANIFEST.open("rb") as handle:
        manifest = tomllib.load(handle)

    dependencies = manifest.get("dependencies", {})
    expected_external = {"fixed", "libfuzzer-sys"}
    external = set(dependencies) - {"perfect-pi"}
    if external != expected_external:
        failures.append(
            f"fuzz: external dependency set must be {sorted(expected_external)}, found {sorted(external)}"
        )

    for crate in sorted(expected_external):
        spec = dependencies.get(crate)
        if spec is None:
            failures.append(f"fuzz/{crate}: dependency missing")
            continue
        require_exact_current(crate, spec, failures, "fuzz")

    fixed = dependencies.get("fixed")
    if not isinstance(fixed, dict) or fixed.get("default-features") is not False:
        failures.append("fuzz/fixed: default features must remain disabled")
    else:
        print("PASS: fuzz/fixed default features are disabled")

    perfect_pi = dependencies.get("perfect-pi")
    if not isinstance(perfect_pi, dict):
        failures.append("fuzz/perfect-pi: path dependency missing")
        return
    if perfect_pi.get("path") != "..":
        failures.append("fuzz/perfect-pi: path must remain '..'")
    required_features = {"all-float-formats", "interop", "runtime-generation"}
    features = set(perfect_pi.get("features", []))
    if not required_features.issubset(features):
        failures.append(
            "fuzz/perfect-pi: must enable all-float-formats, interop, and runtime-generation"
        )
    else:
        print("PASS: fuzz/perfect-pi exercises all optional numeric surfaces")


def main() -> int:
    failures: list[str] = []
    check_production(failures)
    check_fuzz(failures)

    if failures:
        print("FAIL: dependency currency/isolation policy violated", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
