mod errors;
mod reader;
mod common;
mod types;
mod macros;
mod consts;
mod parser;

pub use consts::*;
pub use common::*;
pub use common::CharExt;

use crate::{errors::ParserError, parser::parse_value, reader::char_supplier::{Reader, Supplier}, types::{NumberType, ParsedValue}};

fn main() {
    let f = std::fs::File::open("input.toml").unwrap();
    let mut reader = Reader::new(f);
    let mut supplier = reader.iter();

    while !supplier.is_end() {
        let (mut offset, c) = match skip_whitespaces(&mut supplier) {
            Some(v) => v,
            None => continue,
        };

        if c.is_comment_start() {
            if let Some(err) = check_comment_or_whitespaces(&mut supplier, true) {
                err.explain(supplier.get_last_line());
            }
            continue;
        }

        let mut key = String::from(c);
        if loop {
            if let Some(c) = supplier.get() {
                offset += 1;
                if c == '=' {
                    break false;
                }
                key.push(c);
            } else {
                break true;
            }
        } {
            continue;
        }

        match parse_value(&mut supplier) {
            Ok(wrapped_value) => match wrapped_value {
                ParsedValue::Boolean(value) => { println!("{} = {}\n\t{}", key.trim(), value, supplier.get_last_line()); },
                ParsedValue::Number(num) => match num {
                    NumberType::Float(value) => { println!("{} = {:.1}\n\t{}", key.trim(), value, supplier.get_last_line()); },
                    NumberType::Integer(value) => { println!("{} = {}\n\t{}", key.trim(), value, supplier.get_last_line()); },
                },
                ParsedValue::String(value) => { println!("{} = {}\n\t{}", key.trim(), value, supplier.get_last_line()); },
            },
            Err(err) => if let Err(_err) = ParserError::extend::<()>(err, offset) {
                _err.explain(supplier.get_last_line());
            }
        }
    }
}
