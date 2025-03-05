use crate::token::*;

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

    fn eof_token(&mut self) -> Token {
        self.token_at_current_position(TokenType::Delimiter(DelimiterToken::EOF))
    }

    fn unknown_token(&mut self) -> Token {
        self.token_at_current_position(TokenType::Unknown(
            self.input.chars().nth(self.current_idx).unwrap(),
        ))
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
    pub fn end_of_input(&self) -> bool {
        self.current_idx >= self.input.len()
    }
    fn word_token(&mut self) -> Token {
        let mut word = String::new();
        while !self.end_of_input() {
            let next_char = self.input.chars().nth(self.current_idx).unwrap();
            if next_char.is_alphanumeric() {
                word.push(next_char);
                self.inc();
            } else {
                break;
            }
        }
        if word.eq("let") {
            return self.token_at_current_position(TokenType::Declaration(DeclarationToken::Let));
        } else if word.eq("fn") {
            return self
                .token_at_current_position(TokenType::Declaration(DeclarationToken::Function));
        } else if word.eq("obj") {
            return self
                .token_at_current_position(TokenType::Declaration(DeclarationToken::Object));
        } else if word.eq("true") {
            return self.token_at_current_position(TokenType::Literal(LiteralToken::Boolean(true)));
        } else if word.eq("false") {
            return self
                .token_at_current_position(TokenType::Literal(LiteralToken::Boolean(false)));
        } else if word.eq("null") {
            return self.token_at_current_position(TokenType::Literal(LiteralToken::Null));
        } else if word.eq("undefined") {
            return self.token_at_current_position(TokenType::Literal(LiteralToken::Undefined));
        } else if word.eq("this") {
            return self
                .token_at_current_position(TokenType::ObjectReference(ObjectReferenceToken::This));
        } else if word.eq("super") {
            return self.token_at_current_position(TokenType::ObjectReference(
                ObjectReferenceToken::Super,
            ));
        } else if word.eq("new") {
            return self
                .token_at_current_position(TokenType::ObjectReference(ObjectReferenceToken::New));
        } else if word.eq("if") {
            return self.token_at_current_position(TokenType::ControlFlow(ControlFlowToken::If));
        } else if word.eq("for") {
            return self.token_at_current_position(TokenType::ControlFlow(ControlFlowToken::For));
        } else if word.eq("else") {
            return self.token_at_current_position(TokenType::ControlFlow(ControlFlowToken::Else));
        } else if word.eq("in") {
            return self.token_at_current_position(TokenType::ControlFlow(ControlFlowToken::In));
        } else if word.eq("has") {
            return self.token_at_current_position(TokenType::ControlFlow(ControlFlowToken::Has));
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
                let token = self.token_at_current_position(token_type);
                self.inc();
                return token;
            } else if next_char.is_ascii_punctuation() {
                let token_type = match next_char {
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
                self.inc();
                let token = self.token_at_current_position(token_type);
                return token;
            } else {
                // If no other cases match, return an unknown token
                self.inc();
                let token = self.unknown_token();
                return token;
            }
        };
        self.start_idx = self.current_idx;
        token
    }
}
