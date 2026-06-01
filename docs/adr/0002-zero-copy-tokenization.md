# 2. Zero-copy tokenization with an explicit copy boundary

- Status: Accepted
- Date: 2026-05

## Context

Allocating a new string for every token and field would dominate the cost of
parsing large inputs and defeat the bounded-memory goal.

## Decision

Tokenization borrows sub-slices of the input buffer rather than allocating:
finding terminators, splitting fields, and trimming all return borrowed slices.
The one unavoidable copy is at record emission, because a record may span feed
boundaries and must outlive buffer compaction; the parser makes exactly one
owned copy per completed record there.

## Consequences

- No per-token allocation; work is proportional to bytes scanned.
- The copy boundary is explicit, documented, and limited to one copy per record.
- Lifetimes stay simple: borrowed within a record, owned at the event boundary.
