#!/usr/bin/env python3
"""Reproduce Perfectπ object-level resource probes with standard Python."""

from pathlib import Path
import json
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
PROBE_DIR = ROOT / "verification" / "probes"
OUTPUT_DIR = ROOT / "target" / "resource-probes"
TARGETS = (
    "thumbv6m-none-eabi",
    "thumbv7em-none-eabihf",
    "riscv32imac-unknown-none-elf",
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


def host_triple() -> str:
    version = run(["rustc", "-vV"], capture=True)
    for line in version.splitlines():
        if line.startswith("host: "):
            return line.removeprefix("host: ").strip()
    raise RuntimeError("rustc host triple not found")


def llvm_tool_path(tool: str) -> Path:
    run(["rustup", "component", "add", "llvm-tools"])
    sysroot = Path(run(["rustc", "--print", "sysroot"], capture=True).strip())
    executable = f"{tool}.exe" if os.name == "nt" else tool
    path = sysroot / "lib" / "rustlib" / host_triple() / "bin" / executable
    if not path.is_file():
        raise RuntimeError(f"{tool} not found at {path}")
    return path


def build_rlib(target: str, features: tuple[str, ...] = ()) -> Path:
    command = [
        "cargo",
        "build",
        "--release",
        "--lib",
        "--target",
        target,
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
            f"expected exactly one Perfectπ rlib for {target}, found {len(rlibs)}"
        )
    return rlibs[0]


def section_totals(output: str) -> tuple[int, int, int, int, int]:
    text_bytes = 0
    rodata_bytes = 0
    data_bytes = 0
    bss_bytes = 0
    unwind_bytes = 0
    for line in output.splitlines():
        parts = line.split()
        if len(parts) < 2 or not parts[1].isdigit():
            continue
        section = parts[0]
        size = int(parts[1])
        if section.startswith(".text"):
            text_bytes += size
        elif section.startswith(".rodata"):
            rodata_bytes += size
        elif section.startswith(".data"):
            data_bytes += size
        elif section.startswith(".bss"):
            bss_bytes += size
        elif section.startswith(".ARM.exidx"):
            unwind_bytes += size
    return text_bytes, rodata_bytes, data_bytes, bss_bytes, unwind_bytes


def instruction_count(output: str) -> int:
    instruction = re.compile(r"^\s*[0-9a-fA-F]+:\s")
    return sum(1 for line in output.splitlines() if instruction.match(line))


def main() -> int:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    llvm_size = llvm_tool_path("llvm-size")
    llvm_objdump = llvm_tool_path("llvm-objdump")
    print("| Target | Probe | Text | rodata | data | bss | Instructions | ARM exidx |")
    print("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |")

    for target in TARGETS:
        run(["rustup", "target", "add", target])
        rlibs: dict[tuple[str, ...], Path] = {}

        for probe, features in PROBES:
            if features not in rlibs:
                rlibs[features] = build_rlib(target, features)
            rlib = rlibs[features]
            source = PROBE_DIR / f"{probe}.rs"
            object_file = OUTPUT_DIR / f"{probe}-{target}.o"
            run([
                "rustc", str(source), "--edition=2024", "--crate-type=lib",
                "--emit=obj", "--target", target,
                "--extern", f"perfect_pi={rlib}",
                "-C", "opt-level=z", "-C", "panic=abort",
                "-o", str(object_file),
            ])
            size_output = run([str(llvm_size), "-A", str(object_file)], capture=True)
            disassembly = run([str(llvm_objdump), "-d", str(object_file)], capture=True)
            text_bytes, rodata_bytes, data_bytes, bss_bytes, unwind_bytes = section_totals(size_output)
            instructions = instruction_count(disassembly)
            print(
                f"| `{target}` | `{probe}` | {text_bytes} | {rodata_bytes} | "
                f"{data_bytes} | {bss_bytes} | {instructions} | {unwind_bytes} |"
            )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
