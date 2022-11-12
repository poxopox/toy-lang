use crate::token::{
    DelimiterToken, IdentifierToken, LiteralToken, NumberToken, OperatorToken, Token,
};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::ops::{Add, Deref};
use std::str::Chars;

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let mut buffer = String::new();
        let next_input = self.input.as_bytes()[self.start] as char;
        if next_input.is_numeric() {
            let mut is_floating = false;
            loop {
                if self.start == self.end {
                    break;
                };
                let maybe_next_input = self.input.chars().nth(self.start);
                if let Some(next_char) = maybe_next_input {
                    if next_char.is_numeric() {
                        buffer.push(next_char);
                        self.inc();
                    } else if next_char == '.' || next_char == ',' {
                        if is_floating {
                            panic!("cant have more than one decimal");
                        }
                        is_floating = true;
                        buffer.push(next_char);
                        self.inc();
                    } else {
                        break;
                    }
                }
            }
            if is_floating {
                return Some(Token::Literal(LiteralToken::Number(NumberToken::Float(
                    buffer.parse::<f64>().unwrap(),
                ))));
            } else {
                return Some(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                    buffer.parse::<i64>().unwrap(),
                ))));
            }
        }
        if next_input.is_whitespace() {
            self.inc();
            return Some(Token::Delimiter(DelimiterToken::Space));
        }
        if next_input == '"' || next_input == '\'' || next_input == '`' {
            self.inc();
            loop {
                if self.start == self.end {
                    break;
                };
                let maybe_next_input = self.input.chars().nth(self.start);
                if let Some(next_char) = maybe_next_input {
                    if next_char != next_input {
                        buffer.push(next_char);
                        self.inc();
                    } else {
                        self.inc();
                        break;
                    }
                }
            }
            return Some(Token::Literal(LiteralToken::String(buffer)));
        }
        if next_input.is_alphabetic() {
            return Some(self.next_identifier(&mut buffer));
        }
        let token = match next_input {
            '+' => Some(Token::Operator(OperatorToken::Plus)),
            '-' => Some(Token::Operator(OperatorToken::Minus)),
            _ => Some(Token::Unknown(next_input)),
        };
        self.inc();
        return token;
    }
}

#[derive(Debug, Default)]
pub struct Lexer {
    input: String,
    start: usize,
    end: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let end = input.len();
        Self {
            input: input.to_string(),
            start: 0,
            end,
        }
    }
    fn inc(&mut self) {
        self.start = self.start + 1;
    }
    fn next_identifier(&mut self, buffer: &mut String) -> Token {
        loop {
            if self.start == self.end {
                break;
            };
            let maybe_next_input = self.input.chars().nth(self.start);
            if let Some(next_char) = maybe_next_input {
                if matches!(next_char, 'a'..='z' | '0'..='9' | '_' | '-') {
                    buffer.push(next_char);
                    self.inc();
                } else {
                    break;
                }
            }
        }
        return Token::Identifier(IdentifierToken { name: buffer.to_string() });
    }
}
