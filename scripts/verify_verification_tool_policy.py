#!/usr/bin/env python3
"""Verify Perfectπ verification-tool currency and CI pin consistency."""

from pathlib import Path
import json
import re
import sys
import tomllib
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
POLICY = ROOT / "verification" / "policy" / "tooling.toml"
WORKFLOW = ROOT / ".github" / "workflows" / "ci.yml"
USER_AGENT = "PerfectPi-verification-tool-policy/0.1"
EXPECTED_TOOLS = {
    "cargo-audit",
    "cargo-fuzz",
    "cargo-mutants",
    "cargo-llvm-cov",
}


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
        raise RuntimeError(f"no stable version found for {crate}")
    return max(candidates)[1]


def main() -> int:
    with POLICY.open("rb") as handle:
        policy = tomllib.load(handle)

    failures: list[str] = []
    tools = policy.get("verification-tools", {})
    if not isinstance(tools, dict):
        failures.append("[verification-tools] must be a table")
        tools = {}

    declared_tools = set(tools)
    missing = sorted(EXPECTED_TOOLS - declared_tools)
    unexpected = sorted(declared_tools - EXPECTED_TOOLS)
    if missing:
        failures.append(f"missing required verification tools: {', '.join(missing)}")
    if unexpected:
        failures.append(f"unexpected verification tools: {', '.join(unexpected)}")

    workflow = WORKFLOW.read_text(encoding="utf-8")

    for crate in sorted(EXPECTED_TOOLS):
        declared = tools.get(crate)
        if not isinstance(declared, str):
            failures.append(f"{crate}: version must be a string")
            continue

        install_pattern = re.compile(
            rf"^\s*cargo install {re.escape(crate)} --version ={re.escape(declared)} --locked\s*$",
            re.MULTILINE,
        )
        if install_pattern.search(workflow) is None:
            failures.append(
                f"{crate}: ci.yml must install exactly --version ={declared} --locked"
            )

        latest = newest_stable(crate)
        if declared != latest:
            failures.append(f"{crate}: declared {declared}, newest stable {latest}")
        else:
            print(f"PASS: {crate} is current at stable {latest}")

    if failures:
        print("FAIL: verification tooling policy is inconsistent", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
