# streamparse

> A streaming, incremental data parser that processes arbitrarily large inputs in bounded memory.

`streamparse` parses structured data from a byte stream *incrementally*: you feed
it bytes as they arrive, and it emits parse events as they complete, without ever
holding the entire input in memory. This makes it suitable for parsing files far
larger than RAM, network streams, and pipelines where data arrives in chunks.

## The Problem

Most parsers require the whole input up front, which is impossible for streams
and wasteful for very large files. `streamparse` is push-based: it accepts input
in arbitrary chunks via `feed`, maintains only a small bounded buffer of
partially-parsed data, and emits events as records complete. Peak memory is a
function of the largest single record, not the total input size.

## Architecture

```
streamparse-core      the push-based engine: buffer, tokenizer, Format trait, parser
streamparse-formats   concrete formats: NDJSON and delimited (CSV-like)
streamparse-cli       the binary: stream-parse a file or stdin
```

The engine is format-agnostic: a `Format` trait defines how bytes become events,
and the core drives it incrementally. Tokenization borrows slices of the input
buffer (zero-copy) wherever a token lies within a single fed chunk.

## Build & Test

```bash
cargo build
cargo test
```

## Run

```bash
# Parse newline-delimited JSON from a file
cargo run -p streamparse-cli -- parse --format ndjson data.ndjson

# Parse delimited records from stdin
cat data.csv | cargo run -p streamparse-cli -- parse --format delimited -

# Print record statistics instead of events
cargo run -p streamparse-cli -- stats --format ndjson data.ndjson
```

## License

MIT — see [LICENSE](LICENSE).
