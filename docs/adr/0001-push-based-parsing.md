# 1. Push-based incremental parsing

- Status: Accepted
- Date: 2026-05

## Context

A parser that requires the whole input up front cannot handle streams and
wastes memory on large files. The data may arrive in arbitrary chunks (network,
pipe, large file read).

## Decision

Make the parser push-based: callers `feed(&[u8])` bytes as they arrive, and the
parser emits events as records complete. Internal state is a bounded buffer of
the current partial record plus a record counter.

## Consequences

- Inputs larger than memory can be parsed in a bounded working set.
- The same engine serves files, pipes, and network streams.
- Callers control chunking and can interleave parsing with I/O.
