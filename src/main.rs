#![feature(let_else)]
#![feature(is_some_and)]
extern crate core;

use crate::lexer::Lexer;
mod lexer;
mod token;

fn main() {
    let lexer = Lexer::new("10+10.5");
    for token in lexer {
        println!("{token:?}")
    }
}
