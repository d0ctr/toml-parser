use crate::{check_comment_or_whitespaces, errors::{FormatError, ParserError, UnallowedCharacterReason}, reader::char_supplier::Supplier, types::NumberType, CharExt as _};

pub struct Number;

impl super::TypeParser<NumberType> for Number {
    fn parse(first: char, iter: &mut impl Supplier) -> Result<NumberType, crate::errors::ParserError> {
        let mut offset = 0;
        let mut is_comment = false;

        let mut signed = true;
        let mut dotted = first == '.';

        let mut value = String::from(first);
        let check_comment = loop {
            let c = if let Some(_c) = iter.get() {
                if _c.is_linebreak() {
                    break false;
                }
                if _c.is_whitespace() {
                    break true;
                }
                _c
            } else {
                break false;
            };
            
            offset += 1;
            
            if c.is_comment_start() {
                is_comment = true;
                break true;
            } else if ('0'..='9').contains(&c) {
                // first digit also sets the sign
                signed = true
            } else if (c == '+' || c == '-') && !signed {
                signed = true;
            } else if c == '.' && !dotted {
                dotted = true;
            } else {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeNumber), offset)
            }
    
            value.push(c);
        };

        if check_comment {
            if let Some(err) = check_comment_or_whitespaces(iter, is_comment) {
                return ParserError::extend(err, offset)
            }
        }

        if dotted {
            return match value.parse::<f64>() {
                Ok(v) => Ok(NumberType::Float(v)),
                Err(err) => ParserError::from(err, offset),
            }
        } else {
            return match value.parse::<isize>() {
                Ok(v) => Ok(NumberType::Integer(v)),
                Err(err) => ParserError::from(err, offset),
            }
        }
    }
}