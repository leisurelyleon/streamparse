//! Concrete [`Format`](streamparse_core::Format) implementations for
//! `streamparse`: NDJSON and delimited records.

pub mod delimited;
pub mod ndjson;

pub use delimited::Delimited;
pub use ndjson::Ndjson;
