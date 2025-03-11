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
    cursor_start: usize,
    cursor_end: usize,
    current_line: usize,
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

impl Scanner {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            cursor_start: 0,
            cursor_end: 0,
            current_line: 0,
        }
    }
    fn inc(&mut self) {
        self.cursor_end = self.cursor_end + 1;
    }
    fn new_line(&mut self) -> TokenType {
        self.current_line += 1;
        TokenType::WhiteSpace(WhiteSpaceToken::NewLine)
    }
    fn dec(&mut self) {
        self.cursor_end = self.cursor_end - 1;
    }
    fn current_char(&mut self) -> char {
        self.input.chars().nth(self.cursor_end).unwrap()
    }
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.cursor_end + 1)
    }

    fn eof_token(&mut self) -> Token {
        self.capture_token(TokenType::Delimiter(DelimiterToken::EOF))
    }

    fn capture_token(&mut self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            TokenSpan {
                start: self.cursor_start,
                end: self.cursor_end,
                line: self.current_line,
            },
        )
    }
    pub fn end_of_input(&self) -> bool {
        self.cursor_end >= self.input.len()
    }

    fn tokenize_string_literal(&mut self, delimiter: char) -> TokenType {
        let mut string_value = String::new();
        let mut is_escaped = false;

        while let Some(next_char) = self.peek() {
            self.inc(); // Consume the current character

            if is_escaped {
                // Handle escaped characters
                let escaped_char = match next_char {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    '`' => '`',
                    // For any other character after \, just use the character itself
                    _ => next_char,
                };
                string_value.push(escaped_char);
                is_escaped = false;
            } else if next_char == '\\' {
                is_escaped = true;
            } else if next_char == delimiter {
                break;
            } else {
                string_value.push(next_char);
            }
        }

        TokenType::Literal(LiteralToken::String(string_value))
    }


    pub fn next_token(&mut self) -> Token {
        //First, make sure it's not the end of input
        let token = if self.end_of_input() {
            self.eof_token()
        } else {
            // If the next character is alphanumeric, it's a word token
            let mut word = String::new();
            while !self.end_of_input() {
                let current_char = self.current_char();
                word.push(current_char);
                if current_char.is_alphanumeric() {
                    if let Some(peeked) = self.peek() {
                        if !peeked.is_whitespace() {
                            self.inc();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
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
                "\"" => self.tokenize_string_literal('"'),
                "'" => self.tokenize_string_literal('\''),
                "`" => self.tokenize_string_literal('`'),
                " " => TokenType::WhiteSpace(WhiteSpaceToken::Space),
                "\t" => TokenType::WhiteSpace(WhiteSpaceToken::Tab),
                "\n" => self.new_line(),
                "(" => TokenType::Delimiter(DelimiterToken::OpenParenthesis),
                ")" => TokenType::Delimiter(DelimiterToken::CloseParenthesis),
                "[" => TokenType::Delimiter(DelimiterToken::OpenBrace),
                "]" => TokenType::Delimiter(DelimiterToken::CloseBrace),
                "{" => TokenType::Delimiter(DelimiterToken::OpenBracket),
                "}" => TokenType::Delimiter(DelimiterToken::CloseBracket),
                "," => TokenType::Punctuation(PunctuatorToken::Comma),
                "." => TokenType::Punctuation(PunctuatorToken::Dot),
                ":" => TokenType::Punctuation(PunctuatorToken::Colon),
                ";" => TokenType::Punctuation(PunctuatorToken::Semicolon),
                "+" => TokenType::Arithmetic(ArithmeticToken::Add),
                "-" => TokenType::Arithmetic(ArithmeticToken::Subtract),
                "=" => TokenType::Assignment(AssignmentToken::Assign),
                "/" => TokenType::Arithmetic(ArithmeticToken::Divide),
                "&" => TokenType::Arithmetic(ArithmeticToken::And),
                "|" => TokenType::Arithmetic(ArithmeticToken::Or),
                "*" => TokenType::Arithmetic(ArithmeticToken::Multiply),
                "!" => TokenType::Comparison(ComparisonToken::Not),
                ">" => TokenType::Comparison(ComparisonToken::GreaterThan),
                "<" => TokenType::Comparison(ComparisonToken::LessThan),
                // Default - Identifier
                _ => {
                    let token = if let Ok(num) = word.parse::<i64>() {
                        TokenType::Literal(LiteralToken::Number(NumberToken::SignedInteger(num)))
                    } else if let Ok(num) = word.parse::<f64>() {
                        TokenType::Literal(LiteralToken::Number(NumberToken::Float(num)))
                    } else {
                        TokenType::Identifier(IdentifierToken { value: word })
                    };
                    token
                }
            };
            let token = self.capture_token(token_type);
            self.inc();
            token
        };
        self.cursor_start = self.cursor_end;
        token
    }
}
