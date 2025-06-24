mod errors;
mod types;
mod macros;
mod consts;

use std::{fs::File, io::{BufRead}};

pub use consts::*;

use crate::types::Parser;

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut reader = std::io::BufReader::new(f);

    let mut last_line = String::new();
    let _ = reader.read_line(&mut last_line);
    {
        let mut reader = std::io::Cursor::new(last_line.clone());
        match types::Integer::parse(&mut reader) {
            Ok(v) => { println!("integer = {}", v); },
            Err(err) => { err.panic(last_line.as_str()); }
        };
    }

    let mut last_line = String::new();
    let _ = reader.read_line(&mut last_line);
    {
        let mut reader = std::io::Cursor::new(last_line.clone());
        match types::Float::parse(&mut reader) {
            Ok(v) => { println!("float = {}", v); },
            Err(err) => { err.panic(last_line.as_str()); },
        };
    }
    
    let mut last_line = String::new();
    let _ = reader.read_line(&mut last_line);
    {
        let mut reader = std::io::Cursor::new(last_line.clone());
        match types::Boolean::parse(&mut reader) {
            Ok(v) => { println!("boolean = {}", v); },
            Err(err) => { err.panic(last_line.as_str()); },
        };
    }
    
    let mut last_line = String::new();
    let _ = reader.read_line(&mut last_line);
    {
        let mut reader = std::io::Cursor::new(last_line.clone());
        match types::BasicString::parse(&mut reader) {
            Ok(v) => { println!("string = {}", v); },
            Err(err) => { err.panic(last_line.as_str()); },
        };
    }
}
