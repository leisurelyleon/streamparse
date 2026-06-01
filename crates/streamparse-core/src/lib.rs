//! The push-based incremental parser engine for `streamparse`.
//!
//! Feed bytes in arbitrary chunks via [`StreamParser::feed`]; receive a
//! [`ParseEvent`] per completed record. The engine is format-agnostic: a
//! [`Format`] implementation decides how bytes become events. Tokenization is
//! zero-copy; the parser materializes one owned copy of each record at the
//! emission boundary, since records may span feed chunks.

pub mod buffer;
pub mod error;
pub mod event;
pub mod format;
pub mod parser;
pub mod tokenizer;

pub use buffer::{DEFAULT_MAX_RECORD_SIZE, StreamBuffer};
pub use error::CoreError;
pub use event::{ParseEvent, RecordPayload};
pub use format::Format;
pub use parser::StreamParser;
