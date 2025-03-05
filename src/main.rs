#![feature(is_some_and)]

use crate::lexer::Scanner;
mod lexer;
mod test;
mod token;

fn main() {
    let scanner = Scanner::new("let hello = 10 + 100 - 250;");
    for token in scanner {
        println!("{token:?}");
    }
}
