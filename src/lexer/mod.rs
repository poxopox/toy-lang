use std::borrow::Borrow;
use std::convert::TryFrom;
use std::ops::{Add, Deref};
use std::str::Chars;

#[derive(Debug)]
pub enum LexingError {
    ConversionError,
}

#[derive(Default, Debug)]
pub struct Lexer<'a> {
    token_stream: TokenStream<'a>,
    seq_token: Option<Token<'a>>,
    start_seq: Option<usize>,
    stop_seq: Option<usize>,
    exhausted: bool,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns number of unknown
    fn optimize(&mut self) {
        println!()
    }
    pub fn tokenize(&mut self, input: &'a str) -> TokenStream<'a> {
        let mut curr_char_idx = 0;

        input
            .split("")
            .skip(1)
            .fold(vec![], |mut token_stream, raw| {
                token_stream.push(Token::try_from(raw).unwrap());
                token_stream
            })
    }
}

#[derive(Debug)]
enum NumberToken {
    Integer(i64),
    Float(f64),
}

impl<'a> From<&'a str> for NumberToken {
    fn from(value: &'a str) -> Self {
        let contains_delimiter = value.contains(".") || value.contains(",");
        if !contains_delimiter {
            return NumberToken::Integer(value.parse::<i64>().unwrap());
        }
        NumberToken::Float(value.parse::<f64>().unwrap())
    }
}

impl TryFrom<NumberToken> for i64 {
    type Error = LexingError;
    fn try_from(value: NumberToken) -> Result<Self, Self::Error> {
        if let NumberToken::Integer(res) = value {
            Ok(res)
        } else {
            Err(LexingError::ConversionError)
        }
    }
}

impl Add for NumberToken {
    type Output = f64;
    fn add(self, rhs: Self) -> Self::Output {
        let left_num = match self {
            Self::Integer(inner) => inner as f64,
            Self::Float(inner) => inner,
        };
        let right_num = match rhs {
            Self::Integer(inner) => inner as f64,
            Self::Float(inner) => inner,
        };
        left_num + right_num
    }
}

#[derive(Debug)]
pub enum DelimiterToken {
    Space,
    Period,
    Comma,
    SingleQuote,
    DoubleQuote,
    BackTick,
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Colon,
    SemiColon,
    EOF,
    NewLine,
}

#[derive(Debug)]
pub enum LiteralToken {
    Number(NumberToken),
    String(String),
}

#[derive(Debug)]
pub enum OperatorToken {
    Plus,
    Minus,
    Multiply,
    Divide,
    Or,
    And,
    Not,
    Assign,
}

#[derive(Debug)]
pub enum IdentifierToken {}

#[derive(Debug)]
pub enum KeywordToken {}

#[derive(Debug)]
pub enum Token<'a> {
    Operator(OperatorToken),
    Delimiter(DelimiterToken),
    Identifier(IdentifierToken),
    Literal(LiteralToken),
    Keywords(KeywordToken),
    Unknown(Chars<'a>),
}

impl<'a> TryFrom<&'a str> for Token<'a> {
    type Error = LexingError;
    fn try_from(value: &'a str) -> Result<Token<'a>, Self::Error> {
        match value {
            "0" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                0,
            )))),
            "1" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                1,
            )))),
            "2" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                2,
            )))),
            "3" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                3,
            )))),
            "4" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                4,
            )))),
            "5" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                5,
            )))),
            "6" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                6,
            )))),
            "7" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                7,
            )))),
            "8" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                8,
            )))),
            "9" => Ok(Token::Literal(LiteralToken::Number(NumberToken::Integer(
                9,
            )))),
            "+" => Ok(Token::Operator(OperatorToken::Plus)),
            "-" => Ok(Token::Operator(OperatorToken::Minus)),
            "/" => Ok(Token::Operator(OperatorToken::Divide)),
            "*" => Ok(Token::Operator(OperatorToken::Multiply)),
            " " => Ok(Token::Delimiter(DelimiterToken::Space)),
            "'" => Ok(Token::Delimiter(DelimiterToken::SingleQuote)),
            "\"" => Ok(Token::Delimiter(DelimiterToken::DoubleQuote)),
            "`" => Ok(Token::Delimiter(DelimiterToken::BackTick)),
            "{" => Ok(Token::Delimiter(DelimiterToken::OpenBrace)),
            "}" => Ok(Token::Delimiter(DelimiterToken::CloseBrace)),
            "[" => Ok(Token::Delimiter(DelimiterToken::OpenBracket)),
            "]" => Ok(Token::Delimiter(DelimiterToken::CloseBracket)),

            _ => Ok(Token::Unknown(value.chars())),
        }
    }
}

pub type TokenStream<'a> = Vec<Token<'a>>;

pub fn parse(token_stream: TokenStream) -> Vec<TokenStream> {
    let mut tree: Vec<Vec<Token>> = vec![];
    let mut current_statement = vec![];
    for token in token_stream {
        match token {
            Token::Operator(o) => current_statement.push(Token::Operator(o)),
            Token::Identifier(_) => {}
            Token::Delimiter(delimiter) => match delimiter {
                DelimiterToken::SemiColon => {
                    tree.push(current_statement);
                    current_statement = vec![];
                }
                _ => {}
            },
            Token::Literal(l) => current_statement.push(Token::Literal(l)),
            Token::Unknown(_) => {}
            _ => {}
        }
    }
    tree
}

pub fn execute(input: Vec<TokenStream>) -> String {
    let mut result: String = String::new();
    for mut statement in input.into_iter() {
        let mut operator_idxs: Vec<usize> = vec![];
        let mut i = 0;
        // 1. check for operators
        // 2. find position of operators
        // 3. loop through position and

        for operator_index in operator_idxs {
            let op = statement.get(operator_index).unwrap();
            let left = statement.get(operator_index - 1).unwrap();
            let right = statement.get(operator_index + 1).unwrap();

            let mut left_inner = 0f64;
            let mut right_inner = 0f64;
            // if let Token::Literal(LiteralToken::Number(inn)) = left {
            //     left_inner = *inn;
            // }
            // if let Token::Literal(LiteralToken::Number(inn)) = right {
            //     right_inner = *inn;
            // }

            if let Token::Operator(op_token) = op {
                match op_token {
                    OperatorToken::Plus => result = format!("{:?}", left_inner + right_inner),
                    OperatorToken::Minus => {}
                    OperatorToken::Multiply => {}
                    OperatorToken::Divide => {}
                    OperatorToken::Or => {}
                    OperatorToken::And => {}
                    OperatorToken::Not => {}
                    OperatorToken::Assign => {}
                }
            }
        }
    }
    result
}
