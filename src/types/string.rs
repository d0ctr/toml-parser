use crate::{errors::{FormatError, ParserError, UnallowedCharacterReason::{self, InTypeString, InUnicodeSequence}}, types::TypeParser, NEWLINE_CHARS};
use crate::UNICODE_SEQUENCE_CHARS;
use super::common::{Trimmer,find_replacement_char,check_comment_or_whitespaces};

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
fn read_escape_seq(iter: &mut std::str::Chars) -> Result<(char,usize),FormatError> { 
    let mut len: u8 = 1;
    let mut seq = std::string::String::from('\\');
    let mut seq_type = EscapeSequenceType::EscapeChar;

    while let Some(c) = iter.next() {
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
}

impl StringType {
    fn is_type_quote(&self, c: &char) -> bool {
        match self {
            StringType::Literal if c.eq(&'\'') => true,
            StringType::Basic if c.eq(&'"') => true,
            _ => false
        }
    }
}

impl String {
    fn parse_as_basic(first: char, iter: &mut std::str::Chars) -> Result<(std::string::String, usize), crate::errors::ParserError> {
        let mut offset = 0;

        let mut value = std::string::String::new();

        let mut c = first;
        loop {
            if NEWLINE_CHARS.contains(&c) {
                break;
            }

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

            c = if let Some(_c) = iter.next() {
                _c
            } else {
                return ParserError::from(FormatError::ExpectedCharacter('"'), offset);
            }
        }

        Ok((value, offset))
    }
}
impl TypeParser<std::string::String> for String {
    fn parse(first: char, iter: &mut std::str::Chars) -> Result<std::string::String, crate::errors::ParserError> {
        let mut offset = 0;
        let string_type = if first == '"' { StringType::Basic } else { StringType::Literal };
        
        let mut is_multiline: u8 = 0b1;
        let (is_multiline, is_empty_string, c) = loop {
            let c = if let Some(value) = iter.next() {
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

        let value = if is_empty_string {
            std::string::String::new()
        } else {
            match string_type {
                StringType::Basic if !is_multiline => {
                    match String::parse_as_basic(c, iter) {
                        Ok((value, pos)) => {
                            offset += pos;
                            value
                        },
                        Err(err) => return ParserError::extend(err, offset)
                    }
                },
                _ => return ParserError::from(FormatError::EmptyValue, offset)
            }
        };

        if let Some(err) = check_comment_or_whitespaces(iter, false) {
            return ParserError::extend(err, offset)
        }
        
        Ok(value)
    }
}