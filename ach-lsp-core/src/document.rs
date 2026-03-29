//! Text document utilities.
//!
//! Provides word extraction at a given position.
//! No thread-safe document store — that stays in the LSP server.

use crate::types::{Position, Range};

/// Extract the word under the cursor and its range.
///
/// A "word" is a contiguous run of `[a-zA-Z0-9_]`. Returns `None` if the
/// cursor is not on a word character.
/// Line and col are 0-based.
pub fn word_at_position(text: &str, line: u32, col: u32) -> Option<(String, Range)> {
    let line_str = text.lines().nth(line as usize)?;
    let col = col as usize;
    if col >= line_str.len() {
        return None;
    }

    let bytes = line_str.as_bytes();
    if !is_word_char(bytes[col]) {
        return None;
    }

    let mut start = col;
    while start > 0 && is_word_char(bytes[start - 1]) {
        start -= 1;
    }

    let mut end = col;
    while end < bytes.len() && is_word_char(bytes[end]) {
        end += 1;
    }

    let word = line_str[start..end].to_string();
    let range = Range::new(
        Position::new(line, start as u32),
        Position::new(line, end as u32),
    );
    Some((word, range))
}

fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_at_middle() {
        let (word, range) = word_at_position("let x = poseidon(a, b)", 0, 10).unwrap();
        assert_eq!(word, "poseidon");
        assert_eq!(range.start.character, 8);
        assert_eq!(range.end.character, 16);
    }

    #[test]
    fn word_at_start() {
        let (word, _) = word_at_position("let x = 1", 0, 0).unwrap();
        assert_eq!(word, "let");
    }

    #[test]
    fn word_at_end() {
        let (word, _) = word_at_position("let x = 1", 0, 8).unwrap();
        assert_eq!(word, "1");
    }

    #[test]
    fn no_word_on_space() {
        assert!(word_at_position("let x = 1", 0, 3).is_none());
    }

    #[test]
    fn no_word_past_line() {
        assert!(word_at_position("let", 0, 99).is_none());
    }

    #[test]
    fn multiline() {
        let text = "let x = 1\nlet y = poseidon(x, x)";
        let (word, range) = word_at_position(text, 1, 8).unwrap();
        assert_eq!(word, "poseidon");
        assert_eq!(range.start.line, 1);
    }
}
