use crate::{check_comment_or_whitespaces, errors::{FormatError, ParserError, UnallowedCharacterReason}, reader::char_supplier::Supplier, types::TypeParser, NEWLINE_CHARS};
use crate::UNICODE_SEQUENCE_CHARS;
use super::common::find_replacement_char;

pub struct String;

enum EscapeSequenceType {
    ShortUnicode,
    LongUnicode,
    EscapeChar
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
    let mut seq = std::string::String::from('\\');
    let mut seq_type = EscapeSequenceType::EscapeChar;

    while let Some(c) = iter.get() {
        seq.push(c);
        len += 1;

        match seq_type {
            EscapeSequenceType::EscapeChar => {
                match seq.as_str() {
                    r"\u" => {
                        seq_type = EscapeSequenceType::ShortUnicode;
                        seq.clear();
                        len = 0;
                    },
                    r"\U" => {
                        seq_type = EscapeSequenceType::LongUnicode;
                        seq.clear();
                        len = 0;
                    },
                    value => return match find_replacement_char(value) {
                        Some(c) => Ok((c, len as usize)),
                        None => Err(FormatError::UnknownEscapeSequence),
                    },
                }
            },
            EscapeSequenceType::LongUnicode if len == 6 => {
                return match codepoint_to_char(&seq) {
                    Some(c) => Ok((c, len as usize)),
                    None => Err(FormatError::UnknownEscapeSequence)
                }
            },
            EscapeSequenceType::ShortUnicode if len == 4 => {
                return match codepoint_to_char(&seq) {
                    Some(c) => Ok((c, len as usize)),
                    None => Err(FormatError::UnknownEscapeSequence)
                }
            },
            _ => if !UNICODE_SEQUENCE_CHARS.contains(&c) {
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
    fn is_type_quote(&self, c: &char) -> bool {
        match self {
            StringType::Literal | StringType::LiteralMultiline if c.eq(&'\'') => true,
            StringType::Basic | StringType::BasicMultiline if c.eq(&'"') => true,
            _ => false
        }
    }

    fn to_multiline(self) -> Self {
        match self {
            StringType::Basic => StringType::BasicMultiline,
            StringType::Literal => StringType::LiteralMultiline,
            same => same,
        }
    }

    fn is_multiline(self) -> bool {
        match self {
            StringType::BasicMultiline | StringType::LiteralMultiline => true,
            _ => false,
        }
    }

    fn parse(self, first: char, iter: &mut impl Supplier) -> Result<(std::string::String, usize), crate::errors::ParserError> {
        match self {
            Self::Basic => Self::parse_as_basic(first, iter),
            _ => ParserError::from(FormatError::EmptyValue, 0)
        }
    }

    fn parse_as_basic(first: char, iter: &mut impl Supplier) -> Result<(std::string::String, usize), crate::errors::ParserError> {
        let mut offset = 0;

        let mut value = std::string::String::new();

        let mut c = first;
        loop {
            offset += 1;
            if c == '"' {
                break; 
            }

            if c == '\\' {
                match read_escape_seq(iter) {
                    Ok((replacement, pos)) => {
                        c = replacement;
                        offset += pos;
                    },
                    Err(err) => return ParserError::from(err, offset)
                }
            }

            value.push(c);

            c = if let Some(_c) = iter.get() {
                if NEWLINE_CHARS.contains(&_c) {
                    return ParserError::from(FormatError::ExpectedCharacter('"'), offset);
                }
                _c
            } else {
                return ParserError::from(FormatError::ExpectedCharacter('"'), offset);
            }
        }

        Ok((value, offset))
    }
}

impl TypeParser<std::string::String> for String {
    fn parse(first: char, iter: &mut impl Supplier) -> Result<std::string::String, crate::errors::ParserError> {
        let mut offset = 0;
        let mut string_type = if first == '"' { StringType::Basic } else { StringType::Literal };
        
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