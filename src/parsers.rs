use std::any::Any as _;

use super::types;
use crate::{check_comment_or_whitespaces, errors::{FormatError, ParserError, UnallowedCharacterReason}, reader::char_supplier::{Supplier, ToSupplier}, types::StringType, CharExt};


// parse should assume that iterator will read indefinetely, so line breaks should be handled accordingly
pub trait TypeParser<T> {
    fn parse(first: char, input: &mut impl Supplier) -> Result<T, crate::errors::ParserError>;
}

pub struct ValueParser;

impl ValueParser {
    pub fn parse(input: &mut impl Supplier) -> Result<types::Value,ParserError> {
        let c: char = if let Some(_c) = crate::skip_whitespaces(input, true) {
            _c
        } else {
            return ParserError::from(FormatError::EmptyValue)
        };

        let result = if ['"', '\''].contains(&c) {
            types::String::parse(c, input).map(|v| types::Value::String(v))
        } else if ['t', 'f'].contains(&c) {
            types::Boolean::parse(c, input).map(|v| types::Value::Boolean(v))
        } else if ['+', '-', '.'].contains(&c) {
            types::Number::parse(c, input).map(|v| types::Value::Number(v))
        } else if c.is_ascii_digit() {
            let mut buf = String::from(c);
            let mut len: u8 = 1;
            match loop {
                if let Some(_c) = input.get() {
                    len += 1;

                    if _c.is_ascii_digit() {
                        buf.push(_c);
                    } else if (len == 3 && _c == ':') || (len == 5 && _c == '-') {
                        buf.push(_c);
                        break Some(types::DateTime.type_id());
                    } else if _c == '.' {
                        buf.push(_c);
                        break Some(types::Number.type_id());
                    } else {
                        break Some(types::Number.type_id());
                    }

                    if len > 4 {
                        break Some(types::Number.type_id());
                    }
                } else {
                    break Some(types::Number.type_id());
                }
            } {
                Some(type_id) => {
                    let mut buf = ToSupplier::from_string(&buf);

                    if type_id == types::Number.type_id() {
                        types::Number::parse_with_buf(&mut buf, input).map(|v| types::Value::Number(v))
                    } else if type_id == types::DateTime.type_id() {
                        types::DateTime::parse_with_buf(&mut buf, input).map(|v| types::Value::DateTime(v))
                    } else {
                        ParserError::from(FormatError::EmptyValue)
                    }

                },
                None => ParserError::from(FormatError::EmptyValue)
            }
        } else {
            ParserError::from(FormatError::EmptyValue)
        };

        if result.is_ok() {
            if let Some(c) = input.last().take_if(|c| !c.is_linebreak()) {
                if let Some(err) = check_comment_or_whitespaces(input, c.is_comment_start()) {
                    return ParserError::extend(err);
                }
            }
        }

        result
    }
}

pub struct KeyParser;

impl KeyParser {
    fn parse_segment(first: Option<char>, input: &mut impl Supplier) -> Result<(types::Key,bool),ParserError> {
        let mut c = if let Some(_c) = first {
            _c
        } else {
            if let Some(_c) = crate::skip_whitespaces(input, false) {
                _c
            } else {
                return ParserError::from(FormatError::EmptyValue)
            }
        };
        let mut key = std::string::String::new();

        if c == '"' || c == '\'' {
            key = if c == '"' {
                if let Some(_c) = input.get() {
                    StringType::Basic.parse(_c, input)?
                } else {
                    return ParserError::from(FormatError::UnexpectedEnd);
                }
            } else {
                if let Some(_c) = input.get() {
                    StringType::Literal.parse(_c, input)?
                } else {
                    return ParserError::from(FormatError::UnexpectedEnd);
                }
            };
        } else {
            loop {
                if c.is_ascii_alphanumeric() || ['_', '-'].contains(&c) {
                    key.push(c);
                } else if c == '.' || c == '=' {
                    if key.len() == 0 {
                        return ParserError::from(FormatError::EmptyValue);
                    }
                    return Ok((types::Key::new(key), c == '='));
                } else if c.is_whitespace() && !c.is_linebreak() {
                    break;
                } else {
                    return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InKey));
                }

                c = if let Some(_c) = input.get() {
                    _c
                } else {
                    return ParserError::from(FormatError::UnexpectedEnd);
                };
            }
        }

        if let Some(c) = crate::skip_whitespaces(input, true) {
            if c == '=' || c == '.' {
                return Ok((types::Key::new(key), c == '='));
            } else {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InKey))
            }
        } else {
            return ParserError::from(FormatError::EmptyValue)
        };
    }

}

pub fn parse_entry(input: &mut impl Supplier) -> Result<types::Entry,ParserError> {
    let mut path: Vec<types::Key> = std::vec::Vec::new();

    let mut c = loop {
        if let Some(_c) = crate::skip_whitespaces(input, false) {
            if _c.is_comment_start() {
                if let Some(err) = check_comment_or_whitespaces(input, true) {
                    return ParserError::extend(err);
                }
                continue;
            }
            break Some(_c);
        } else {
            return ParserError::from(FormatError::EmptyValue)
        };
    };

    
    let (key, is_done) = KeyParser::parse_segment(c, input)?;

    let value = if is_done {
        ValueParser::parse(input)?
    } else {
        let mut map = std::boxed::Box::new(std::collections::HashMap::new());
        let mut inner_map = &mut map;
        loop {
            let (key, is_done) = KeyParser::parse_segment(None, input)?;

            if is_done {
                let value = ValueParser::parse(input)?;
                inner_map.insert(key, value);
                break;
            } else {
                let mut new_inner_map = std::boxed::Box::new(std::collections::HashMap::new());
                inner_map.insert(key, types::Value::Nested(new_inner_map.clone()));
                inner_map = &mut new_inner_map;
            }
        }

        types::Value::Nested(map)
    }


    Ok(types::Entry::new(key, value))
}
