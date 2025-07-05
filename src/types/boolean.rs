use crate::errors::{FormatError, ParserError, UnallowedCharacterReason::InTypeBoolean};
use crate::reader::char_supplier::Supplier;
use crate::CharExt as _;

pub struct Boolean;

impl super::TypeParser<bool> for Boolean {
    fn parse(first: char, iter: &mut impl Supplier) -> Result<bool, crate::errors::ParserError> {
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
            
            if !['t','r','u','e','f','a','l','s'].contains(&c) {
                return ParserError::from(FormatError::UnallowedCharacter(c, InTypeBoolean))
            }

            value.push(c);
        };

        match value.parse::<bool>() {
            Ok(v) => Ok(v),
            Err(err) => ParserError::from(err),
        }
    }
}