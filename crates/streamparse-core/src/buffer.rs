//! The bounded accumulator that holds partially-parsed input between feeds.

/// Default cap on a single record's size (1 MiB), bounding peak memory.
pub const DEFAULT_MAX_RECORD_SIZE: usize = 1 << 20;

/// Accumulates fed bytes and yields complete records. Peak memory is bounded by
/// the largest single record, never the total input size.
#[derive(Debug)]
pub struct StreamBuffer {
    data: Vec<u8>,
    max_record_size: usize,
}

impl StreamBuffer {
    pub fn new(max_record_size: usize) -> Self {
        Self {
            data: Vec::new(),
            max_record_size,
        }
    }

    /// Appends a fed chunk.
    pub fn extend(&mut self, chunk: &[u8]) {
        self.data.extend_from_slice(chunk);
    }

    /// The currently buffered bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn max_record_size(&self) -> usize {
        self.max_record_size
    }

    /// Removes the record ending at the terminator located at `content_end`,
    /// returning the record's bytes (excluding the terminator). The record and
    /// its terminator are drained from the front of the buffer.
    pub fn take_record(&mut self, content_end: usize) -> Vec<u8> {
        let record = self.data[..content_end].to_vec();
        self.data.drain(..=content_end);
        record
    }

    /// Removes and returns all remaining buffered bytes.
    pub fn take_all(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_record_removes_content_and_terminator() {
        let mut buf = StreamBuffer::new(1024);
        buf.extend(b"hello\nworld");
        let record = buf.take_record(5); // terminator at index 5
        assert_eq!(record.as_slice(), &b"hello"[..]);
        assert_eq!(buf.as_slice(), &b"world"[..]);
    }

    #[test]
    fn take_all_drains_the_buffer() {
        let mut buf = StreamBuffer::new(1024);
        buf.extend(b"abc");
        let all = buf.take_all();
        assert_eq!(all.as_slice(), &b"abc"[..]);
        assert!(buf.is_empty());
    }
}
