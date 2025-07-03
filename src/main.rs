mod errors;
mod common;
mod types;
mod macros;
mod consts;
mod parser;

use std::{char, fs::File, io::BufRead, string};

pub use consts::*;
pub use common::*;

use crate::{errors::ParserError, parser::parse_value, types::{ParsedValue,NumberType}};

// use crate::types::Parser;

fn skip_key(iter: &mut std::str::Chars) -> Option<usize> {
    let mut pos = 0;
    while let Some(c) = iter.next() {
        if !c.is_whitespace() {
            return Some(pos);
        }
        pos += 1
    }

    None
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut reader = std::io::BufReader::new(f);

    
    loop {
        let mut buf = String::new();
        if let Ok(bytes) = reader.read_line(&mut buf) {
            if bytes == 0 {
                break;
            }
        }

        let dbg_cp = buf.clone();
        let mut chars: std::str::Chars<'_> = buf.chars();
        
        let mut offset = 0;

        let mut key = String::new();
        if loop {
            if let Some(c) = chars.next() {
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

        match parse_value(&mut chars) {
            Ok(wrapped_value) => match wrapped_value {
                ParsedValue::Boolean(value) => { println!("{} = {}", key.trim(), value); },
                ParsedValue::Number(num) => match num {
                    NumberType::Float(value) => { println!("{} = {:.1}", key.trim(), value); },
                    NumberType::Integer(value) => { println!("{} = {}", key.trim(), value); },
                },
                ParsedValue::String(value) => { println!("{} = {}", key.trim(), value); },
            },
            Err(err) => if let Err(_err) = ParserError::extend::<()>(err, offset) {
                _err.explain(&dbg_cp);
            }
        }
    }
}
