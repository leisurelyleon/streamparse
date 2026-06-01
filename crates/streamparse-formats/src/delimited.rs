//! Delimited records (CSV-like): fields separated by a single delimiter byte.

use streamparse_core::tokenizer::{split_fields, trim};
use streamparse_core::{CoreError, Format, ParseEvent, RecordPayload};

/// Parses single-delimiter records, trimming each field. CRLF-safe: a trailing
/// `\r` is trimmed from the final field.
pub struct Delimited {
    delimiter: u8,
}

impl Delimited {
    pub fn new(delimiter: u8) -> Self {
        Self { delimiter }
    }

    /// A comma-delimited parser.
    pub fn comma() -> Self {
        Self { delimiter: b',' }
    }
}

impl Format for Delimited {
    fn record_terminator(&self) -> u8 {
        b'\n'
    }

    fn parse_record(&self, record: &[u8], index: u64) -> Result<ParseEvent, CoreError> {
        let fields: Vec<String> = split_fields(record, self.delimiter)
            .into_iter()
            .map(|field| String::from_utf8_lossy(trim(field)).into_owned())
            .collect();
        Ok(ParseEvent {
            index,
            raw_len: record.len(),
            payload: RecordPayload::Fields(fields),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use streamparse_core::{RecordPayload, StreamParser};

    #[test]
    fn splits_fields_on_delimiter() {
        let mut parser = StreamParser::new(Delimited::comma());
        let events = parser.feed(b"a,b,c\n").unwrap();
        assert_eq!(events.len(), 1);
        match &events[0].payload {
            RecordPayload::Fields(fields) => {
                assert_eq!(fields, &vec!["a".to_string(), "b".to_string(), "c".to_string()]);
            }
            RecordPayload::Json(_) => panic!("expected fields payload"),
        }
    }

    #[test]
    fn trims_fields_and_handles_crlf() {
        let mut parser = StreamParser::new(Delimited::comma());
        let events = parser.feed(b" a , b \r\n").unwrap();
        match &events[0].payload {
            RecordPayload::Fields(fields) => {
                assert_eq!(fields, &vec!["a".to_string(), "b".to_string()]);
            }
            RecordPayload::Json(_) => panic!("expected fields payload"),
        }
    }
}
