use crate::errors::{FormatError, ParserError, UnallowedCharacterReason};
use crate::reader::char_supplier::ToSupplier;
use crate::{reader::char_supplier::Supplier, types::NumberType, CharExt as _};

pub struct Number;

impl Number {
    pub fn parse_with_buf(buf: &mut impl Supplier, input: &mut impl Supplier) -> Result<NumberType, crate::errors::ParserError> {
        let mut dotted = false;
        let mut signed = false;

        let mut value = String::new();
        let mut from_buf = true;

        loop {
            let next = if from_buf {
                let _next = buf.get();
                if _next.is_none() {
                    from_buf = false;
                    input.get()
                } else {
                    _next
                }
            } else {
                input.get()
            };

            let c = if let Some(_c) = next {
                if _c.is_linebreak() || _c.is_whitespace() || _c.is_comment_start() {
                    break;
                }
                _c
            } else {
                break;
            };

            if c == '.' && !dotted {
                dotted = true;
            } else if !c.is_digit(10) && !(['+', '-'].contains(&c) && !signed) {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeNumber));
            }
            
            signed = true;
            value.push(c);
        }

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

impl super::TypeParser<NumberType> for Number {
    fn parse(first: char, input: &mut impl Supplier) -> Result<NumberType, crate::errors::ParserError> {
        let first_as_string = first.to_string();
        let mut buf = ToSupplier::from_string(&first_as_string);

        return Self::parse_with_buf(&mut buf, input)
    }
}
