use phf::phf_map;
use std::ops::Add;

#[derive(Debug)]
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
#[derive(Debug)]
pub enum WhiteSpaceToken {
    Space,
    Tab,
    NewLine,
}

#[derive(Debug)]
pub enum PunctuatorToken {
    Semicolon, // ;
    Comma,     // ,
    Dot,       // .
    Colon,     // :
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum LiteralToken {
    Number(NumberToken),
    String(String),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ComparisonToken {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug)]
pub enum LogicalToken {
    And,
    Or,
    BitwiseOr,
    BitwiseAnd,
    Not,
    XOr,
    XAnd,
}

#[derive(Debug)]
pub struct IdentifierToken {
    pub(crate) name: String,
}

#[derive(Debug)]
pub enum ControlFlowToken {
    If,
    Else,
    Let,
    For,
    In,
    Has,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Token {
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
pub const KEYWORD_STRING_TO_TOKEN: phf::Map<&'static str, Token> = phf_map! {
    "if" => Token::ControlFlow(ControlFlowToken::If),
    "let" => Token::ControlFlow(ControlFlowToken::Let),
    "for" => Token::ControlFlow(ControlFlowToken::For),
    "in" => Token::ControlFlow(ControlFlowToken::In),
    "has" => Token::ControlFlow(ControlFlowToken::Has),
};
