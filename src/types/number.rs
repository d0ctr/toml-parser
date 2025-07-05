use crate::errors::{FormatError, ParserError, UnallowedCharacterReason};
use crate::{reader::char_supplier::Supplier, types::NumberType, CharExt as _};

pub struct Number;

impl super::TypeParser<NumberType> for Number {
    fn parse(first: char, iter: &mut impl Supplier) -> Result<NumberType, crate::errors::ParserError> {
        let mut dotted = first == '.';

        let mut value = String::from(first);
        loop {
            let c = if let Some(_c) = iter.get() {
                if _c.is_linebreak() || _c.is_whitespace() || _c.is_comment_start() {
                    break;
                }
                _c
            } else {
                break;
            };

            if c == '.' && !dotted {
                dotted = true;
            } else if !c.is_digit(10) {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeNumber));
            }
    
            value.push(c);
        };

        if dotted {
            return match value.parse::<f64>() {
                Ok(v) => Ok(NumberType::Float(v)),
                Err(err) => ParserError::from(err),
            }
        } else {
            return match value.parse::<isize>() {
                Ok(v) => Ok(NumberType::Integer(v)),
                Err(err) => ParserError::from(err),
            }
        }
    }
}