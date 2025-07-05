use super::types;
use super::types::TypeParser;
use crate::{errors::{FormatError, ParserError}, reader::char_supplier::Supplier};

pub fn parse_value(iter: &mut impl Supplier) -> Result<types::ParsedValue,ParserError> {
    let mut offset = 1;
    let c: char;

    // skip whitespaces at the start
    if let Some((pos,last_c)) = crate::skip_whitespaces(iter) {
        offset += pos;
        c = last_c;
    } else {
        return ParserError::from(FormatError::EmptyValue, offset)
    }

    match if ['"', '\''].contains(&c) {
        types::String::parse(c, iter).map(|v| types::ParsedValue::String(v))
    } else if ['t', 'f'].contains(&c) {
        types::Boolean::parse(c, iter).map(|v| types::ParsedValue::Boolean(v))
    } else if c.is_ascii_digit() || ['+', '-', '.'].contains(&c) {
        types::Number::parse(c, iter).map(|v| types::ParsedValue::Number(v))
    } else {
        ParserError::from(FormatError::EmptyValue, offset)
    } {
        Ok(value) => Ok(value),
        Err(err) => ParserError::extend(err, offset),
    }

}