use crate::{errors::{ParserError, UnallowedCharacter, UnallowedCharacterReason::{InComment,InLine}}, COMMENT_START, CTRL_CHARACTERS};

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

pub fn check_comment_or_whitespaces(comment: &str) -> Option<ParserError> {
    let mut offset = comment.len();
    let line = comment.trim();
    offset -= line.len();

    if let Some(c) = line.chars().next() {
        if c != COMMENT_START {
            return ParserError::from::<(),UnallowedCharacter>(UnallowedCharacter::new(c, InLine), offset).err();
        }
    } else {
        return None;
    }

    for (i, c) in line.chars().enumerate() {

        if CTRL_CHARACTERS.contains(&c) {
            return ParserError::from::<(),UnallowedCharacter>(UnallowedCharacter::new(c, InComment), offset + i).err();
        }
    }
    None
}

pub trait FromUtf8Trimmed {
    fn from_utf8_trimmed_line(buf: std::vec::Vec<u8>) -> Result<String,ParserError>;
}

impl FromUtf8Trimmed for String {
    fn from_utf8_trimmed_line(buf: std::vec::Vec<u8>) -> Result<String,ParserError> {
        let mut line = match String::from_utf8(buf) {
            Ok(v) => v,
            Err(err) => return ParserError::from(err, 0),
        };
    
        if let Some(split_index) = line.find(COMMENT_START) {
            let comment = line.split_off(split_index);
            if let Some(err) = check_comment_or_whitespaces(&comment) {
                return ParserError::extend(err, line.len());
            }
        }
    
        Ok(line.trim().to_string())
    }
}

