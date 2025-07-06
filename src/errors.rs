use std::fmt::{Display, Debug};
use core::error::Error;

use crate::reader::char_supplier::DebuggingIterator;


#[derive(Debug)]
pub enum UnallowedCharacterReason {
    // InLine,
    InComment,
    InTypeNumber,
    InTypeBoolean,
    InTypeBasicString,
    InTypeMultilineBasicString,
    InTypeMultilineLiteralString,
    InTypeLiteralString,
    InUnicodeSequence,
}

#[derive(Debug)]
pub enum FormatError {
    UnallowedCharacter(char, UnallowedCharacterReason),
    ExpectedCharacter(char),
    ExpectedSequence(std::string::String),
    UnknownEscapeSequence,
    EmptyValue,
    UnexpectedEnd
}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FormatError::UnallowedCharacter(c, reason) => {
                let reason = match reason {
                    UnallowedCharacterReason::InTypeNumber => "in a number",
                    // UnallowedCharacterReason::InLine => "in line",
                    UnallowedCharacterReason::InComment => "in a comment",
                    UnallowedCharacterReason::InTypeBoolean => "in a boolean",
                    UnallowedCharacterReason::InTypeBasicString => "in a basic string",
                    UnallowedCharacterReason::InTypeMultilineBasicString => "in a multi-line basic string",
                    UnallowedCharacterReason::InTypeLiteralString => "in a literal string",
                    UnallowedCharacterReason::InTypeMultilineLiteralString => "in a multi-line literal string",
                    UnallowedCharacterReason::InUnicodeSequence => "in a unicode escape sequence",
                };
                write!(f, "unallowed character `{c}` {reason}")
            },
            FormatError::ExpectedCharacter(c) => write!(f, "expected character `{c}`"),
            FormatError::UnknownEscapeSequence => write!(f, "unknown escape sequence"),
            FormatError::EmptyValue => write!(f, "empty value"),
            FormatError::UnexpectedEnd => write!(f, "unexpected end of file"),
            FormatError::ExpectedSequence(seq) => write!(f, "expected `{seq}`"),
        }
    }
}

impl Error for FormatError {}

#[derive(Debug)]
pub struct ParserError {
    source: Box<dyn Error>,
    // offset: usize,
}

impl ParserError {
    pub fn from<T,E: Error + 'static>(source: E) -> Result<T,ParserError> {
        Err(ParserError {
            source: Box::new(source),
        })
    }

    pub fn extend<T>(source: Self) -> Result<T,Self> {
        Err(ParserError {
            ..source
        })
    }

    pub fn explain_with_debug<R: std::io::Read>(&self, iter: &mut DebuggingIterator<'_,R>) {
        let needle = iter.get_needle();
        let line = iter.get_last_line();

        println!("{}", line.trim_end());

        let mut underline = vec![' '; needle - 1];
        underline.push('^');
        let underline = String::from_iter(underline.iter());
        println!("{}", underline);

        println!("{}", self)
    }

    pub fn explain(&self) {
        println!("{}", self)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse value: ")?;
        Display::fmt(&self.source, f)
    }
}

impl Error for ParserError {}