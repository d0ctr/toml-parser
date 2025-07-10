use std::any::{Any, TypeId};

use super::types;
use super::types::TypeParser;
use crate::{check_comment_or_whitespaces, errors::{FormatError, ParserError}, reader::char_supplier::{Supplier, ToSupplier}, types::{DateTimeType, NumberType}, CharExt};

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
                        buf.push(_c);
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
