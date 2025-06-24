use crate::errors::{FormatError, ParserError, UnallowedCharacterReason::{InTypeString,InUnicodeSequence}};
use crate::UNICODE_SEQUENCE_CHARS;
use super::Parser;
use super::common::{Trimmer,find_replacement_char,check_comment_or_whitespaces};

pub struct BasicString;

impl Parser<String> for BasicString {
    fn parse<R: std::io::Read>(reader: &mut R) -> Result<String, crate::errors::ParserError> {
        let buf = super::common::read_line(reader);
        
        let line = match String::from_utf8(buf) {
            Ok(v) => v,
            Err(err) => return ParserError::from(err, 0),
        };

        let (line, mut offset) = line.trim_start_with_difference();

        let mut chars = line.chars();
        match chars.next() {
            Some(c) => if c != '"' { return ParserError::from(FormatError::UnallowedCharacter(c, InTypeString), offset) },
            None => return ParserError::from(FormatError::ExpectedCharacter('"'), offset)
        }

        let mut value = String::new();
        let mut escape_seq = String::new();
        let mut unicode_seq = String::new();
        let mut quoted = false;

        while let Some(c) = chars.next() {
            offset += 1;
            // if escape buf is empty and this is a quote sign => the value has been fully read
            if c == '"' && escape_seq.is_empty() {
                quoted = true;
                break; 
            }

            if c == '\\' && escape_seq.is_empty() { 
                // if this is a start of an escape sequence -> push it
                escape_seq.push(c);
                continue;
            } else if !escape_seq.is_empty() {
                // if an escape sequence has started -> push the character
                escape_seq.push(c);

                // this is a unicode sequence, switch buffer
                if escape_seq == "\\u" || escape_seq == "\\U" {
                    unicode_seq = escape_seq.clone();
                    escape_seq.clear();
                    continue;
                }
                
                if let Some(replacement) = find_replacement_char(&escape_seq) {
                    value.push(replacement);
                    offset += 1;
                    escape_seq.clear();
                } else {
                    return ParserError::from(FormatError::UnknownEscapeSequence, offset);
                }
            } else if !unicode_seq.is_empty() {
                if UNICODE_SEQUENCE_CHARS.contains(&c) {
                    unicode_seq.push(c);
                } else {
                    return ParserError::from(FormatError::UnallowedCharacter(c, InUnicodeSequence), offset);
                }

                let char_count = unicode_seq.chars().count();
                let codepoint = if unicode_seq.starts_with("\\u") {
                    if char_count < 6 {
                        continue;
                    }
                    if char_count > 6 {
                        return ParserError::from(FormatError::UnknownEscapeSequence, offset)
                    }

                    unicode_seq.trim_start_matches("\\u")
                } else if unicode_seq.starts_with("\\U") {
                    if char_count < 10 {
                        continue;
                    }
                    if char_count > 10 {
                        return ParserError::from(FormatError::UnknownEscapeSequence, offset)
                    }

                    unicode_seq.trim_start_matches("\\u")
                } else {
                    return ParserError::from(FormatError::UnknownEscapeSequence, offset)
                };

                let codepoint = match u32::from_str_radix(&codepoint, 16) {
                    Ok(codepoint) => codepoint,
                    Err(_) => return ParserError::from(FormatError::UnknownEscapeSequence, offset),
                };

                if let Some(replacement) = char::from_u32(codepoint) {
                    value.push(replacement);
                    offset += 1;
                    unicode_seq.clear();
                } else {
                    return ParserError::from(FormatError::UnknownEscapeSequence, offset);
                }
            } else {
                value.push(c);
            }
        }

        if !unicode_seq.is_empty() {
            return ParserError::from(FormatError::UnknownEscapeSequence, offset);
        } 

        if !quoted {
            return ParserError::from(FormatError::ExpectedCharacter('"'), offset);
        }


        // check the remainder of the line can be a comment or a whitespaces
        let remainder = String::from_iter(chars);
        if let Some(err) = check_comment_or_whitespaces(&remainder, false) {
            return ParserError::extend(err, offset)
        }

        Ok(value)
    }
}