#![feature(is_some_and)]
extern crate core;

use crate::lexer::Scanner;
mod lexer;
mod test;
mod token;

fn main() {
    let mut lexer = Scanner::new("let hello = 10 + 100 - 250;");
    let mut token = lexer.next().unwrap();
    print!("{token:?}")
}
