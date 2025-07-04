use core::str;

use crate::errors::{ParserError, FormatError, UnallowedCharacterReason::InComment};
use crate::reader::char_supplier::Supplier;
use crate::{ALLOWED_CTRL_CHARACTERS, NEWLINE_CHARS, STRING_REPLACEMENTS};

pub fn find_replacement_char(sequence: &str) -> Option<char> {
    if let Some(c) = STRING_REPLACEMENTS.iter().find(|entry| entry.0 == sequence).map(|entry| entry.1) {
        return Some(c);
    }

    None
}