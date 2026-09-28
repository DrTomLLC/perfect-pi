# Universal 1.0 Independent Critic Review

Date: 2026-09-28

## Scope

This review covers Perfectπ pull request #10 and the Universal 1.0 repository-scoped
candidate. The reviewed production source is the source present at
`f11b0ab21b5e820777afd5f0b04b6732b9dbc349` and its descendants that change
verification or documentation only. The conversion hardening itself was introduced
at `64a1926faa027004500d0d93af03dae4cb7ee18b`.

This review does not publish a crate, create a GitHub release, or claim that a
downstream aircraft, medical device, vehicle, reactor, controller, or other critical
system is pre-certified.

## Review method

The critic review used the strongest available evidence rather than roadmap or
handoff assertions:

1. exact Git source and commit history;
2. pull-request review findings;
3. GitHub Actions job results;
4. independent numerical reference scripts;
5. mutation, coverage, fuzz, audit, portability, and reproducibility evidence;
6. measured resource and timing outputs;
7. normative specification, traceability, safety, qualification, and support documents.

Documentation claims were treated as subordinate to executable evidence.

## Material findings and dispositions

### 1. Directed-up bounded values were initially converted incorrectly — resolved

The PR review identified a P1 defect: values produced by `AwayFromZero` or
`TowardPositiveInfinity` could be classified as nearest-rounded values during
native-float conversion. Concrete examples included `Pi<0>` producing decimal
`4` but converting as `3`, and `Pi<2>` producing `3.15` but converting as
`3.14`.

The conversion layer was corrected and independently reverified. The current
converter recognizes the only two distinct finite decimal values available at a
fixed precision for positive irrational π: truncation/floor and ceiling. Nearest
rounding necessarily selects one of those two values.

Evidence:

- `tests/conversion_bits.rs` covers truncation, nearest, and ceiling conversion
  outcomes through every bounded precision;
- `scripts/verify_conversions.py` independently verifies 246 bounded outcomes
  (41 precisions × 3 decimal value classes × f32/f64);
- the original P1 review thread is resolved after the fix.

### 2. Conversion verification structure allowed surviving mutants — resolved

A const-generic lookup representation contained branches that were semantically
redundant or could not be distinguished by the public value model. The prior mutation
run reported 24 surviving mutants and coverage fell below the repository floor.

The lookup implementation was simplified to runtime-parameter,
`#[inline(always)]` helpers while preserving optimized dead stripping. Redundant
nearest-only tables and classification were removed.

Exact-source evidence at `64a1926faa027004500d0d93af03dae4cb7ee18b`:

- coverage: 96.57% lines, 92.98% regions, 100.00% functions;
- mutation testing: 238 mutants, 222 caught, 16 unviable, 0 missed, 0 timed out;
- Quality and verification, Current stable, Current nightly, Coverage, Mutation
  testing, Security audit, and no_std checks all passed for the unchanged production
  source in subsequent CI runs.

### 3. Reproducibility golden manifests became stale after legitimate code changes — resolved

The production changes altered expected object bytes. Ubuntu, macOS, and Windows
independently produced the same replacement hashes for every changed probe before
the golden manifest was refreshed.

The current manifest records those three-host-agreed hashes. A subsequent exact-head
run is required to remain green before merge.

### 4. Published resource measurements became stale after conversion hardening — resolved

The critic compared the exact-head `measure_resources.py` output with
`docs/MEASUREMENTS.md` and found stale conversion text/instruction counts.
The documentation and resource budget were updated to the measured values.

Current constrained-target conversion probe text sizes are:

- Cortex-M0: 394 B (`conversion40`), 524 B (`conversion_low`);
- Cortex-M hardware-float: 408 B, 580 B;
- RISC-V: 534 B, 722 B.

All measured constrained-target probes retain 0 bytes `.data` and 0 bytes `.bss`.
Static stack-frame measurements remain 92/8 B on Cortex-M0, 8/8 B on Cortex-M
hardware-float, and 48/0 B on RISC-V for bounded40/conversion40 respectively.
The stripped Windows linked-size probe remains +256 B text for bounded round-40
plus ASCII relative to baseline.

### 5. Runtime API documentation briefly claimed an owned-string API that was not retained — resolved

Perfectπ deliberately keeps final runtime output caller-owned. The stale API-stability
sentence describing an owned-string entry point was corrected. The specification,
runtime implementation, support contract, and API-stability document now agree.

## Numerical and API assessment

The critic found the current design internally consistent:

- bounded precision is exactly 0..=40 fractional decimal places;
- six public decimal rounding policies are explicit;
- because π is positive and irrational, floor equals truncation, ceiling equals
  away-from-zero, and exact halfway ties cannot occur;
- native f32/f64 conversion remains explicit about precision loss;
- optional binary16/binary128 adapters are bit-format interchange types, not hidden
  arithmetic emulation;
- runtime generation remains feature-gated, variable-cost, caller-buffer based, and
  protected by an optional caller-selected precision ceiling;
- the empty default feature set remains dependency-free, no_std, allocator-free, and
  unsafe-free;
- `full` is additive and does not alter the default core contract.

## Portability and assurance assessment

The dependency-free core is exercised across representative embedded, endian,
word-size, OS, mobile, WebAssembly/WASI, and architecture classes. Selected
no_std targets also exercise all production features. Host behavior and
reproducibility are checked on Windows, Linux, and macOS.

This is a universal library support contract, not universal downstream
certification. Target-specific WCET, transitive stack high-water, power/thermal
behavior, final linked-image effects, hardware/FPU behavior, fault models, and
applicable certification evidence remain bound to the actual deployed system as
specified in `docs/QUALIFICATION.md`.

## Critic conclusion

After resolving the findings above, no unresolved repository-scoped correctness,
portability, verification-architecture, licensing, or documentation blocker is
known to this review.

**Merge remains conditional on every configured GitHub CI job passing on the exact
final pull-request head.** If that head changes, exact-head CI must run again.

Package publication and creation of a stable public release remain separate,
owner-controlled actions. `publish = false` is intentionally unchanged.
