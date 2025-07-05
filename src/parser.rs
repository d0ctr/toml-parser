use super::types;
use super::types::TypeParser;
use crate::{check_comment_or_whitespaces, errors::{FormatError, ParserError}, reader::char_supplier::Supplier, CharExt};

pub fn parse_value(iter: &mut impl Supplier) -> Result<types::ParsedValue,ParserError> {
    let c: char = if let Some(_c) = crate::skip_whitespaces(iter, true) {
        _c
    } else {
        return ParserError::from(FormatError::EmptyValue)
    };

    let result = if ['"', '\''].contains(&c) {
        types::String::parse(c, iter).map(|v| types::ParsedValue::String(v))
    } else if ['t', 'f'].contains(&c) {
        types::Boolean::parse(c, iter).map(|v| types::ParsedValue::Boolean(v))
    } else if c.is_ascii_digit() || ['+', '-', '.'].contains(&c) {
        types::Number::parse(c, iter).map(|v| types::ParsedValue::Number(v))
    } else {
        ParserError::from(FormatError::EmptyValue)
    };

    if result.is_ok() {
        if let Some(c) = iter.last() {
            if !c.is_linebreak() {
                if let Some(err) = check_comment_or_whitespaces(iter, c.is_comment_start()) {
                    return ParserError::extend(err);
                }
            }
        }
    }

    result
}