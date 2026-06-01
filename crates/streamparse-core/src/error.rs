//! Core error type.

/// Errors produced by the incremental parser.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("record exceeds maximum size: buffered {got} bytes, limit {max}")]
    RecordTooLarge { got: usize, max: usize },

    #[error("invalid record at index {index}: {message}")]
    InvalidRecord { index: u64, message: String },
}
