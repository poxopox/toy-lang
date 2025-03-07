use crate::token::*;
use crate::token::TokenType::Literal;

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_of_input() {
            None
        } else {
            Some(self.next_token())
        }
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
    fn dec(&mut self) {
        self.current_idx = self.current_idx - 1;
    }
    fn current_char(&mut self) -> char {
        self.input.chars().nth(self.current_idx).unwrap()
    }
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.current_idx + 1)
    }

    fn eof_token(&mut self) -> Token {
        self.capture_token(TokenType::Delimiter(DelimiterToken::EOF))
    }

    fn capture_token(&mut self, token_type: TokenType) -> Token {
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
    pub fn end_of_input(&self) -> bool {
        self.current_idx >= self.input.len()
    }
    fn word_token(&mut self) -> Token {
        let mut word = String::new();
        let mut is_number = true;
        let mut is_float = true;
        while !self.end_of_input() {
            let next_char = self.current_char();
            if next_char.is_alphanumeric() {
                word.push(next_char);
                if let Some(next_next_char) = self.peek() {
                    if !next_next_char.is_alphanumeric() {
                        break;
                    }
                } else if self.end_of_input() {
                    break;
                }
            } else if next_char == '.' {
                if !is_float {
                    is_float = true;
                } else {
                    is_number = false;
                }
            } else {
                break;
            }
            self.inc();
        }
        let token_type = match word.as_str() {
            // Declarations
            "let" => TokenType::Declaration(DeclarationToken::Let),
            "fn" => TokenType::Declaration(DeclarationToken::Function),
            "obj" => TokenType::Declaration(DeclarationToken::Object),

            // Literals
            "true" => TokenType::Literal(LiteralToken::Boolean(true)),
            "false" => TokenType::Literal(LiteralToken::Boolean(false)),
            "null" => TokenType::Literal(LiteralToken::Null),
            "undefined" => TokenType::Literal(LiteralToken::Undefined),

            // Object References
            "this" => TokenType::ObjectReference(ObjectReferenceToken::This),
            "super" => TokenType::ObjectReference(ObjectReferenceToken::Super),
            "new" => TokenType::ObjectReference(ObjectReferenceToken::New),

            // Control Flow
            "if" => TokenType::ControlFlow(ControlFlowToken::If),
            "for" => TokenType::ControlFlow(ControlFlowToken::For),
            "else" => TokenType::ControlFlow(ControlFlowToken::Else),
            "in" => TokenType::ControlFlow(ControlFlowToken::In),
            "has" => TokenType::ControlFlow(ControlFlowToken::Has),
            "return" => TokenType::ControlFlow(ControlFlowToken::Return),

            // Default - Identifier
            _ => {
                let token =  if let Ok(num) = word.parse::<u64>() {
                    Literal(LiteralToken::Number(NumberToken::UnsignedInteger(num)))
                } else if let Ok(num) = word.parse::<i64>() {
                    Literal(LiteralToken::Number(NumberToken::SignedInteger(num)))
                } else if let Ok(num) = word.parse::<f64>() {
                    Literal(LiteralToken::Number(NumberToken::Float(num)))
                } else {
                    TokenType::Identifier(IdentifierToken { value: word })
                };
                token
            }
        };
        let token = self.capture_token(token_type);
        self.inc();
        token
    }

    pub fn next_token(&mut self) -> Token {
        //First, make sure it's not the end of input
        let token = if self.end_of_input() {
            self.eof_token()
        } else {
            let next_char = self.current_char();
            if next_char.is_alphanumeric() {
                // If the next character is alphanumeric, it's a word token
                self.word_token()
            } else {
                let token_type = match next_char {
                    ' ' => TokenType::WhiteSpace(WhiteSpaceToken::Space),
                    '\t' => TokenType::WhiteSpace(WhiteSpaceToken::Tab),
                    '\n' => TokenType::WhiteSpace(WhiteSpaceToken::NewLine),
                    '(' => TokenType::Delimiter(DelimiterToken::OpenParenthesis),
                    ')' => TokenType::Delimiter(DelimiterToken::CloseParenthesis),
                    '[' => TokenType::Delimiter(DelimiterToken::OpenBrace),
                    ']' => TokenType::Delimiter(DelimiterToken::CloseBrace),
                    '{' => TokenType::Delimiter(DelimiterToken::OpenBracket),
                    '}' => TokenType::Delimiter(DelimiterToken::CloseBracket),
                    ',' => TokenType::Punctuation(PunctuatorToken::Comma),
                    '.' => TokenType::Punctuation(PunctuatorToken::Dot),
                    ':' => TokenType::Punctuation(PunctuatorToken::Colon),
                    ';' => TokenType::Punctuation(PunctuatorToken::Semicolon),
                    '+' => TokenType::Arithmetic(ArithmeticToken::Add),
                    '-' => TokenType::Arithmetic(ArithmeticToken::Subtract),
                    '!' => TokenType::Comparison(ComparisonToken::Not),
                    '=' => TokenType::Assignment(AssignmentToken::Assign),
                    '/' => TokenType::Arithmetic(ArithmeticToken::Divide),
                    '&' => TokenType::Arithmetic(ArithmeticToken::And),
                    '|' => TokenType::Arithmetic(ArithmeticToken::And),
                    '*' => TokenType::Arithmetic(ArithmeticToken::Multiply),
                    '>' => TokenType::Comparison(ComparisonToken::GreaterThan),
                    '<' => TokenType::Comparison(ComparisonToken::LessThan),
                    _ => TokenType::Unknown(next_char),
                };
                let token = self.capture_token(token_type);
                self.inc();
                token
            }
        };
        self.start_idx = self.current_idx;
        token
    }
}
