#![feature(let_chains)]

use crate::lexer::Scanner;
use crate::token::TokenType;

mod lexer;
mod test;
mod token;

fn main() {
    let input = r#"
let x = "Hello World!";
'other string';
`third string`
"string with 'inside string' and \"inside string\""
"#;
    let scanner = Scanner::new(input);
    for token in scanner {
        if !matches!(token.token_type, TokenType::WhiteSpace(_)) {
        println!("{token:?}");
        }
    }
}
