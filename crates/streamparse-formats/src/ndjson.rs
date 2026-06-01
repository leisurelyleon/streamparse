//! Newline-delimited JSON: one JSON value per line.

use streamparse_core::tokenizer::trim;
use streamparse_core::{CoreError, Format, ParseEvent, RecordPayload};

/// Parses newline-delimited JSON, validating each line as a JSON value and
/// storing its canonical text.
pub struct Ndjson;

impl Format for Ndjson {
    fn record_terminator(&self) -> u8 {
        b'\n'
    }

    fn parse_record(&self, record: &[u8], index: u64) -> Result<ParseEvent, CoreError> {
        let trimmed = trim(record);
        let value: serde_json::Value = serde_json::from_slice(trimmed)
            .map_err(|err| CoreError::InvalidRecord {
                index,
                message: err.to_string(),
            })?;
        Ok(ParseEvent {
            index,
            raw_len: record.len(),
            payload: RecordPayload::Json(value.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use streamparse_core::{RecordPayload, StreamParser};

    #[test]
    fn parses_ndjson_values() {
        let mut parser = StreamParser::new(Ndjson);
        let events = parser.feed(b"{\"a\":1}\n{\"b\":2}\n").unwrap();
        assert_eq!(events.len(), 2);
        match &events[0].payload {
            RecordPayload::Json(text) => assert!(text.contains("\"a\"")),
            RecordPayload::Fields(_) => panic!("expected JSON payload"),
        }
    }

    #[test]
    fn rejects_invalid_json() {
        let mut parser = StreamParser::new(Ndjson);
        assert!(parser.feed(b"not json\n").is_err());
    }

    #[test]
    fn handles_value_split_across_feeds() {
        let mut parser = StreamParser::new(Ndjson);
        assert!(parser.feed(b"{\"a\":").unwrap().is_empty());
        let events = parser.feed(b"1}\n").unwrap();
        assert_eq!(events.len(), 1);
    }
}
