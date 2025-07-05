use crate::{errors::{FormatError, ParserError, UnallowedCharacterReason}, reader::char_supplier::Supplier, COMMENT_START, NEWLINE_CR, NEWLINE_LF, SPECIAL_CTRL_CHARACTERS, WHITESPACE_SPACE, WHITESPACE_TAB};

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

    fn is_special_control(&self) -> bool;

    fn is_whitespace(&self) -> bool;

    fn is_comment_start(&self) -> bool;
}

impl CharExt for char {
    fn is_linebreak(&self) -> bool {
        [NEWLINE_LF, NEWLINE_CR].contains(self)
    }

    fn is_special_control(&self) -> bool {
        SPECIAL_CTRL_CHARACTERS.contains(self)
    }

    fn is_whitespace(&self) -> bool {
        [WHITESPACE_SPACE, WHITESPACE_TAB].contains(self)
    }

    fn is_comment_start(&self) -> bool {
        *self == COMMENT_START
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
        if !is_comment && !c.is_comment_start() {
            return ParserError::from::<(),FormatError>(FormatError::ExpectedCharacter(COMMENT_START), offset).err();
        } else if !is_comment {
            is_comment = true;
        }
        
        if c.is_control() && !c.is_special_control() {
            return ParserError::from::<(),FormatError>(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InComment), offset).err();
        }
        
        if let Some(_c) = iter.get() {
            if _c.is_linebreak() {
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