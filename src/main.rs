mod errors;
mod reader;
mod common;
mod types;
mod consts;
mod parsers;

pub use consts::*;
pub use common::*;

use crate::{parsers::ValueParser, reader::char_supplier::{Reader, Supplier}, types::{NumberType, Value}};

fn main() {
    let f = std::fs::File::open("input.toml").unwrap();
    let mut reader = Reader::new(f);
    let mut supplier = reader.iter_with_debug();

    while !supplier.is_end() {
        let mut key = String::new();

        match skip_whitespaces(&mut supplier, true) {
            Some(c) => {
                if c.is_comment_start() {
                    if let Some(err) = check_comment_or_whitespaces(&mut supplier, true) {
                        err.explain_with_debug(&mut supplier);
                    }
                    continue;
                } else {
                    key.push(c);
                }
            },
            None => continue,
        };

        // reading key
        if loop {
            if let Some(c) = supplier.get() {
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

        match ValueParser::parse(&mut supplier) {
            Ok(wrapped_value) => match wrapped_value {
                Value::Boolean(value) => { println!("{} = {}", key.trim(), value); },
                Value::Number(num) => match num {
                    NumberType::Float(value) => { println!("{} = {:.1}", key.trim(), value); },
                    NumberType::Integer(value) => { println!("{} = {}", key.trim(), value); },
                },
                Value::String(value) => { println!("{} = {}", key.trim(), value); },
                Value::DateTime(value) => println!("{} = {}", key, value.to_string())
            },
            Err(err) => {
                err.explain_with_debug(&mut supplier);
            }
        }
    }
}
