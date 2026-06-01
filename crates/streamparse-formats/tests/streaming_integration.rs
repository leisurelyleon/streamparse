//! End-to-end streaming tests: drive real formats through the engine with input
//! deliberately fragmented across feed boundaries, proving records reassemble
//! correctly regardless of how the bytes arrive.

use streamparse_core::{RecordPayload, StreamParser};
use streamparse_formats::{Delimited, Ndjson};

/// Feeds `input` one byte at a time — the most adversarial fragmentation — and
/// collects all events.
fn feed_byte_by_byte<F: streamparse_core::Format>(
    format: F,
    input: &[u8],
) -> Vec<streamparse_core::ParseEvent> {
    let mut parser = StreamParser::new(format);
    let mut events = Vec::new();
    for &byte in input {
        events.extend(parser.feed(&[byte]).unwrap());
    }
    events.extend(parser.finish().unwrap());
    events
}

#[test]
fn ndjson_reassembles_under_single_byte_feeds() {
    let input = b"{\"id\":1}\n{\"id\":2}\n{\"id\":3}\n";
    let events = feed_byte_by_byte(Ndjson, input);
    assert_eq!(events.len(), 3);
    assert_eq!(events[2].index, 2);
}

#[test]
fn delimited_reassembles_under_single_byte_feeds() {
    let input = b"a,b,c\nd,e,f\n";
    let events = feed_byte_by_byte(Delimited::comma(), input);
    assert_eq!(events.len(), 2);
    match &events[1].payload {
        RecordPayload::Fields(fields) => {
            assert_eq!(fields, &vec!["d".to_string(), "e".to_string(), "f".to_string()]);
        }
        RecordPayload::Json(_) => panic!("expected fields payload"),
    }
}

#[test]
fn ndjson_chunked_matches_whole_input() {
    let input = b"{\"a\":1}\n{\"b\":2}\n";

    // Whole input in one feed.
    let mut whole = StreamParser::new(Ndjson);
    let mut whole_events = whole.feed(input).unwrap();
    whole_events.extend(whole.finish().unwrap());

    // Same input, fragmented.
    let chunked_events = feed_byte_by_byte(Ndjson, input);

    assert_eq!(whole_events.len(), chunked_events.len());
    assert_eq!(whole_events, chunked_events);
}

#[test]
fn large_stream_processes_all_records() {
    let mut input = Vec::new();
    for i in 0..5_000 {
        input.extend_from_slice(format!("{{\"n\":{i}}}\n").as_bytes());
    }

    let mut parser = StreamParser::new(Ndjson);
    let mut count = 0u64;
    for chunk in input.chunks(512) {
        count += parser.feed(chunk).unwrap().len() as u64;
    }
    count += parser.finish().unwrap().len() as u64;

    assert_eq!(count, 5_000);
}
