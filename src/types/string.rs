use crate::{skip_whitespaces, DOUBLE_QUOTE_STR, DOUBLE_QUOTE_THRICE, LINE_ENDING_BACKSLASH, NEWLINE_CR, NEWLINE_LF, SINGLE_QUOTE_STR, SINGLE_QUOTE_THRICE, UNICODE_HIGH_ESCAPE_START, UNICODE_LOW_ESCAPE_START, WHITESPACE_TAB};
use crate::{reader::char_supplier::Supplier, types::TypeParser, CharExt, DOUBLE_QUOTE, ESCAPE_START, SINGLE_QUOTE};
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
fn read_escape_seq(iter: &mut impl Supplier, is_multiline: bool) -> Result<char,FormatError> { 
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
                        Some(c) => Ok(c),
                        None => if is_multiline && seq.as_str() == LINE_ENDING_BACKSLASH {
                            match skip_whitespaces(iter, false) {
                                Some(c) => Ok(c),
                                None => Err(FormatError::UnexpectedEnd)
                            }
                        } else {
                            Err(FormatError::UnknownEscapeSequence)
                        }
                    };
                }
            },
            EscapeSequenceType::HighUnicode if len == 6 => {
                return match codepoint_to_char(&seq) {
                    Some(c) => Ok(c),
                    None => Err(FormatError::UnknownEscapeSequence)
                }
            },
            EscapeSequenceType::LowUnicode if len == 4 => {
                return match codepoint_to_char(&seq) {
                    Some(c) => Ok(c),
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

    fn quotes(&self) -> &str {
        match self {
            StringType::Basic => DOUBLE_QUOTE_STR,
            StringType::Literal => SINGLE_QUOTE_STR,
            StringType::BasicMultiline => DOUBLE_QUOTE_THRICE,
            StringType::LiteralMultiline => SINGLE_QUOTE_THRICE
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

    fn parse(self, first: char, iter: &mut impl Supplier) -> Result<std::string::String, crate::errors::ParserError> {
        match self {
            Self::Basic => StringType::parse_as_basic(first, iter),
            Self::Literal => StringType::parse_as_literal(first, iter),
            Self::BasicMultiline => StringType::parse_as_basic_multiline(first, iter),
            Self::LiteralMultiline => StringType::parse_as_literal_multiline(first, iter),
        }
    }

    fn parse_as_basic(first: char, iter: &mut impl Supplier) -> Result<std::string::String, crate::errors::ParserError> {
        const TYPE: StringType = StringType::Basic;

        let mut value = std::string::String::new();

        let mut c = first;

        loop {
            if TYPE.is_type_quote(&c) {
                break; 
            }

            if c.is_special_control() {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeBasicString));
            }

            if c == ESCAPE_START {
                match read_escape_seq(iter, false) {
                    Ok(replacement) => {
                        c = replacement;
                    },
                    Err(err) => return ParserError::from(err)
                }
            }

            value.push(c);

            c = if let Some(_c) = iter.get() {
                if _c.is_linebreak() {
                    return ParserError::from(FormatError::ExpectedCharacter(TYPE.quote()));
                }
                _c
            } else {
                return ParserError::from(FormatError::ExpectedCharacter(TYPE.quote()));
            }
        }

        Ok(value)
    }

    fn parse_as_literal(first: char, iter: &mut impl Supplier) -> Result<std::string::String, crate::errors::ParserError> {
        const TYPE: StringType = StringType::Literal;

        let mut value = std::string::String::new();

        let mut c = first;
        loop {
            if TYPE.is_type_quote(&c) {
                break; 
            }

            if c.is_control() && c != WHITESPACE_TAB {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeLiteralString));
            }

            value.push(c);
            c = if let Some(_c) = iter.get() {
                if _c.is_linebreak() {
                    return ParserError::from(FormatError::ExpectedCharacter(TYPE.quote()));
                }
                _c
            } else {
                return ParserError::from(FormatError::ExpectedCharacter(TYPE.quote()));
            }
        }

        Ok(value)
    }

    fn parse_as_basic_multiline(first: char, iter: &mut impl Supplier) -> Result<std::string::String, crate::errors::ParserError> {
        const TYPE: StringType = StringType::BasicMultiline;

        let mut value = std::string::String::new();
        let mut quotes: u8 = 0b1;

        let mut c: char = if first.is_linebreak() {
            if let Some(_c) = iter.get() {
                _c
            } else {
                return ParserError::from(FormatError::ExpectedSequence(TYPE.quotes().to_string()));
            }
        } else {
            first
        };

        loop {
            if TYPE.is_type_quote(&c) {
                quotes <<= 0b1;

                if quotes == 0b1000 {
                    break;
                }
            } else {
                while quotes != 0b1 {
                    value.push(TYPE.quote());
                    quotes >>= 0b1;
                }
            }

            if c.is_special_control() && ![NEWLINE_CR, NEWLINE_LF, WHITESPACE_TAB].contains(&c) {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeMultilineBasicString));
            }

            if c == ESCAPE_START {
                match read_escape_seq(iter, true) {
                    Ok(replacement) => {
                        c = replacement;
                    },
                    Err(err) => match err {
                        FormatError::UnexpectedEnd => return ParserError::from(FormatError::ExpectedSequence(TYPE.quotes().to_string())),
                        _ => return ParserError::from(err),
                    }
                }
            }

            if quotes == 0b1 {
                value.push(c);
            } 

            c = if let Some(_c) = iter.get() {
                _c
            } else {
                return ParserError::from(FormatError::ExpectedSequence(TYPE.quotes().to_string()));
            }
        }

        Ok(value)
    }

    fn parse_as_literal_multiline(first: char, iter: &mut impl Supplier) -> Result<std::string::String, crate::errors::ParserError> {
        const TYPE: StringType = StringType::LiteralMultiline;

        let mut value = std::string::String::new();
        let mut quotes: u8 = 0b1;

        let mut c: char = if first.is_linebreak() {
            if let Some(_c) = iter.get() {
                _c
            } else {
                return ParserError::from(FormatError::ExpectedSequence(TYPE.quotes().to_string()));
            }
        } else {
            first
        };

        loop {
            if TYPE.is_type_quote(&c) {
                quotes <<= 0b1;

                if quotes == 0b1000 {
                    break;
                }
            } else {
                while quotes != 0b1 {
                    value.push(TYPE.quote());
                    quotes >>= 0b1;
                }
            }

            if c.is_control() && ![NEWLINE_CR, NEWLINE_LF, WHITESPACE_TAB].contains(&c) {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeMultilineLiteralString));
            }

            if quotes == 0b1 {
                value.push(c);
            } 

            c = if let Some(_c) = iter.get() {
                _c
            } else {
                return ParserError::from(FormatError::ExpectedSequence(TYPE.quotes().to_string()));
            }
        }

        Ok(value)
    }
}

impl TypeParser<std::string::String> for String {
    fn parse(first: char, iter: &mut impl Supplier) -> Result<std::string::String, crate::errors::ParserError> {
        let mut string_type = if StringType::Basic.is_type_quote(&first) {
            StringType::Basic
        } else {
            StringType::Literal
        };
        
        let mut quotes: u8 = 0b1;
        let (is_empty_string, first) = loop {
            let c = if let Some(_c) = iter.get() {
                _c
            } else {
                return ParserError::from(FormatError::EmptyValue)
            };
            
            if quotes == 0b100 {
                string_type = string_type.to_multiline();
                break (false, c);
            } else if string_type.is_type_quote(&c) {
                quotes <<= 0b1;
            } else if quotes == 0b010 {
                break (true, c);
            } else {
                break (false, c);
            }
        };

        return if is_empty_string {
            Ok(std::string::String::new())
        } else {
            string_type.parse(first, iter)
        };
    }
}