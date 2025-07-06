mod common;

mod boolean;
mod string;
mod number;

pub use number::Number;
pub use boolean::Boolean;
pub use string::String;

use crate::reader::char_supplier::Supplier;

#[derive(Debug)]
pub enum NumberType {
    Integer(isize),
    Float(f64)
}

impl ToString for NumberType {
    fn to_string(&self) -> std::string::String {
        match self {
            Self::Integer(v) => v.to_string(),
            Self::Float(v) => v.to_string()
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Number(NumberType),
    Boolean(bool),
    String(std::string::String)
}

impl ToString for Value {
    fn to_string(&self) -> std::string::String {
        match self {
            Self::String(v) => v.to_string(),
            Self::Boolean(v) => v.to_string(),
            Self::Number(v) => v.to_string()
        }
    }
}

// parse should assume that iterator will read indefinetely, so line breaks should be handled accordingly
pub trait TypeParser<T> {
    fn parse(first: char, iter: &mut impl Supplier) -> Result<T, crate::errors::ParserError>;
}
