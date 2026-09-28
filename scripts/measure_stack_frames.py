#!/usr/bin/env python3
"""Measure static stack-frame metadata for constrained-target probes."""

from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "verification" / "stack-probe" / "Cargo.toml"
OUTPUT = ROOT / "target" / "stack-probe"
TARGETS = ("thumbv6m-none-eabi", "thumbv7em-none-eabihf", "riscv32imac-unknown-none-elf")
FUNCTIONS = ("bounded40", "conversion40")


def run(command: list[str], capture: bool = False, env: dict[str, str] | None = None) -> str:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=capture, check=False, env=env)
    if result.returncode != 0:
        if capture:
            sys.stderr.write(result.stdout)
            sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    return result.stdout if capture else ""


def host_triple() -> str:
    output = run(["rustc", "+nightly", "-vV"], capture=True)
    for line in output.splitlines():
        if line.startswith("host: "):
            return line.removeprefix("host: ").strip()
    raise RuntimeError("nightly rustc host triple not found")


def llvm_readobj_path() -> Path:
    run(["rustup", "component", "add", "llvm-tools", "--toolchain", "nightly"])
    sysroot = Path(run(["rustc", "+nightly", "--print", "sysroot"], capture=True).strip())
    executable = "llvm-readobj.exe" if os.name == "nt" else "llvm-readobj"
    path = sysroot / "lib" / "rustlib" / host_triple() / "bin" / executable
    if not path.is_file():
        raise RuntimeError(f"llvm-readobj not found at {path}")
    return path


def parse_stack_sizes(output: str) -> dict[str, int]:
    measurements: dict[str, int] = {}
    pattern = re.compile(
        r"Functions:\s*\[[^\]]*(bounded40|conversion40)[^\]]*\]\s*Size:\s*0x([0-9A-Fa-f]+)",
        re.DOTALL,
    )
    for name, size in pattern.findall(output):
        measurements[name] = int(size, 16)
    return measurements


def main() -> int:
    llvm_readobj = llvm_readobj_path()
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(OUTPUT)

    version = run(["rustc", "+nightly", "--version"], capture=True).strip()
    print(f"Nightly instrumenter: {version}")
    print("| Target | bounded40 frame bytes | conversion40 frame bytes |")
    print("| --- | ---: | ---: |")

    for target in TARGETS:
        run(["rustup", "target", "add", target, "--toolchain", "nightly"])
        run(
            [
                "cargo", "+nightly", "rustc",
                "--manifest-path", str(MANIFEST),
                "--release", "--target", target,
                "--", "-Z", "emit-stack-sizes",
            ],
            env=env,
        )
        artifact = OUTPUT / target / "release" / "libperfect_pi_stack_probe.rlib"
        if not artifact.is_file():
            raise RuntimeError(f"stack probe artifact not found at {artifact}")
        measured = parse_stack_sizes(
            run([str(llvm_readobj), "--stack-sizes", str(artifact)], capture=True)
        )
        missing = [name for name in FUNCTIONS if name not in measured]
        if missing:
            raise RuntimeError(f"missing stack-size entries for {target}: {missing}")
        print(f"| `{target}` | {measured['bounded40']} | {measured['conversion40']} |")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
