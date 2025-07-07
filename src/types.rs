mod common;

mod boolean;
mod string;
mod number;
mod datetime;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
pub use number::Number;
pub use boolean::Boolean;
pub use string::String;
pub use datetime::DateTime;

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
pub enum DateTimeType {
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
}

impl ToString for DateTimeType {
    fn to_string(&self) -> std::string::String {
        match self {
            Self::Date(v) => v.to_string(),
            Self::Time(v) => v.to_string(),
            Self::DateTime(v) => v.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Number(NumberType),
    Boolean(bool),
    String(std::string::String),
    DateTime(DateTimeType)
}

impl ToString for Value {
    fn to_string(&self) -> std::string::String {
        match self {
            Self::String(v) => v.to_string(),
            Self::Boolean(v) => v.to_string(),
            Self::Number(v) => v.to_string(),
            Self::DateTime(v) => v.to_string()
        }
    }
}

// parse should assume that iterator will read indefinetely, so line breaks should be handled accordingly
pub trait TypeParser<T> {
    fn parse(first: char, input: &mut impl Supplier) -> Result<T, crate::errors::ParserError>;
}
