#![feature(let_else)]
extern crate core;

use crate::lexer::{execute, parse, Lexer};

mod lexer;

fn main() {
    println!("{:?}", Lexer::new().tokenize("5 + 1 + 'hello';"));
}
