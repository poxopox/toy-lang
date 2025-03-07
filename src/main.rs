#![feature(is_some_and)]

use crate::lexer::Scanner;
use crate::token::TokenType;

mod lexer;
mod test;
mod token;

fn main() {
    let input = r#"Hello"#;
    let scanner = Scanner::new(input);
    for token in scanner {
        if !matches!(token.token_type, TokenType::WhiteSpace(_)) {
            println!("{token:?}");
        }
    }
}
