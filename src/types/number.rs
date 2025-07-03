use crate::{errors::{FormatError, ParserError, UnallowedCharacterReason}, types::NumberType};
use crate::types::common::{check_comment_or_whitespaces};

pub struct Number;

impl super::TypeParser<NumberType> for Number {
    fn parse(first: char, iter: &mut std::str::Chars) -> Result<NumberType, crate::errors::ParserError> {
        let mut offset = 0;
        let mut is_comment = false;
        let mut _iter = iter.take_while(|c| !c.is_whitespace());

        let mut signed = ['+', '-'].contains(&first);
        let mut dotted = first == '.';

        let mut value = String::from(first);
        while let Some(c) = _iter.next() {
            offset += 1;
            
            if c == '#' {
                is_comment = true;
                break;
            } else if ('0'..='9').contains(&c) {
                // first digit also sets the sign
                signed = true
            } else if (c == '+' || c == '-') && !signed {
                signed = true;
            } else if c == '.' && !dotted {
                dotted = true;
            } else {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeFloat), offset)
            }
    
            value.push(c);
        }

        if let Some(err) = check_comment_or_whitespaces(iter, is_comment) {
            return ParserError::extend(err, offset)
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