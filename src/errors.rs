use std::fmt::{Display, Debug};
use core::error::Error;


#[derive(Debug)]
pub enum UnallowedCharacterReason {
    // InLine,
    InComment,
    InTypeInteger,
    InTypeBoolean,
    InTypeFloat,
    InTypeString,
    InUnicodeSequence,
}

#[derive(Debug)]
pub enum FormatError {
    UnallowedCharacter(char, UnallowedCharacterReason),
    ExpectedCharacter(char),
    UnknownEscapeSequence,
}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FormatError::UnallowedCharacter(c, reason) => {
                let reason = match reason {
                    UnallowedCharacterReason::InTypeInteger => "in integer",
                    // UnallowedCharacterReason::InLine => "in line",
                    UnallowedCharacterReason::InComment => "in a comment",
                    UnallowedCharacterReason::InTypeFloat => "in a float",
                    UnallowedCharacterReason::InTypeBoolean => "in a boolean",
                    UnallowedCharacterReason::InTypeString => "in a string",
                    UnallowedCharacterReason::InUnicodeSequence => "in a unicode escape sequence",
                };
                write!(f, "unallowed character `{c}` {reason}")
            },
            FormatError::ExpectedCharacter(c) => write!(f, "expected character `{c}`"),
            FormatError::UnknownEscapeSequence => write!(f, "unknown escape sequence"),
        }
    }
}

impl Error for FormatError {}

#[derive(Debug)]
pub struct ParserError {
    source: Box<dyn Error>,
    offset: usize,
}

impl ParserError {
    pub fn from<T,E: Error + 'static>(source: E, offset: usize) -> Result<T,ParserError> {
        Err(ParserError {
            source: Box::new(source),
            offset: offset,
        })
    }

    pub fn extend<T>(source: Self, offset: usize) -> Result<T,Self> {
        Err(ParserError {
            offset: source.offset + offset,
            ..source
        })
    }

    pub fn panic(&self, line: &str) {
        print!("{}", line);
        let mut underline = vec!['-'; self.offset];
        underline.push('^');
        let underline = String::from_iter(underline.iter());
        println!("{}", underline);
        panic!("{}", self);
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse value: ")?;
        Display::fmt(&self.source, f)
    }
}

impl Error for ParserError {}