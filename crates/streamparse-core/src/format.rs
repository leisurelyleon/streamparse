//! The Format trait — the seam between the streaming engine and a concrete
//! grammar. The engine drives any `Format` incrementally; the format decides
//! how a complete record's bytes become an event.

use crate::error::CoreError;
use crate::event::ParseEvent;

/// Defines how a byte stream is divided into records and how each record is
/// decoded. Implementations live in `streamparse-formats`.
pub trait Format {
    /// The byte that terminates one record (e.g. `b'\n'`).
    fn record_terminator(&self) -> u8;

    /// Decodes one complete record's bytes (without the terminator) into an
    /// event. `record` borrows the parser's buffer and is not retained.
    fn parse_record(&self, record: &[u8], index: u64) -> Result<ParseEvent, CoreError>;
}
