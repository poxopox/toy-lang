use crate::token::{
    ArithmeticToken, AssignmentToken, ComparisonToken, DelimiterToken, IdentifierToken,
    LiteralToken, LogicalToken, NumberToken, Token, WhiteSpaceToken, KEYWORDS,
};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::ops::{Add, Deref};
use std::str::Chars;

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        // If we've reached the end of input, return None to signal end of iteration
        if self.start == self.end {
            return None;
        }

        // Buffer to accumulate characters for multi-character tokens
        let mut buffer = String::new();

        // Get the next character from input
        let next_input = self.input.as_bytes()[self.start] as char;

        // If the next character is a number, process a numeric token
        if next_input.is_numeric() {
            return Some(self.next_number(&mut buffer));
        }

        // If the next character is whitespace, consume it and return a Space token
        if next_input.is_whitespace() {
            self.inc();
            return Some(Token::WhiteSpace(WhiteSpaceToken::Space));
        }

        // If the next character is a quotation mark, process a string token
        if next_input == '"' || next_input == '\'' || next_input == '`' {
            return Some(self.next_string(&mut buffer, next_input));
        }

        // If the next character is alphabetic, process an identifier token
        if next_input.is_alphabetic() {
            return Some(self.next_identifier(&mut buffer));
        }

        // Process single-character operators or return unknown token
        let token = match next_input {
            '+' => Some(Token::Arithmetic(ArithmeticToken::Plus)),
            '-' => Some(Token::Arithmetic(ArithmeticToken::Minus)),
            '=' => Some(Token::Assignment(AssignmentToken::Assign)),
            '>' => Some(Token::Comparison(ComparisonToken::GreaterThan)),
            '<' => Some(Token::Comparison(ComparisonToken::LessThan)),
            '!' => Some(Token::Logical(LogicalToken::Not)),
            '&' => Some(Token::Logical(LogicalToken::BitwiseAnd)),
            '|' => Some(Token::Logical(LogicalToken::BitwiseOr)),
            '(' => Some(Token::Delimiter(DelimiterToken::OpenParenthesis)),
            ')' => Some(Token::Delimiter(DelimiterToken::CloseParenthesis)),
            '[' => Some(Token::Delimiter(DelimiterToken::OpenBracket)),
            ']' => Some(Token::Delimiter(DelimiterToken::CloseBracket)),
            '{' => Some(Token::Delimiter(DelimiterToken::OpenBrace)),
            '}' => Some(Token::Delimiter(DelimiterToken::CloseBrace)),
            _ => Some(Token::Unknown(next_input)),
        };

        // Consume the character and return the token
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
    fn next_number(&mut self, buffer: &mut String) -> Token {
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
            return Token::Literal(LiteralToken::Number(NumberToken::Float(
                buffer.parse::<f64>().unwrap(),
            )));
        } else {
            return Token::Literal(LiteralToken::Number(NumberToken::SignedInteger(
                buffer.parse::<i64>().unwrap(),
            )));
        }
    }
    fn next_string(&mut self, buffer: &mut String, quote_type: char) -> Token {
        self.inc();
        loop {
            if self.start == self.end {
                break;
            };
            let maybe_next_input = self.input.chars().nth(self.start);
            if let Some(next_char) = maybe_next_input {
                if next_char != quote_type {
                    buffer.push(next_char);
                    self.inc();
                } else {
                    self.inc();
                    break;
                }
            }
        }
        return Token::Literal(LiteralToken::String(buffer.to_string()));
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
                    if KEYWORDS.contains(&buffer.as_str()) {
                        // return Token::Keyword(KeywordToken::from(buffer.as_str()));
                    }
                    self.inc();
                } else {
                    break;
                }
            }
        }
        return Token::Identifier(IdentifierToken {
            name: buffer.to_string(),
        });
    }
}
