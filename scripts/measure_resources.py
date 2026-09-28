#!/usr/bin/env python3
"""Reproduce Perfectπ object-level resource probes with standard Python."""

from pathlib import Path
import os
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
PROBES = ("native", "bounded40", "conversion40", "conversion_low")


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


def llvm_size_path() -> Path:
    run(["rustup", "component", "add", "llvm-tools"])
    sysroot = Path(run(["rustc", "--print", "sysroot"], capture=True).strip())
    executable = "llvm-size.exe" if os.name == "nt" else "llvm-size"
    path = sysroot / "lib" / "rustlib" / host_triple() / "bin" / executable
    if not path.is_file():
        raise RuntimeError(f"llvm-size not found at {path}")
    return path


def newest_rlib(target: str) -> Path:
    directory = ROOT / "target" / target / "release" / "deps"
    matches = list(directory.glob("libperfect_pi-*.rlib"))
    if not matches:
        raise RuntimeError(f"Perfectπ rlib not found for {target}")
    return max(matches, key=lambda path: path.stat().st_mtime_ns)


def section_totals(output: str) -> tuple[int, int, int]:
    text_bytes = 0
    rodata_bytes = 0
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
        elif section.startswith(".ARM.exidx"):
            unwind_bytes += size
    return text_bytes, rodata_bytes, unwind_bytes


def main() -> int:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    llvm_size = llvm_size_path()
    print("| Target | Probe | Text | rodata | ARM exidx |")
    print("| --- | --- | ---: | ---: | ---: |")

    for target in TARGETS:
        run(["rustup", "target", "add", target])
        run(["cargo", "build", "--release", "--lib", "--target", target])
        rlib = newest_rlib(target)

        for probe in PROBES:
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
            text_bytes, rodata_bytes, unwind_bytes = section_totals(size_output)
            print(
                f"| `{target}` | `{probe}` | {text_bytes} | "
                f"{rodata_bytes} | {unwind_bytes} |"
            )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
