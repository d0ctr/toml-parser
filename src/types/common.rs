use core::str;

use crate::ESCAPE_SEQUENCE_TO_CHAR;

pub fn to_escaped_char(sequence: &str) -> Option<char> {
    ESCAPE_SEQUENCE_TO_CHAR
            .iter()
            .find(|entry| entry.0 == sequence)
            .map(|entry| entry.1)
}