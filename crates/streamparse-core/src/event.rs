//! The output of the parser: one event per completed record.

/// A parsed record, owned so it outlives buffer compaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEvent {
    /// 0-based record index in the stream.
    pub index: u64,
    /// Byte length of the raw record (excluding the terminator).
    pub raw_len: usize,
    /// The format-specific payload.
    pub payload: RecordPayload,
}

/// The decoded content of a record, depending on the format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordPayload {
    /// A validated JSON value, stored as its canonical text.
    Json(String),
    /// Delimited fields, each trimmed.
    Fields(Vec<String>),
}
