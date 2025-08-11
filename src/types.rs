pub use super::parsers::TypeParser;
mod common;

mod boolean;
mod string;
mod number;
mod datetime;

use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub use number::Number;
pub use boolean::Boolean;
pub use string::{String, StringType};
pub use datetime::DateTime;


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

impl Clone for DateTimeType {
    fn clone(&self) -> Self {
        match self {
            Self::Date(arg0) => Self::Date(arg0.clone()),
            Self::Time(arg0) => Self::Time(arg0.clone()),
            Self::DateTime(arg0) => Self::DateTime(arg0.clone()),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Number(NumberType),
    Boolean(bool),
    String(std::string::String),
    DateTime(DateTimeType),
    Nested(Box<HashMap<Key, Value>>)
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}", v),
            Self::Boolean(v) =>  write!(f, "{}", v.to_string()),
            Self::Number(v) =>  write!(f, "{}", v.to_string()),
            Self::DateTime(v) =>  write!(f, "{}", v.to_string()),
            Self::Nested(v) =>  write!(f, "{:?}", v),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Number(NumberType::Float(arg0)) => Self::Number(NumberType::Float(*arg0)),
            Self::Number(NumberType::Integer(arg0)) => Self::Number(NumberType::Integer(*arg0)),
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::DateTime(arg0) => Self::DateTime(arg0.clone()),
            Self::Nested(arg0) => Self::Nested(arg0.clone()),
        }
    }
}


#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Key {
    name: std::string::String
}

impl Key {
    pub fn new(name: std::string::String) -> Self {
        Key {
            name
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Clone for Key {
    fn clone(&self) -> Self {
        Self { name: self.name.clone() }
    }
}

#[derive(Debug)]
pub struct Entry {
    key: Key,
    value: Value,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={:?}", self.key, v)
    }
}

impl Entry {
    pub fn new(key: Key, value: Value) -> Self {
        Self {
            key,
            value
        }
    }

    pub fn get_key(&self) -> &Key {
        &self.key
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }
}

