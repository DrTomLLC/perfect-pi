# Critical-System Qualification Guide

Perfectπ supplies reusable library evidence. Qualification of a critical
deployment is necessarily performed against an exact downstream system and
applicable assurance standard.

## Qualification baseline

Record all of the following before accepting Perfectπ into a controlled build:

1. exact Perfectπ commit SHA and source archive/hash;
2. enabled Cargo features and complete Cargo.lock;
3. rustc, cargo, LLVM, linker, target triple, target specification, and flags;
4. dependency inventory and advisory/license review;
5. final linked image/map and dead-code/link-time-optimization settings;
6. hardware/CPU/FPU, clock, memory, cache, MPU/MMU, RTOS/executor, and interrupt assumptions;
7. which Perfectπ APIs are reachable in the deployed configuration.

Changing any controlled item requires impact analysis and the re-verification
required by the deployment's change-control process.

## Numerical evidence

For bounded use, retain:

- canonical digit/reference checks;
- all 0..=40 known-answer vectors;
- explicit rounding-mode vectors;
- exact IEEE conversion vectors and checked-preservation boundaries;
- optional-format/interoperability evidence if those features are used.

For runtime/arbitrary precision, retain:

- independent Chudnovsky and Gauss-Legendre comparison;
- requested precision and caller maximum;
- selected rounding mode;
- resource and timeout limits;
- application tests at the maximum permitted precision.

## Target evidence

Repository object measurements do not replace target qualification. Measure on
the final toolchain and application when the safety case depends on them:

- final ROM/flash and RAM delta;
- transitive worst-case stack/high-water behavior;
- WCET or cycle bounds under the actual execution environment;
- FPU versus software-float behavior where applicable;
- startup, scheduling, interrupt, cache, and contention effects;
- power/thermal effects if they are safety relevant.

## Fault model and integrity

Define which faults are in scope: storage corruption, RAM upset, CPU fault,
compiler defect, power loss, radiation, concurrency, or external corruption.
Perfectπ does not install hidden startup hooks. If a safety case requires
startup or periodic integrity checks, the application should compare against
independently stored expected values with the required separation.

## Tool and process assurance

Where required by the governing framework, address compiler/tool qualification,
requirements traceability, independent verification, structural coverage,
configuration management, anomaly tracking, reproducible builds, and review
independence.

Relevant frameworks can include IEC 61508, IEC 62304/ISO 14971, ISO 26262,
DO-178C, EN 50128/EN 50716, nuclear-sector controls, or organization-specific
assurance processes. Perfectπ does not claim certification under a framework
unless a completed certification artifact explicitly says so.

## Acceptance record

A downstream qualification record should state:

- accepted Perfectπ SHA;
- application/system identifier and version;
- exact target/toolchain/features;
- tests and analyses executed with exact results;
- unresolved anomalies and dispositions;
- reviewers/approvers;
- validity conditions and requalification triggers.

This separation allows the crate to remain broadly reusable without making
claims that can only be proven for a concrete deployed system.
