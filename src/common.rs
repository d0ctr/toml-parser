use crate::{errors::{FormatError, ParserError, UnallowedCharacterReason}, reader::char_supplier::Supplier, COMMENT_START, NEWLINE_CR, NEWLINE_LF, SPECIAL_CTRL_CHARACTERS, WHITESPACE_SPACE, WHITESPACE_TAB};

pub fn skip_whitespaces(input: &mut impl Supplier, stop_at_linebreak: bool) -> Option<char> {
    while let Some(c) = input.get() {
        if stop_at_linebreak && c.is_linebreak() {
            return None;
        }
        if !c.is_whitespace() {
            return Some(c);
        }
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

pub fn check_comment_or_whitespaces(input: &mut impl Supplier, is_comment: bool) -> Option<ParserError> {
    let mut is_comment = is_comment;
    let mut c: char = crate::skip_whitespaces(input, true)?;

    loop {
        if !is_comment && !c.is_comment_start() {
            return ParserError::from::<(),FormatError>(FormatError::ExpectedCharacter(COMMENT_START)).err();
        } else if !is_comment {
            is_comment = true;
        }
        
        if c.is_control() && !c.is_special_control() {
            return ParserError::from::<(),FormatError>(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InComment)).err();
        }
        
        if let Some(_c) = input.get() {
            if _c.is_linebreak() {
                break;
            }

            c = _c;
        } else {
            break;
        }
    }

    None
}

pub struct Counter {
    value: u8,
    max: u8
}

impl Counter {
    pub fn new(max: u8) -> Self {
        Counter {
            value: 0b0,
            max
        }
    }

    pub fn inc(mut self) -> Self {
        if !self.is_capped() {
            self.value += 0b1;
        }
        self
    }

    pub fn is_capped(&self) -> bool {
        self.value >= self.max
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0b0
    }
}