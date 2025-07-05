use crate::{UNICODE_HIGH_ESCAPE_START, UNICODE_LOW_ESCAPE_START, WHITESPACE_TAB};
use crate::{check_comment_or_whitespaces, reader::char_supplier::Supplier, types::TypeParser, CharExt, DOUBLE_QUOTE, ESCAPE_START, SINGLE_QUOTE};
use crate::errors::{FormatError, ParserError, UnallowedCharacterReason};
use super::common::to_escaped_char;

pub struct String;

enum EscapeSequenceType {
    LowUnicode,
    HighUnicode,
    EscapedChar
}

impl EscapeSequenceType {
    fn to_unicode_type(start: &str) -> Option<Self> {
        match start {
            UNICODE_LOW_ESCAPE_START => Some(Self::LowUnicode),
            UNICODE_HIGH_ESCAPE_START => Some(Self::HighUnicode),
            _ => None
        }
    }
}

fn codepoint_to_char(codepoint: &str) -> Option<char> {
    match u32::from_str_radix(codepoint, 16) {
        Ok(codepoint) => char::from_u32(codepoint),
        Err(_) => None,
    }
}

/// returns a union of (replacement char, last read)
fn read_escape_seq(iter: &mut impl Supplier) -> Result<(char,usize),FormatError> { 
    let mut len: u8 = 1;
    let mut seq = std::string::String::from(ESCAPE_START);
    let mut seq_type = EscapeSequenceType::EscapedChar;

    while let Some(c) = iter.get() {
        seq.push(c);
        len += 1;

        match seq_type {
            EscapeSequenceType::EscapedChar => {
                if let Some(new_type) = EscapeSequenceType::to_unicode_type(seq.as_str()) {
                    seq_type = new_type;
                    seq.clear();
                    len = 0;
                } else {
                    return match to_escaped_char(seq.as_str()) {
                        Some(c) => Ok((c, len as usize)),
                        None => Err(FormatError::UnknownEscapeSequence),
                    };
                }
            },
            EscapeSequenceType::HighUnicode if len == 6 => {
                return match codepoint_to_char(&seq) {
                    Some(c) => Ok((c, len as usize)),
                    None => Err(FormatError::UnknownEscapeSequence)
                }
            },
            EscapeSequenceType::LowUnicode if len == 4 => {
                return match codepoint_to_char(&seq) {
                    Some(c) => Ok((c, len as usize)),
                    None => Err(FormatError::UnknownEscapeSequence)
                }
            },
            _ => if !c.is_digit(16) {
                return Err(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InUnicodeSequence))
            }
        }
    }

    Err(FormatError::UnknownEscapeSequence)
}

enum StringType {
    Literal,
    Basic,
    LiteralMultiline,
    BasicMultiline,
}

impl StringType {
    fn quote(&self) -> char {
        match self {
            StringType::Basic | StringType::BasicMultiline => DOUBLE_QUOTE,
            StringType::Literal | StringType::LiteralMultiline => SINGLE_QUOTE
        }
    }

    fn is_type_quote(&self, c: &char) -> bool {
        self.quote() == *c
    }

    fn to_multiline(self) -> Self {
        match self {
            StringType::Basic => StringType::BasicMultiline,
            StringType::Literal => StringType::LiteralMultiline,
            same => same,
        }
    }

    fn parse(self, first: char, iter: &mut impl Supplier) -> Result<(std::string::String, usize), crate::errors::ParserError> {
        match self {
            Self::Basic => Self::parse_as_basic(first, iter),
            Self::Literal => Self::parse_as_literal(first, iter),
            _ => ParserError::from(FormatError::EmptyValue, 0)
        }
    }

    fn parse_as_basic(first: char, iter: &mut impl Supplier) -> Result<(std::string::String, usize), crate::errors::ParserError> {
        const TYPE: StringType = StringType::Basic;

        let mut offset = 1;

        let mut value = std::string::String::new();

        let mut c = first;
        loop {
            if TYPE.is_type_quote(&c) {
                break; 
            }

            if c.is_special_control() {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeBasicString), offset);
            }

            if c == ESCAPE_START {
                match read_escape_seq(iter) {
                    Ok((replacement, pos)) => {
                        c = replacement;
                        offset += pos;
                    },
                    Err(err) => return ParserError::from(err, offset)
                }
            }

            value.push(c);
            offset += 1;

            c = if let Some(_c) = iter.get() {
                if _c.is_linebreak() {
                    return ParserError::from(FormatError::ExpectedCharacter(TYPE.quote()), offset);
                }
                _c
            } else {
                return ParserError::from(FormatError::ExpectedCharacter(TYPE.quote()), offset);
            }
        }

        Ok((value, offset))
    }

    fn parse_as_literal(first: char, iter: &mut impl Supplier) -> Result<(std::string::String, usize), crate::errors::ParserError> {
        const TYPE: StringType = StringType::Literal;

        let mut offset = 1;

        let mut value = std::string::String::new();

        let mut c = first;
        loop {
            if TYPE.is_type_quote(&c) {
                break; 
            }

            if c.is_control() && c != WHITESPACE_TAB {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeBasicString), offset);
            }

            value.push(c);
            offset += 1;
            c = if let Some(_c) = iter.get() {
                if _c.is_linebreak() {
                    return ParserError::from(FormatError::ExpectedCharacter(TYPE.quote()), offset);
                }
                _c
            } else {
                return ParserError::from(FormatError::ExpectedCharacter(TYPE.quote()), offset);
            }
        }

        Ok((value, offset))
    }
}

impl TypeParser<std::string::String> for String {
    fn parse(first: char, iter: &mut impl Supplier) -> Result<std::string::String, crate::errors::ParserError> {
        let mut offset = 0;
        let mut string_type = if StringType::Basic.is_type_quote(&first) {
            StringType::Basic
        } else {
            StringType::Literal
        };
        
        let mut is_multiline: u8 = 0b1;
        let (is_multiline, is_empty_string, c) = loop {
            let c = if let Some(value) = iter.get() {
                value
            } else {
                return ParserError::from(FormatError::EmptyValue, 0)
            };
            
            if is_multiline == 0b100 {
                break (true, false, c);
            } else if string_type.is_type_quote(&c) {
                is_multiline <<= 0b1;
            } else if is_multiline == 0b010 {
                break (false, true, c);
            } else {
                break (false, false, c);
            }

            offset += 1;
        };

        if is_multiline {
            string_type = string_type.to_multiline();
        }

        let value = if is_empty_string {
            std::string::String::new()
        } else {
            match string_type.parse(c, iter) {
                Ok((value, pos)) => {
                    offset += pos;
                    value
                },
                Err(err) => return ParserError::extend(err, offset)
            }
        };

        if let Some(err) = check_comment_or_whitespaces(iter, false) {
            return ParserError::extend(err, offset)
        }
        
        Ok(value)
    }
}