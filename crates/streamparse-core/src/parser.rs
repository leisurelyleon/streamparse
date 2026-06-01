//! The push-based incremental parser.

use crate::buffer::{DEFAULT_MAX_RECORD_SIZE, StreamBuffer};
use crate::error::CoreError;
use crate::event::ParseEvent;
use crate::format::Format;
use crate::tokenizer;

/// Drives a `Format` over a byte stream incrementally. Feed bytes in arbitrary
/// chunks; receive events as records complete. Peak memory is bounded by the
/// largest single record.
pub struct StreamParser<F: Format> {
    format: F,
    buffer: StreamBuffer,
    next_index: u64,
}

impl<F: Format> StreamParser<F> {
    pub fn new(format: F) -> Self {
        Self::with_max_record_size(format, DEFAULT_MAX_RECORD_SIZE)
    }

    pub fn with_max_record_size(format: F, max_record_size: usize) -> Self {
        Self {
            format,
            buffer: StreamBuffer::new(max_record_size),
            next_index: 0,
        }
    }

    /// Total records emitted so far.
    pub fn records_emitted(&self) -> u64 {
        self.next_index
    }

    /// Feeds a chunk of input, returning every record completed by it.
    pub fn feed(&mut self, chunk: &[u8]) -> Result<Vec<ParseEvent>, CoreError> {
        self.buffer.extend(chunk);
        self.drain_complete_records()
    }

    /// Signals end of input, emitting any final unterminated record.
    pub fn finish(&mut self) -> Result<Vec<ParseEvent>, CoreError> {
        let mut events = self.drain_complete_records()?;
        if !self.buffer.is_empty() {
            let record = self.buffer.take_all();
            if !tokenizer::trim(&record).is_empty() {
                let event = self.format.parse_record(&record, self.next_index)?;
                self.next_index += 1;
                events.push(event);
            }
        }
        Ok(events)
    }

    fn drain_complete_records(&mut self) -> Result<Vec<ParseEvent>, CoreError> {
        let terminator = self.format.record_terminator();
        let mut events = Vec::new();

        loop {
            // Deliberate two-step: resolve the terminator index in its own
            // statement so the immutable borrow of `self.buffer` ends at the
            // semicolon, BEFORE the mutable `take_record` below. (A `while let`
            // with the search in the scrutinee would hold the borrow across the
            // loop body and conflict — this avoids that entirely.)
            let next_terminator = tokenizer::find_byte(self.buffer.as_slice(), terminator);
            let end = match next_terminator {
                Some(end) => end,
                None => break,
            };

            let record = self.buffer.take_record(end);
            if tokenizer::trim(&record).is_empty() {
                continue; // skip blank/whitespace-only lines
            }
            let event = self.format.parse_record(&record, self.next_index)?;
            self.next_index += 1;
            events.push(event);
        }

        let buffered = self.buffer.len();
        if buffered > self.buffer.max_record_size() {
            return Err(CoreError::RecordTooLarge {
                got: buffered,
                max: self.buffer.max_record_size(),
            });
        }

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::RecordPayload;

    /// A trivial test format: each line becomes a single-field record.
    struct LineFormat;

    impl Format for LineFormat {
        fn record_terminator(&self) -> u8 {
            b'\n'
        }

        fn parse_record(&self, record: &[u8], index: u64) -> Result<ParseEvent, CoreError> {
            let text = String::from_utf8_lossy(record).into_owned();
            Ok(ParseEvent {
                index,
                raw_len: record.len(),
                payload: RecordPayload::Fields(vec![text]),
            })
        }
    }

    fn line_text(event: &ParseEvent) -> String {
        match &event.payload {
            RecordPayload::Fields(fields) => fields.join(""),
            RecordPayload::Json(text) => text.clone(),
        }
    }

    #[test]
    fn parses_two_lines_in_one_feed() {
        let mut parser = StreamParser::new(LineFormat);
        let events = parser.feed(b"hello\nworld\n").unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].index, 0);
        assert_eq!(line_text(&events[0]), "hello");
        assert_eq!(line_text(&events[1]), "world");
    }

    #[test]
    fn handles_record_split_across_feeds() {
        let mut parser = StreamParser::new(LineFormat);
        assert!(parser.feed(b"hel").unwrap().is_empty());
        let events = parser.feed(b"lo\n").unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(line_text(&events[0]), "hello");
    }

    #[test]
    fn skips_blank_lines_without_consuming_index() {
        let mut parser = StreamParser::new(LineFormat);
        let events = parser.feed(b"a\n\nb\n").unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[1].index, 1);
    }

    #[test]
    fn finish_emits_unterminated_record() {
        let mut parser = StreamParser::new(LineFormat);
        let first = parser.feed(b"x\ny").unwrap();
        assert_eq!(first.len(), 1);
        let last = parser.finish().unwrap();
        assert_eq!(last.len(), 1);
        assert_eq!(line_text(&last[0]), "y");
    }

    #[test]
    fn finish_ignores_whitespace_remainder() {
        let mut parser = StreamParser::new(LineFormat);
        let _ = parser.feed(b"a\n  ").unwrap();
        assert!(parser.finish().unwrap().is_empty());
    }

    #[test]
    fn enforces_max_record_size() {
        let mut parser = StreamParser::with_max_record_size(LineFormat, 4);
        let result = parser.feed(b"abcdef"); // 6 bytes, no terminator
        assert!(matches!(result, Err(CoreError::RecordTooLarge { .. })));
    }

    #[test]
    fn reports_records_emitted() {
        let mut parser = StreamParser::new(LineFormat);
        parser.feed(b"a\nb\n").unwrap();
        assert_eq!(parser.records_emitted(), 2);
    }
}
