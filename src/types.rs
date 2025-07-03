mod common;

mod boolean;
mod string;
mod number;

pub use number::Number;
pub use boolean::Boolean;
pub use string::String;

pub enum NumberType {
    Integer(isize),
    Float(f64)
}

pub enum ParsedValue {
    Number(NumberType),
    Boolean(bool),
    String(std::string::String)
}

// parse should assume that iterator will read indefinetely, so line breaks should be handled accordingly
pub trait TypeParser<T> {
    fn parse(first: char, iter: &mut std::str::Chars) -> Result<T, crate::errors::ParserError>;
}
