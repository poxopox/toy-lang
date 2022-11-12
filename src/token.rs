use std::ops::Add;

#[derive(Debug)]
pub enum NumberToken {
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
pub struct IdentifierToken {
    pub(crate) name: String,
}

#[derive(Debug)]
pub enum KeywordToken {}

#[derive(Debug)]
pub enum Token {
    Operator(OperatorToken),
    Delimiter(DelimiterToken),
    Identifier(IdentifierToken),
    Literal(LiteralToken),
    Keywords(KeywordToken),
    Unknown(char),
}

const KEYWORDS: [&str; 4] = ["let", "for", "in", "has"];
