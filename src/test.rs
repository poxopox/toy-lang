#[cfg(test)]
mod tests {
    use crate::lexer::Scanner;
    use crate::token::{DelimiterToken, IdentifierToken, Token, TokenSpan, TokenType};
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
    fn eof() {
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
}
