#!/usr/bin/env python3
"""Measure linked host probe sections with release LTO and stripping."""

from pathlib import Path
import os
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "target" / "linked-probes"
PROBES = ("linked_baseline", "linked_native", "linked_bounded")


def run(command: list[str], capture: bool = False, env: dict[str, str] | None = None) -> str:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=capture, check=False, env=env)
    if result.returncode != 0:
        if capture:
            sys.stderr.write(result.stdout)
            sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    return result.stdout if capture else ""


def host_triple() -> str:
    output = run(["rustc", "-vV"], capture=True)
    for line in output.splitlines():
        if line.startswith("host: "):
            return line.removeprefix("host: ").strip()
    raise RuntimeError("rustc host triple not found")


def llvm_size_path() -> Path:
    run(["rustup", "component", "add", "llvm-tools"])
    sysroot = Path(run(["rustc", "--print", "sysroot"], capture=True).strip())
    executable = "llvm-size.exe" if os.name == "nt" else "llvm-size"
    path = sysroot / "lib" / "rustlib" / host_triple() / "bin" / executable
    if not path.is_file():
        raise RuntimeError(f"llvm-size not found at {path}")
    return path


def section_totals(output: str) -> dict[str, int]:
    totals = {"text": 0, "rodata": 0, "data": 0, "total": 0}
    for line in output.splitlines():
        parts = line.split()
        if len(parts) < 2 or not parts[1].isdigit():
            continue
        section = parts[0]
        size = int(parts[1])
        if section in {".text", "__text"}:
            totals["text"] += size
        elif section in {".rdata", ".rodata", "__const"}:
            totals["rodata"] += size
        elif section in {".data", "__data"}:
            totals["data"] += size
        if section != "Total":
            totals["total"] += size
    return totals


def main() -> int:
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(OUTPUT)
    env["CARGO_PROFILE_RELEASE_LTO"] = "fat"
    env["CARGO_PROFILE_RELEASE_CODEGEN_UNITS"] = "1"
    env["CARGO_PROFILE_RELEASE_STRIP"] = "symbols"

    command = ["cargo", "build", "--release"]
    for probe in PROBES:
        command.extend(["--example", probe])
    run(command, env=env)

    llvm_size = llvm_size_path()
    suffix = ".exe" if os.name == "nt" else ""
    print("| Probe | text | rodata | data | total measured sections |")
    print("| --- | ---: | ---: | ---: | ---: |")
    for probe in PROBES:
        executable = OUTPUT / "release" / "examples" / f"{probe}{suffix}"
        if not executable.is_file():
            raise RuntimeError(f"linked probe not found at {executable}")
        measured = section_totals(run([str(llvm_size), "-A", str(executable)], capture=True))
        print(
            f"| `{probe}` | {measured['text']} | {measured['rodata']} | "
            f"{measured['data']} | {measured['total']} |"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
