#[derive(Clone, Copy)]
pub enum Literal {
    Integer(i64),
    Float(f64),
}

#[derive(Clone)]
pub enum TokenKind {
    LeftParen, RightParen,

    Plus, Minus, Mult, Div,
    Equals,
    
    Identifier,

    Number, String,
}

#[derive(Clone)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
    pub literal: Option<Literal>
}

#[derive(Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Paren(Box<Expr>),
    Literal(Literal)
}