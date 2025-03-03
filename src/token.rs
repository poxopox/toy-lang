use std::cmp::PartialEq;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    // Not used yet
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumberToken {
    SignedInteger(i64),
    UnsignedInteger(u64),
    Float(f64),
}

impl<'a> From<&'a str> for NumberToken {
    fn from(value: &'a str) -> Self {
        let contains_delimiter = value.contains(".") || value.contains(",");
        if !contains_delimiter {
            return NumberToken::SignedInteger(value.parse::<i64>().unwrap());
        }
        NumberToken::Float(value.parse::<f64>().unwrap())
    }
}
impl Add for NumberToken {
    type Output = NumberToken;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // If both are signed integers, keep as signed integer if possible
            (Self::SignedInteger(left), Self::SignedInteger(right)) => {
                // Check for overflow
                match left.checked_add(right) {
                    Some(result) => Self::SignedInteger(result),
                    None => Self::Float(left as f64 + right as f64),
                }
            }
            // If both are unsigned integers, keep as unsigned integer if possible
            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => {
                // Check for overflow
                match left.checked_add(right) {
                    Some(result) => Self::UnsignedInteger(result),
                    None => Self::Float(left as f64 + right as f64),
                }
            }
            // If one is float, result is float
            (Self::Float(left), Self::Float(right)) => Self::Float(left + right),
            (Self::Float(left), Self::SignedInteger(right)) => Self::Float(left + right as f64),
            (Self::Float(left), Self::UnsignedInteger(right)) => Self::Float(left + right as f64),
            (Self::SignedInteger(left), Self::Float(right)) => Self::Float(left as f64 + right),
            (Self::UnsignedInteger(left), Self::Float(right)) => Self::Float(left as f64 + right),
            // Mixed integer types - try to use signed if possible, otherwise float
            (Self::SignedInteger(left), Self::UnsignedInteger(right)) => {
                if right <= i64::MAX as u64 {
                    match left.checked_add(right as i64) {
                        Some(result) => Self::SignedInteger(result),
                        None => Self::Float(left as f64 + right as f64),
                    }
                } else {
                    Self::Float(left as f64 + right as f64)
                }
            }
            (Self::UnsignedInteger(left), Self::SignedInteger(right)) => {
                if left <= i64::MAX as u64 {
                    match (left as i64).checked_add(right) {
                        Some(result) => Self::SignedInteger(result),
                        None => Self::Float(left as f64 + right as f64),
                    }
                } else {
                    Self::Float(left as f64 + right as f64)
                }
            }
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum WhiteSpaceToken {
    Space,
    Tab,
    NewLine,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PunctuatorToken {
    Semicolon,
    Comma,
    Dot,
    Colon,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DelimiterToken {
    SingleQuote,
    DoubleQuote,
    BackTick,
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralToken {
    Number(NumberToken),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArithmeticToken {
    Plus,
    Minus,
    Multiply,
    Divide,
    BitwiseAnd,
    BitwiseOr,
    Or,
    And,
    Not,
    Modulo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonToken {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalToken {
    And,
    Or,
    BitwiseOr,
    BitwiseAnd,
    Not,
    XOr,
    XAnd,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ControlFlowToken {
    If,
    Else,
    Let,
    For,
    In,
    Has,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentToken {
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    OrAssign,
    AndAssign,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierToken {
    pub value: String,
}

impl IdentifierToken {
    pub fn new(value: String) -> Self {
        IdentifierToken { value }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Assignment(AssignmentToken),
    Arithmetic(ArithmeticToken),
    Punctuation(PunctuatorToken),
    Comparison(ComparisonToken),
    Logical(LogicalToken),
    Delimiter(DelimiterToken),
    ControlFlow(ControlFlowToken),
    WhiteSpace(WhiteSpaceToken),
    Identifier(IdentifierToken),
    Literal(LiteralToken),
    Unknown(char),
}

pub const KEYWORDS: [&str; 5] = ["if", "let", "for", "in", "has"];

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub token_span: TokenSpan,
}

impl Token {
    pub fn new(token_type: TokenType, token_span: TokenSpan) -> Self {
        Token {
            token_type,
            token_span,
        }
    }
}
