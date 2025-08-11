mod errors;
mod reader;
mod common;
mod types;
mod consts;
mod parsers;

pub use consts::*;
pub use common::*;

use crate::{parsers::{KeyParser, ValueParser}, reader::char_supplier::{Reader, Supplier}, types::{NumberType, Value, Entry}};
use std::collections::hash_map::HashMap;

fn main() {
    let f = std::fs::File::open("input.toml").unwrap();
    let mut reader = Reader::new(f);
    let mut supplier = reader.iter_with_debug();

    // let mut map = std::vec::Vec::new();
    // while !supplier.is_end() {
    //     let mut keys = match KeyParser::parse_path(&mut supplier) {
    //         Ok(_keys) => _keys,
    //         Err(err) => { 
    //             err.explain_with_debug(&mut supplier);
    //             continue;
    //         },
    //     };
    //
    //     
    //     match ValueParser::parse(&mut supplier) {
    //         Ok(wrapped_value) => match wrapped_value {
    //             Value::Boolean(value) => { println!("{:?} = {}", keys, value); },
    //             Value::Number(num) => match num {
    //                             NumberType::Float(value) => { println!("{:?} = {:.1}", keys, value); },
    //                             NumberType::Integer(value) => { println!("{:?} = {}", keys, value); },
    //                         },
    //             Value::String(value) => { println!("{:?} = {}", keys, value); },
    //             Value::DateTime(value) => println!("{:?} = {}", keys, value.to_string()),
    //             Value::Nested(hash_map) => todo!(),
    //         },
    //         Err(err) => {
    //             err.explain_with_debug(&mut supplier);
    //         }
    //     }
    // }
}
