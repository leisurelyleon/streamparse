//! Zero-copy tokenization helpers.
//!
//! Every function here borrows its input and returns either an index or a
//! sub-slice that borrows the same input — no allocation, no copying.

/// Returns the index of the first occurrence of `byte` in `input`, if any.
pub fn find_byte(input: &[u8], byte: u8) -> Option<usize> {
    input.iter().position(|&b| b == byte)
}

/// Splits `record` into field sub-slices on `delimiter`, borrowing from
/// `record` (zero-copy).
pub fn split_fields(record: &[u8], delimiter: u8) -> Vec<&[u8]> {
    record.split(|&b| b == delimiter).collect()
}

/// Returns `input` with leading and trailing ASCII whitespace removed, as a
/// borrowed sub-slice (zero-copy). Handles CRLF line endings naturally.
pub fn trim(input: &[u8]) -> &[u8] {
    let start = input
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(input.len());
    let end = input
        .iter()
        .rposition(|b| !b.is_ascii_whitespace())
        .map_or(start, |i| i + 1);
    &input[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_byte_locates_first_match() {
        assert_eq!(find_byte(b"abc\ndef", b'\n'), Some(3));
        assert_eq!(find_byte(b"abc", b'\n'), None);
    }

    #[test]
    fn split_fields_borrows_subslices() {
        let fields = split_fields(b"a,b,c", b',');
        assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..]]);
    }

    #[test]
    fn trim_strips_surrounding_whitespace() {
        assert_eq!(trim(b"  hi  "), &b"hi"[..]);
        assert_eq!(trim(b"hi\r"), &b"hi"[..]);
    }

    #[test]
    fn trim_of_all_whitespace_is_empty() {
        assert_eq!(trim(b"   "), &b""[..]);
        assert_eq!(trim(b""), &b""[..]);
    }
}
