use super::types;
use super::types::TypeParser;
use crate::{check_comment_or_whitespaces, errors::{FormatError, ParserError}, reader::char_supplier::{Supplier, ToSupplier}, CharExt};

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
            let buf = String::from(c);
            let mut supplier = ToSupplier::from_string(&buf);
            types::Number::parse_with_buf(&mut supplier, input).map(|v| types::Value::Number(v))
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
