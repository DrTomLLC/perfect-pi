# Security Policy

## Supported versions

Perfectπ has not yet published a production release. Security support begins with the first released version.

## Reporting a vulnerability

Please use GitHub's private vulnerability reporting / Security Advisory mechanism when available rather than opening a public issue for a vulnerability that could affect users.

For ordinary correctness, numerical, documentation, or performance defects, use the public issue templates.

## Relevant issue classes

Security-sensitive reports may include:

- memory-safety violations;
- unexpected `unsafe` behavior;
- dependency compromise;
- build/release provenance problems;
- input handling that can cause denial of service in optional parsers or arbitrary-precision components;
- integrity-check bypasses if an integrity feature is implemented.

Numerical inaccuracies are treated as high-priority correctness issues even when they are not security vulnerabilities.
