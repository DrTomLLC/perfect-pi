#!/usr/bin/env python3
"""Verify Perfectπ verification-tool pins and workflow usage."""

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
EXPECTED_TOOLS = frozenset({
    "cargo-audit",
    "cargo-fuzz",
    "cargo-llvm-cov",
    "cargo-mutants",
})
INSTALL_PATTERN = re.compile(
    r"cargo install (?P<crate>cargo-[a-z0-9-]+) "
    r"--version =(?P<version>\d+(?:\.\d+)+) --locked"
)


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


def workflow_pins() -> tuple[dict[str, str], list[str]]:
    text = WORKFLOW.read_text(encoding="utf-8")
    pins: dict[str, str] = {}
    duplicates: list[str] = []

    for match in INSTALL_PATTERN.finditer(text):
        crate = match.group("crate")
        version = match.group("version")
        if crate in pins:
            duplicates.append(crate)
        pins[crate] = version

    return pins, duplicates


def main() -> int:
    with POLICY.open("rb") as handle:
        policy = tomllib.load(handle)

    failures: list[str] = []
    raw_tools = policy.get("verification-tools", {})
    if not isinstance(raw_tools, dict):
        print("FAIL: [verification-tools] must be a table", file=sys.stderr)
        return 1

    tools = dict(raw_tools)
    policy_names = set(tools)
    if policy_names != EXPECTED_TOOLS:
        missing = sorted(EXPECTED_TOOLS - policy_names)
        unexpected = sorted(policy_names - EXPECTED_TOOLS)
        if missing:
            failures.append(f"tooling.toml missing required tools: {', '.join(missing)}")
        if unexpected:
            failures.append(f"tooling.toml contains unexpected tools: {', '.join(unexpected)}")

    workflow, duplicates = workflow_pins()
    if duplicates:
        failures.append(
            "workflow contains duplicate cargo-install pins: " + ", ".join(sorted(set(duplicates)))
        )

    workflow_names = set(workflow)
    if workflow_names != EXPECTED_TOOLS:
        missing = sorted(EXPECTED_TOOLS - workflow_names)
        unexpected = sorted(workflow_names - EXPECTED_TOOLS)
        if missing:
            failures.append(f"CI workflow missing required tool installs: {', '.join(missing)}")
        if unexpected:
            failures.append(f"CI workflow installs unexpected tools: {', '.join(unexpected)}")

    for crate in sorted(EXPECTED_TOOLS):
        declared = tools.get(crate)
        if not isinstance(declared, str):
            failures.append(f"{crate}: tooling.toml version must be a string")
            continue
        if stable_key(declared) is None:
            failures.append(f"{crate}: tooling.toml version is not a stable numeric version: {declared}")
            continue

        workflow_version = workflow.get(crate)
        if workflow_version != declared:
            failures.append(
                f"{crate}: workflow pin {workflow_version!r} does not match tooling.toml {declared!r}"
            )

        latest = newest_stable(crate)
        if declared != latest:
            failures.append(f"{crate}: declared {declared}, newest stable {latest}")
        else:
            print(f"PASS: {crate} policy/workflow pin is exact and current at stable {latest}")

    if failures:
        print("FAIL: verification tooling policy violated", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("PASS: required verification-tool set is complete and CI pins match policy exactly")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
