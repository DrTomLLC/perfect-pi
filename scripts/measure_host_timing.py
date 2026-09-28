#!/usr/bin/env python3
"""Build and run the host benchmark repeatedly, then report medians."""

from pathlib import Path
import csv
import statistics
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
RUNS = 5


def run(command: list[str], capture: bool = False) -> str:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=capture, check=False)
    if result.returncode != 0:
        if capture:
            sys.stderr.write(result.stdout)
            sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    return result.stdout if capture else ""


def main() -> int:
    run(["cargo", "build", "--release", "--example", "host_benchmark", "--features", "runtime-generation"])
    suffix = ".exe" if sys.platform == "win32" else ""
    executable = ROOT / "target" / "release" / "examples" / f"host_benchmark{suffix}"
    if not executable.is_file():
        raise RuntimeError(f"benchmark executable not found at {executable}")

    samples: dict[str, list[float]] = {}
    iterations: dict[str, int] = {}
    for _ in range(RUNS):
        output = run([str(executable)], capture=True)
        rows = csv.DictReader(output.splitlines())
        for row in rows:
            name = row["operation"]
            iterations[name] = int(row["iterations"])
            samples.setdefault(name, []).append(float(row["ns_per_op"]))

    print("| Operation | Iterations | Median ns/op | Five samples ns/op |")
    print("| --- | ---: | ---: | --- |")
    for name, values in samples.items():
        median = statistics.median(values)
        rendered = ", ".join(f"{value:.3f}" for value in values)
        print(f"| `{name}` | {iterations[name]} | {median:.3f} | {rendered} |")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
