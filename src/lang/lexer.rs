use anyhow::{Result};
use crate::lang::errors::KovaError;
use crate::lang::expr::{Literal, TokenKind, Token};

pub struct Lexer {
    pos: usize,
    chars: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            pos: 0,
            chars: input.chars().collect(),
        }
    }
    
    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while self.pos < self.chars.len() {
            let char = self.chars[self.pos];

            if char.is_whitespace() {
                self.pos += 1;
                continue;
            }

            if char.is_ascii_digit() {
                let start = self.pos;

                while self.pos < self.chars.len()
                    && self.chars[self.pos].is_ascii_digit()
                {
                    self.pos += 1;
                }

                let lexeme: String = self.chars[start..self.pos].iter().collect();
                let value = match lexeme.parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => return Err(KovaError::LexerError { message: "invalid integer".into() }.into()),
                };

                tokens.push(Token {
                    lexeme,
                    kind: TokenKind::Number,
                    literal: Some(Literal::Integer(value)),
                });

                continue;
            }

            if char.is_alphabetic() || char == '_' {
                let start = self.pos;

                while self.pos < self.chars.len()
                    && (self.chars[self.pos].is_alphanumeric()
                    || self.chars[self.pos] == '_')
                {
                    self.pos += 1;
                }

                let lexeme: String = self.chars[start..self.pos].iter().collect();

                tokens.push(Token {
                    lexeme,
                    kind: TokenKind::Identifier,
                    literal: None,
                });

                continue;
            }

            let kind = match char {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Mult,
                '/' => TokenKind::Div,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '=' => TokenKind::Equals,
                _ => return Err(KovaError::LexerError { message: "invalid token".into() }.into()),
            };

            tokens.push(Token {
                lexeme: char.to_string(),
                kind,
                literal: None,
            });

            self.pos += 1;
        }

        Ok(tokens)
    }
}