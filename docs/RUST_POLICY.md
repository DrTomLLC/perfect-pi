# Rust and Dependency Currency Policy

Perfectπ deliberately targets current software rather than long-tail compiler compatibility.

## Rust policy

Perfectπ supports the **newest stable Rust release only**.

At the time this policy was established:

```text
stable:  rustc 1.98.1
nightly: rustc 1.101.0-nightly (2026-09-27)
```

`Cargo.toml` sets `rust-version` to the current stable version. CI compares that value to the Rust `stable` channel and fails when stable advances until Perfectπ is updated, retested, and the manifest floor is moved forward.

Perfectπ does not maintain compatibility with older Rust releases merely for compatibility's sake.

## Nightly policy

Current nightly Rust is continuously tested as a forward-compatibility lane.

Nightly is not required by the default production crate. It is used to:

- detect upcoming compiler changes early;
- verify future language/library behavior;
- compare unstable native `f16` / `f128` constants against Perfectπ's stable IEEE binary16/binary128 contracts;
- prepare migrations before features reach stable.

When an unstable capability becomes stable and satisfies Perfectπ's correctness and portability requirements, Perfectπ may adopt it promptly.

## Dependency policy

Perfectπ's default bounded core remains dependency-free.

If an optional integration requires an external crate:

1. use the newest stable, appropriate release available when the dependency is adopted;
2. do not select an older release merely to support an obsolete Rust compiler;
3. disable unnecessary default features where practical;
4. keep the dependency behind an explicit opt-in feature;
5. document why it exists and its resource/dependency impact;
6. keep the default core free of that dependency;
7. review dependency currency as part of release preparation.

A dependency may be held back only for a documented correctness, safety, regression, platform, or compatibility defect in the newer release. Such an exception must be explicit and temporary.

## CI enforcement

Required CI includes:

- **Current stable** — verifies the manifest Rust floor equals the current stable toolchain and runs all-feature checks/tests/Clippy;
- **Current nightly** — runs Perfectπ against the newest nightly and verifies native nightly `f16` / `f128` π-family encodings against Perfectπ;
- host tests on current stable Linux, Windows, and macOS;
- current-stable `no_std` cross-target builds and resource probes.

This policy intentionally means a new Rust stable release can make CI fail until Perfectπ is moved forward and revalidated.
