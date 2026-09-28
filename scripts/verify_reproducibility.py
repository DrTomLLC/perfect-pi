#!/usr/bin/env python3
"""Verify deterministic Perfectπ probe objects across clean builds and hosts."""

from pathlib import Path
import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
PROBE_DIR = ROOT / "verification" / "probes"
WORK_ROOT = ROOT / "target" / "reproducibility"
TARGETS = (
    "thumbv6m-none-eabi",
    "thumbv7em-none-eabihf",
    "riscv32imac-unknown-none-elf",
    "wasm32-unknown-unknown",
    "aarch64-unknown-linux-gnu",
)
PROBES = (
    ("native", ()),
    ("bounded40", ()),
    ("conversion40", ()),
    ("conversion_low", ()),
    ("binary16", ("binary16",)),
    ("binary128", ("binary128",)),
)


def run(command: list[str], *, capture: bool = False) -> str:
    result = subprocess.run(
        command, cwd=ROOT, text=True, capture_output=capture, check=False
    )
    if result.returncode != 0:
        if capture:
            sys.stderr.write(result.stdout)
            sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    return result.stdout if capture else ""


def rustc_info() -> dict[str, str]:
    output = run(["rustc", "-vV"], capture=True)
    info: dict[str, str] = {}
    for line in output.splitlines():
        if ": " in line:
            key, value = line.split(": ", maxsplit=1)
            info[key] = value
    return info


def build_rlib(target: str, features: tuple[str, ...], target_dir: Path) -> Path:
    command = [
        "cargo",
        "build",
        "--release",
        "--lib",
        "--target",
        target,
        "--target-dir",
        str(target_dir),
        "--message-format=json-render-diagnostics",
    ]
    if features:
        command.extend(["--features", ",".join(features)])

    output = run(command, capture=True)
    rlibs: list[Path] = []
    for line in output.splitlines():
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        if message.get("reason") != "compiler-artifact":
            continue
        artifact_target = message.get("target", {})
        if artifact_target.get("name") != "perfect_pi":
            continue
        for filename in message.get("filenames", []):
            path = Path(filename)
            if path.suffix == ".rlib":
                rlibs.append(path)

    if len(rlibs) != 1:
        raise RuntimeError(
            f"expected one Perfectπ rlib for {target}/{features}, found {len(rlibs)}"
        )
    return rlibs[0]


def digest(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            hasher.update(block)
    return hasher.hexdigest()


def build_run(label: str) -> dict[str, str]:
    run_root = WORK_ROOT / label
    if run_root.exists():
        shutil.rmtree(run_root)
    target_dir = run_root / "cargo"
    object_dir = run_root / "objects"
    object_dir.mkdir(parents=True, exist_ok=True)

    hashes: dict[str, str] = {}
    for target in TARGETS:
        run(["rustup", "target", "add", target])
        rlibs: dict[tuple[str, ...], Path] = {}
        for probe, features in PROBES:
            if features not in rlibs:
                rlibs[features] = build_rlib(target, features, target_dir)
            source = PROBE_DIR / f"{probe}.rs"
            object_file = object_dir / f"{probe}-{target}.o"
            run([
                "rustc",
                str(source),
                "--edition=2024",
                "--crate-type=lib",
                "--emit=obj",
                "--target",
                target,
                "--extern",
                f"perfect_pi={rlibs[features]}",
                "--remap-path-prefix",
                f"{ROOT}=/perfect-pi",
                "-C",
                "opt-level=z",
                "-C",
                "panic=abort",
                "-o",
                str(object_file),
            ])
            hashes[f"{target}/{probe}"] = digest(object_file)
    return hashes


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path)
    parser.add_argument("--expect", type=Path)
    args = parser.parse_args()

    WORK_ROOT.mkdir(parents=True, exist_ok=True)
    first = build_run("run-a")
    second = build_run("run-b")

    mismatches = [key for key in first if first[key] != second.get(key)]
    if mismatches:
        print("FAIL: same-host reproducibility mismatch", file=sys.stderr)
        for key in mismatches:
            print(f"- {key}: {first[key]} != {second.get(key)}", file=sys.stderr)
        return 1

    info = rustc_info()
    manifest = {
        "rustc_release": info.get("release", ""),
        "rustc_commit_hash": info.get("commit-hash", ""),
        "host": info.get("host", ""),
        "artifacts": first,
    }

    if args.output is not None:
        output = args.output if args.output.is_absolute() else ROOT / args.output
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if args.expect is not None:
        expected_path = args.expect if args.expect.is_absolute() else ROOT / args.expect
        expected = json.loads(expected_path.read_text(encoding="utf-8"))
        failures: list[str] = []
        for key in ("rustc_release", "rustc_commit_hash"):
            if manifest.get(key) != expected.get(key):
                failures.append(
                    f"{key}: actual={manifest.get(key)!r}, expected={expected.get(key)!r}"
                )
        if manifest["artifacts"] != expected.get("artifacts"):
            actual_keys = set(manifest["artifacts"])
            expected_artifacts = expected.get("artifacts", {})
            expected_keys = set(expected_artifacts)
            for key in sorted(actual_keys | expected_keys):
                if manifest["artifacts"].get(key) != expected_artifacts.get(key):
                    failures.append(
                        f"{key}: actual={manifest['artifacts'].get(key)}, "
                        f"expected={expected_artifacts.get(key)}"
                    )

        if failures:
            print("FAIL: reproducibility golden-manifest mismatch", file=sys.stderr)
            for failure in failures:
                print(f"- {failure}", file=sys.stderr)
            return 1
        print(f"PASS: all {len(first)} probe objects match the committed golden manifest")

    print(f"PASS: {len(first)} probe objects are byte-identical across two clean builds")
    print(f"PASS: rustc {manifest['rustc_release']} commit {manifest['rustc_commit_hash']}")
    print(f"HOST: {manifest['host']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
