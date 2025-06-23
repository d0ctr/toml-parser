use super::common::check_comment_or_whitespaces;
use crate::errors::{ParserError, UnallowedCharacter, UnallowedCharacterReason::InTypeInteger};

pub struct Integer;

impl super::Parser<isize> for Integer {
    fn parse<T: std::io::Read> (reader: &mut T) -> Result<isize, ParserError> {
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
        let mut flag: u8 = 0b00;

        for (i, c) in line.chars().enumerate() {
            if c == ' ' || c == '#' {
                // if we hit a space or a comment -> break
                break;
            }

            if ('0'..='9').contains(&c) {
                // first digit sets validity
                flag = 0b01;
            } else {
                if c == '+' || c == '-' {
                    // if the character is not a digit or a sign -> fail
                    flag += 0b01;
                } else {
                    return ParserError::from(UnallowedCharacter::new(c, InTypeInteger), offset + i);
                }
            }
            if flag > 0b01 {
                return ParserError::from(UnallowedCharacter::new(c, InTypeInteger), offset + i);
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

        match value.parse::<isize>() {
            Ok(v) => Ok(v),
            Err(err) => ParserError::from(err, offset),
        }
    }
}
