use crate::lang::expr;
use crate::lang::lexer;

#[cfg(test)]
mod tests {
    use crate::lang::expr::{Token, TokenKind};
    use crate::lang::expr::Literal;
    use crate::lang::lexer::Lexer;

    #[test]
    fn test_lexer() {
        let input = "int hi = 4";
        let output = vec![
            Token {
                lexeme: String::from("int"),
                kind: TokenKind::Identifier,
                literal: None,
            },
            Token {
                lexeme: String::from("hi"),
                kind: TokenKind::Identifier,
                literal: None,
            },
            Token {
                lexeme: String::from("="),
                kind: TokenKind::Equals,
                literal: None,
            },
            Token {
                lexeme: String::from("4"),
                kind: TokenKind::Number,
                literal: Some(Literal::Integer(4)),
            },
        ];
        
        let mut lexer = Lexer::new(input);
        let result = lexer.lex();
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), output);
    }
}