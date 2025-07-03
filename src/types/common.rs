use core::str;

use crate::errors::{ParserError, FormatError, UnallowedCharacterReason::InComment};
use crate::{ALLOWED_CTRL_CHARACTERS, NEWLINE_CHARS, STRING_REPLACEMENTS};

/**
 always returns a bytes buffer, it may be emppty 
 */
pub fn read_line(reader: &mut dyn std::io::Read) -> std::vec::Vec<u8> {
    let mut buf = std::vec::Vec::<u8>::new();
    let mut last_byte = [0_u8];
    loop {
        let size = match reader.read(&mut last_byte) {
            Ok(size) => size,
            Err(_) => 0,
        };

        if size == 0 || last_byte[0] == b'\n' || last_byte[0] == b'\r' {
            break buf;
        }

        buf.push(last_byte[0]);
    }
}

pub fn check_comment_or_whitespaces(iter: &mut std::str::Chars, is_comment: bool) -> Option<ParserError> {
    let mut offset = 0;
    let mut is_comment = is_comment;
    let mut c: char;

    if let Some((pos,last_c)) = crate::skip_whitespaces(iter) {
        offset += pos + 1;
        c = last_c;
    } else {
        return None
    }

    let mut _iter = iter.take_while(|c| !NEWLINE_CHARS.contains(c));

    loop {
        if !is_comment && c != '#' {
            return ParserError::from::<(),FormatError>(FormatError::ExpectedCharacter('#'), offset).err();
        } else if !is_comment {
            is_comment = true;
        }
        
        if c.is_control() && !ALLOWED_CTRL_CHARACTERS.contains(&c) {
            return ParserError::from::<(),FormatError>(FormatError::UnallowedCharacter(c, InComment), offset).err();
        }
        
        if let Some(_c) = _iter.next() {
            c = _c;
            offset += 1;
        } else {
            break;
        }
    }

    None
}

pub trait Trimmer {
    /// Trims leading whitespace character and returns a union of the trimmed string and the difference in bytes between versions
    fn trim_start_with_difference(&self) -> (&str,usize);
}

impl Trimmer for str {
    fn trim_start_with_difference(&self) -> (&str,usize) {
        let mut count = self.len();
        let line = self.trim_start();
        count -= line.len();
    
        (line, count)
    }
}

impl Trimmer for String {
    fn trim_start_with_difference(&self) -> (&str,usize) {
        let mut count = self.len();
        let line = self.trim_start();
        count -= line.len();
    
        (line, count)
    }

}

pub fn find_replacement_char(sequence: &str) -> Option<char> {
    if let Some(c) = STRING_REPLACEMENTS.iter().find(|entry| entry.0 == sequence).map(|entry| entry.1) {
        return Some(c);
    }

    None
}