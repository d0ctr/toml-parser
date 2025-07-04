use crate::{errors::{FormatError, ParserError, UnallowedCharacterReason}, reader::char_supplier::Supplier, ALLOWED_CTRL_CHARACTERS, NEWLINE_CHARS};

pub fn skip_whitespaces(iter: &mut impl Supplier) -> Option<(usize,char)> {
    let mut pos = 0;
    while let Some(c) = iter.get() {
        if c.is_linebreak() {
            return None;
        }
        if !c.is_whitespace() {
            return Some((pos,c));
        }
        pos += 1
    }

    None
}

pub trait CharExt {
    fn is_linebreak(&self) -> bool;
}

impl CharExt for char {
    fn is_linebreak(&self) -> bool {
        NEWLINE_CHARS.contains(self)
    }
}

pub fn check_comment_or_whitespaces(iter: &mut impl Supplier, is_comment: bool) -> Option<ParserError> {
    let mut offset = 0;
    let mut is_comment = is_comment;
    let mut c: char;

    if let Some((pos,last_c)) = crate::skip_whitespaces(iter) {
        offset += pos + 1;
        c = last_c;
    } else {
        return None
    }

    loop {
        if !is_comment && c != '#' {
            return ParserError::from::<(),FormatError>(FormatError::ExpectedCharacter('#'), offset).err();
        } else if !is_comment {
            is_comment = true;
        }
        
        if c.is_control() && !ALLOWED_CTRL_CHARACTERS.contains(&c) {
            return ParserError::from::<(),FormatError>(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InComment), offset).err();
        }
        
        if let Some(_c) = iter.get() {
            if NEWLINE_CHARS.contains(&_c) {
                break;
            }

            c = _c;
            offset += 1;
        } else {
            break;
        }
    }

    None
}