#![feature(is_some_and)]
extern crate core;

use crate::lexer::Lexer;
mod lexer;
mod token;

fn main() {
    let lexer = Lexer::new("let x = (10 + 10.5 === 20.5)");
    for token in lexer {
        println!("{token:?}")
    }
}
