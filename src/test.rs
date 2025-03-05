#[cfg(test)]
mod tests {
    use crate::lexer::Scanner;
    use crate::token::LogicalToken::Or;
    use crate::token::{
        ArithmeticToken, AssignmentToken, ComparisonToken, ControlFlowToken, DeclarationToken,
        DelimiterToken, IdentifierToken, LiteralToken, LogicalToken, ObjectReferenceToken,
        PunctuatorToken, Token, TokenSpan, TokenType,
    };

    #[test]
    fn identifier() {
        let mut lexer = Scanner::new("test");
        let token = lexer.next_token();
        assert_eq!(
            token,
            Token::new(
                TokenType::Identifier(IdentifierToken::new("test".to_string())),
                TokenSpan {
                    start: 0,
                    end: 4,
                    line: 0,
                    column: 0,
                }
            )
        );
    }
    #[test]
    fn eof_token() {
        let mut lexer = Scanner::new("");
        let token = lexer.next_token();

        assert_eq!(
            token,
            Token::new(
                TokenType::Delimiter(DelimiterToken::EOF),
                TokenSpan {
                    start: 0,
                    end: 0,
                    line: 0,
                    column: 0,
                }
            )
        );
    }
    #[test]
    fn let_keyword() {
        let mut lexer = Scanner::new("let");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Declaration(DeclarationToken::Let),
                TokenSpan {
                    start: 0,
                    end: 3,
                    line: 0,
                    column: 0,
                }
            )
        );
    }
    #[test]
    fn fn_keyword() {
        let mut lexer = Scanner::new("fn");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Declaration(DeclarationToken::Function),
                TokenSpan {
                    start: 0,
                    end: 2,
                    line: 0,
                    column: 0,
                }
            )
        );
    }
    #[test]
    fn obj_keyword() {
        let mut lexer = Scanner::new("obj");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Declaration(DeclarationToken::Object),
                TokenSpan {
                    start: 0,
                    end: 3,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    // Literal token tests
    #[test]
    fn true_literal() {
        let mut lexer = Scanner::new("true");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Literal(LiteralToken::Boolean(true)),
                TokenSpan {
                    start: 0,
                    end: 4,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn false_literal() {
        let mut lexer = Scanner::new("false");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Literal(LiteralToken::Boolean(false)),
                TokenSpan {
                    start: 0,
                    end: 5,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn null_literal() {
        let mut lexer = Scanner::new("null");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Literal(LiteralToken::Null),
                TokenSpan {
                    start: 0,
                    end: 4,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn undefined_literal() {
        let mut lexer = Scanner::new("undefined");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Literal(LiteralToken::Undefined),
                TokenSpan {
                    start: 0,
                    end: 9,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn this_literal() {
        let mut lexer = Scanner::new("this");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::ObjectReference(ObjectReferenceToken::This),
                TokenSpan {
                    start: 0,
                    end: 4,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn super_literal() {
        let mut lexer = Scanner::new("super");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::ObjectReference(ObjectReferenceToken::Super),
                TokenSpan {
                    start: 0,
                    end: 5,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn new_literal() {
        let mut lexer = Scanner::new("new");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::ObjectReference(ObjectReferenceToken::New),
                TokenSpan {
                    start: 0,
                    end: 3,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    // Delimiter token tests
    #[test]
    fn left_paren_delimiter() {
        let mut lexer = Scanner::new("(");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Delimiter(DelimiterToken::OpenParenthesis),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn right_paren_delimiter() {
        let mut lexer = Scanner::new(")");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Delimiter(DelimiterToken::CloseParenthesis),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn left_brace_delimiter() {
        let mut lexer = Scanner::new("{");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Delimiter(DelimiterToken::OpenBracket),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn right_brace_delimiter() {
        let mut lexer = Scanner::new("}");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Delimiter(DelimiterToken::CloseBracket),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn semicolon_delimiter() {
        let mut lexer = Scanner::new(";");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Punctuation(PunctuatorToken::Semicolon),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn comma_delimiter() {
        let mut lexer = Scanner::new(",");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Punctuation(PunctuatorToken::Comma),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    // Arithmetic token tests
    #[test]
    fn addition_operator() {
        let mut lexer = Scanner::new("+");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Arithmetic(ArithmeticToken::Add),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn subtraction_operator() {
        let mut lexer = Scanner::new("-");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Arithmetic(ArithmeticToken::Subtract),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn multiplication_operator() {
        let mut lexer = Scanner::new("*");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Arithmetic(ArithmeticToken::Multiply),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn division_operator() {
        let mut lexer = Scanner::new("/");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Arithmetic(ArithmeticToken::Divide),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    // Comparison token tests
    #[test]
    fn equals_comparison() {
        let mut lexer = Scanner::new("==");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Comparison(ComparisonToken::Equal),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn not_equals_comparison() {
        let mut lexer = Scanner::new("!=");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Comparison(ComparisonToken::NotEqual),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn greater_than_comparison() {
        let mut lexer = Scanner::new(">");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Comparison(ComparisonToken::GreaterThan),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn less_than_comparison() {
        let mut lexer = Scanner::new("<");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Comparison(ComparisonToken::LessThan),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn greater_equal_comparison() {
        let mut lexer = Scanner::new(">=");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Comparison(ComparisonToken::GreaterThanOrEqual),
                TokenSpan {
                    start: 0,
                    end: 2,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn less_equal_comparison() {
        let mut lexer = Scanner::new("<=");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Comparison(ComparisonToken::LessThanOrEqual),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    // Assignment token tests
    #[test]
    fn assignment_operator() {
        let mut lexer = Scanner::new("=");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Assignment(AssignmentToken::Assign),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    // Boolean operator tests
    #[test]
    fn and_operator() {
        let mut lexer = Scanner::new("&&");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Logical(LogicalToken::And),
                TokenSpan {
                    start: 0,
                    end: 2,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn or_operator() {
        let mut lexer = Scanner::new("||");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Logical(Or),
                TokenSpan {
                    start: 0,
                    end: 2,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    #[test]
    fn not_operator() {
        let mut lexer = Scanner::new("!");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::Comparison(ComparisonToken::Not),
                TokenSpan {
                    start: 0,
                    end: 1,
                    line: 0,
                    column: 0,
                }
            )
        );
    }

    // Statement token tests
    #[test]
    fn if_statement() {
        let mut lexer = Scanner::new("if");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::ControlFlow(ControlFlowToken::If),
                TokenSpan {
                    start: 0,
                    end: 2,
                    line: 0,
                    column: 0,
                }
            )
        );
    }
    
    #[test] 
    fn numbers () {
        let mut lexer = Scanner::new("100");
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token::new(
                TokenType::ControlFlow(ControlFlowToken::If),
                TokenSpan {
                    start: 0,
                    end: 2,
                    line: 0,
                    column: 0,
                }
            )
        );
    }
    
}
