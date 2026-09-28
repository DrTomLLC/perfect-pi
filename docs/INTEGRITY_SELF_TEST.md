# Startup Integrity Self-Test Evaluation

## Decision

Perfectπ does **not** install an automatic startup self-test or global constructor.

## Rationale

The bounded core is `no_std`, allocation-free, deterministic, and usable on targets where there may be no conventional process startup. An automatic self-test would introduce initialization policy into a numerical constant library and could create misleading assurance: comparing compiled constants with compiled expected values does not independently validate the compiler, CPU, memory subsystem, power integrity, or downstream integration.

## Supported pattern

Applications with a safety case may perform their own startup or periodic test using Perfectπ's public deterministic surfaces and independently stored expected values. Suitable checks include native IEEE bit patterns, a bounded `Pi<D>` ASCII vector, and—in environments that enable it—a separately qualified runtime-generation comparison.

Perfectπ keeps those checks explicit at the application boundary instead of claiming that an internal self-test certifies hardware or system integrity.

## Revisit condition

Add a library self-test only if a concrete target supplies an independent fault model, storage separation requirement, diagnostic-coverage objective, and measured resource budget that the test can satisfy.
