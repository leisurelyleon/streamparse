//! Throughput + bounded-memory demonstration.
//!
//! The key property: parsing cost scales with input size, but peak *buffer*
//! memory does not — the parser holds at most one record at a time. This bench
//! streams progressively larger inputs in fixed-size chunks to show throughput
//! stays linear while the parser's buffer stays bounded.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use streamparse_core::{CoreError, Format, ParseEvent, RecordPayload, StreamParser};

/// A minimal line format, so the bench measures the *engine*, not a grammar.
struct LineFormat;

impl Format for LineFormat {
    fn record_terminator(&self) -> u8 {
        b'\n'
    }

    fn parse_record(&self, record: &[u8], index: u64) -> Result<ParseEvent, CoreError> {
        Ok(ParseEvent {
            index,
            raw_len: record.len(),
            payload: RecordPayload::Fields(vec![String::from_utf8_lossy(record).into_owned()]),
        })
    }
}

/// Builds an input of `record_count` newline-terminated records.
fn make_input(record_count: usize) -> Vec<u8> {
    let mut data = Vec::new();
    for i in 0..record_count {
        data.extend_from_slice(format!("record-number-{i}-with-some-payload\n").as_bytes());
    }
    data
}

fn bench_streaming(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_parse");

    for &record_count in &[1_000usize, 10_000, 100_000] {
        let input = make_input(record_count);
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(record_count),
            &input,
            |b, data| {
                b.iter(|| {
                    // Feed in fixed 8 KiB chunks — bounded memory regardless of
                    // total input size.
                    let mut parser = StreamParser::new(LineFormat);
                    let mut total = 0u64;
                    for chunk in data.chunks(8192) {
                        let events = parser.feed(chunk).unwrap();
                        total += events.len() as u64;
                    }
                    let tail = parser.finish().unwrap();
                    total += tail.len() as u64;
                    black_box(total)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_streaming);
criterion_main!(benches);
