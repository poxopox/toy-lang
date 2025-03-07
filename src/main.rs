#![feature(is_some_and)]

use crate::lexer::Scanner;
use crate::token::TokenType;

mod lexer;
mod test;
mod token;

fn main() {
    let input = r#"
    let hello = 10 + 100 - 250;
    let x = hello + "Hello World!";
    let z = !!x === false;
    "#;
    let scanner = Scanner::new(input);
    for token in scanner {
        if !matches!(token.token_type, TokenType::WhiteSpace(_)) {
            println!("{token:?}");
        }
    }
}
