use crate::errors::{FormatError, ParserError, UnallowedCharacterReason::InTypeBoolean};
use crate::reader::char_supplier::Supplier;
use crate::{check_comment_or_whitespaces, CharExt as _};

pub struct Boolean;

impl super::TypeParser<bool> for Boolean {
    fn parse(first: char, iter: &mut impl Supplier) -> Result<bool, crate::errors::ParserError> {
        let mut offset = 0;
        let mut is_comment = false;

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
            
            if c == '#' {
                is_comment = true;
                break true;
            } else if !['t','r','u','e','f','a','l','s'].contains(&c) {
                return ParserError::from(FormatError::UnallowedCharacter(c, InTypeBoolean), offset)
            } else {
                value.push(c);    
            }
        };

        if check_comment {
            if let Some(err) = check_comment_or_whitespaces(iter, is_comment) {
                return ParserError::extend(err, offset)
            }
        }

        match value.parse::<bool>() {
            Ok(v) => Ok(v),
            Err(err) => ParserError::from(err, 0),
        }
    }
}