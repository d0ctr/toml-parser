use crate::{errors::{ParserError, FormatError, UnallowedCharacterReason::InTypeFloat}, types::common::{check_comment_or_whitespaces, Trimmer}};

pub struct Float;

impl super::Parser<f64> for Float {
    fn parse<T: std::io::Read>(reader: &mut T) -> Result<f64, ParserError> {
        let line = super::common::read_line(reader);
        let line = match String::from_utf8(line) {
            Ok(v) => v,
            Err(err) => return ParserError::from(err, 0),
        };

        let (line, mut offset) = line.trim_start_with_difference();

        // crate new buffer, here the actual value will be stored
        let mut value = String::new();
        let mut chars = line.chars();
        let mut is_comment = false;
        
        let mut signed = false;
        let mut dotted = false;
        while let Some(c) = chars.next() {
            offset += 1;

            if c == ' ' {
                // if we hit a space or a comment -> break
                break;
            } else if c == '#' {
                is_comment = true;
                break;
            }
    
            if ('0'..='9').contains(&c) {
                // first digit also sets the sign
                signed = true
            } else if (c == '+' || c == '-') && !signed {
                signed = true;
            } else if c == '.' && !dotted {
                dotted = true;
            } else {
                return ParserError::from(FormatError::UnallowedCharacter(c, InTypeFloat), offset)
            }
    
            value.push(c);
        }

        // check the remainder of the line can be a comment or a whitespaces
        let remainder = String::from_iter(chars);
        if let Some(err) = check_comment_or_whitespaces(&remainder, is_comment) {
            return ParserError::extend(err, offset)
        }

        match value.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(err) => ParserError::from(err, offset),
        }
    }
}