mod common;

mod integer;
mod float;
mod boolean;
mod string;

pub use integer::Integer;
pub use float::Float;
pub use boolean::Boolean;
pub use string::{BasicString};


pub trait Parser<T> {
    fn parse<R: std::io::Read>(reader: &mut R) -> Result<T, crate::errors::ParserError>;
}
