# Architecture

`streamparse` is a Rust workspace implementing a push-based, incremental parser
that processes byte streams in bounded memory.

## Crates

```text
streamparse-core the engine: buffer, zero-copy tokenizer, Format trait, 
StreamParser streamparse-formats concrete formats: NDJSON and delimited 
(CSV-like) streamparse-cli the streamparse binary (parse / stats over files or stdin)
```

## The push-based model

`StreamParser` is fed bytes in arbitrary chunks via `feed(&[u8])`. It appends
them to a bounded internal buffer, scans for complete records (delimited by the
format's terminator), decodes each via the `Format`, and returns the resulting
events. Bytes left over after the last terminator remain buffered for the next
feed. `finish()` flushes any final unterminated record.

Because only the current partial record is retained, peak memory is bounded by
the largest single record — not by the total input size. A multi-gigabyte file
streams through a few-kilobyte working set.

## Zero-copy tokenization, with a documented copy boundary

The `tokenizer` module borrows slices of the input (finding terminators,
splitting fields, trimming) without allocating. The single place a copy is
unavoidable is at the *record emission boundary*: a record may span two `feed`
calls, so the parser materializes one owned copy of each completed record before
returning it, so the event outlives buffer compaction. This boundary is explicit
and documented; everything up to it is zero-copy.

## The Format seam

`Format` defines a record terminator and how a record's bytes become an event.
The engine drives any `Format`, so adding a grammar (e.g. a new delimited
dialect) requires no engine changes.
