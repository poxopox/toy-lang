use crate::token::*;
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::ops::{Add, Deref};
use std::str::Chars;

impl Iterator for Scanner {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        // If we've reached the end of input, return None to signal end of iteration
        if self.start_idx == self.current_idx {
            return None;
        }

        // Buffer to accumulate characters for multi-character tokens
        let mut buffer = String::new();

        // Get the next character from input
        let next_input = self.input.as_bytes()[self.start_idx] as char;
        return Some(TokenType::Unknown(' '));
    }
}

#[derive(Debug, Default)]
pub struct Scanner {
    input: String,
    start_idx: usize,
    current_idx: usize,
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            start_idx: 0,
            current_idx: 0,
        }
    }
    fn inc(&mut self) {
        self.current_idx = self.current_idx + 1;
    }

    fn eof_token(&mut self) -> Token {
        self.token_at_current_position(TokenType::Delimiter(DelimiterToken::EOF))
    }

    fn unknown_token(&mut self) -> Token {
        self.token_at_current_position(TokenType::Unknown(self.input.chars().nth(0).unwrap()))
    }

    fn token_at_current_position(&mut self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            TokenSpan {
                start: self.start_idx,
                end: self.current_idx,
                line: 0,
                column: 0,
            },
        )
    }

    fn word_token(&mut self) -> Token {
        let mut word = String::new();
        while self.current_idx < self.input.len() {
            let next_char = self.input.chars().nth(self.current_idx).unwrap();
            if next_char.is_alphanumeric() {
                word.push(next_char);
                self.inc();
            } else {
                break;
            }
        }
        self.token_at_current_position(TokenType::Identifier(IdentifierToken { value: word }))
    }

    pub fn next_token(&mut self) -> Token {
        //First, make sure it's not the end of input
        let token = if self.current_idx >= self.input.len() {
            self.eof_token()
        } else {
            let next_char = self.input.chars().nth(self.current_idx).unwrap();
            if next_char.is_alphanumeric() {
                // If the next character is alphanumeric, it's a word token
                self.word_token()
            } else if next_char.is_whitespace() {
                // If the next character is whitespace, capture as whitespace token
                // first, get the whitespace type
                let token_type = match next_char {
                    ' ' => TokenType::WhiteSpace(WhiteSpaceToken::Space),
                    '\t' => TokenType::WhiteSpace(WhiteSpaceToken::Tab),
                    '\n' => TokenType::WhiteSpace(WhiteSpaceToken::NewLine),
                    _ => TokenType::Unknown(next_char),
                };
                self.token_at_current_position(token_type)
            } else {
                // If no other cases match, return an unknown token
                self.unknown_token()
            }
        };
        self.start_idx = self.current_idx;
        self.inc();
        token
    }
}
