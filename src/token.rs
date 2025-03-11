use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumberToken {
    SignedInteger(i64),
    Float(f64),
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
                    _ => Self::Float(left as f64 + right as f64),
                }
            }
            // If one is float, result is float
            (Self::Float(left), Self::Float(right)) => Self::Float(left + right),
            (Self::Float(left), Self::SignedInteger(right)) => Self::Float(left + right as f64),
            (Self::SignedInteger(left), Self::Float(right)) => Self::Float(left as f64 + right),
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
    Boolean(bool),
    Null,
    Undefined,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArithmeticToken {
    Add,
    Subtract,
    Multiply,
    Divide,
    BitwiseAnd,
    BitwiseOr,
    Or,
    And,
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
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalToken {
    And,
    Or,
    Not,
    XOr,
    XAnd,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ControlFlowToken {
    If,
    Else,
    For,
    In,
    Has,
    Return,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierToken {
    pub value: String,
}

impl IdentifierToken {
    pub fn new(value: String) -> Self {
        IdentifierToken { value }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationToken {
    Let,
    Function,
    Object,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectReferenceToken {
    This,
    Super,
    New,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Assignment(AssignmentToken),
    Arithmetic(ArithmeticToken),
    Punctuation(PunctuatorToken),
    // Used for equality checks
    Comparison(ComparisonToken),
    // Used for combining expressions
    Logical(LogicalToken),
    Delimiter(DelimiterToken),
    ControlFlow(ControlFlowToken),
    WhiteSpace(WhiteSpaceToken),
    Identifier(IdentifierToken),
    Literal(LiteralToken),
    Declaration(DeclarationToken),
    ObjectReference(ObjectReferenceToken),
    Unknown(char),
}

#[derive(Debug, PartialEq, Clone)]
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
