use crate::{errors::{ParserError, UnallowedCharacter, UnallowedCharacterReason::InTypeBoolean}, types::common::{check_comment_or_whitespaces}};

pub struct Boolean;

impl super::Parser<bool> for Boolean {
    fn parse<T: std::io::Read>(reader: &mut T) -> Result<bool, ParserError> {
        let line = super::common::read_line(reader);
        let line = match String::from_utf8(line) {
            Ok(v) => v,
            Err(err) => return ParserError::from(err, 0),
        };

        // calculating offset for bullseye errors
        let mut offset = line.chars().count();

        // remove only leading whitespaces, we can leave the rest as is
        let line = line.trim_start().to_string();

        // offset is reduced to the difference before and after trim
        offset -= line.chars().count();

        // crate new buffer, here the actual value will be stored
        let mut buf = vec![];

        for (i, c) in line.chars().enumerate() {
            if c == ' ' || c == '#' {
                // if we hit a space or a comment -> break
                break;
            }
            if !['t','r','u','e','f','a','l','s'].contains(&c) {
                return ParserError::from(UnallowedCharacter::new(c, InTypeBoolean), offset + buf.len())
            }

            buf.push(c);
        };

        // this will be parsed
        let value = dbg!(String::from_iter(buf.iter()));

        // check the remainder of the line can be a comment or a whitespaces
        let (_, remainder) = line.split_at(value.len());
        if let Some(err) = check_comment_or_whitespaces(remainder) {
            return ParserError::extend(err, offset)
        }

        match value.parse::<bool>() {
            Ok(v) => Ok(v),
            Err(err) => ParserError::from(err, offset),
        }
    }
}