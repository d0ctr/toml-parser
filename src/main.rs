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

        match parse_value(&mut supplier) {
            Ok(wrapped_value) => match wrapped_value {
                ParsedValue::Boolean(value) => { println!("{} = {}", key.trim(), value); },
                ParsedValue::Number(num) => match num {
                    NumberType::Float(value) => { println!("{} = {:.1}", key.trim(), value); },
                    NumberType::Integer(value) => { println!("{} = {}", key.trim(), value); },
                },
                ParsedValue::String(value) => { println!("{} = {}", key.trim(), value); },
            },
            Err(err) => {
                err.explain_with_debug(&mut supplier);
            }
        }
    }
}
