# 3. Pluggable formats via a trait

- Status: Accepted
- Date: 2026-05

## Context

The streaming engine and the grammar of a specific format are separate concerns.
Hardcoding a format into the engine would prevent reuse.

## Decision

Define a `Format` trait (record terminator + record decoder). The engine is
generic over `F: Format`. NDJSON and delimited formats are independent
implementations in a separate crate.

## Consequences

- New formats are added without touching the engine.
- The engine is tested independently with a trivial line format.
- Formats are tested independently and via cross-crate integration tests.
