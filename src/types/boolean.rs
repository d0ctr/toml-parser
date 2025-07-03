use crate::errors::{FormatError, ParserError, UnallowedCharacterReason::InTypeBoolean};
use crate::types::common::{check_comment_or_whitespaces};

pub struct Boolean;

impl super::TypeParser<bool> for Boolean {
    fn parse(first: char, iter: &mut std::str::Chars) -> Result<bool, crate::errors::ParserError> {
        let mut offset = 0;
        let mut is_comment = false;
        let mut _iter = iter.take_while(|c| !c.is_whitespace());

        let mut value = String::from(first);
        while let Some(c) = _iter.next() {
            offset += 1;
            
            if c == '#' {
                is_comment = true;
                break;
            } else if !['t','r','u','e','f','a','l','s'].contains(&c) {
                return ParserError::from(FormatError::UnallowedCharacter(c, InTypeBoolean), offset)
            } else {
                value.push(c);    
            }
        }

        if let Some(err) = check_comment_or_whitespaces(iter, is_comment) {
            return ParserError::extend(err, offset)
        }

        match value.parse::<bool>() {
            Ok(v) => Ok(v),
            Err(err) => ParserError::from(err, 0),
        }
    }
}