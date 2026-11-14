use crate::lang::expr::{Expr, Token};

pub enum Stmt {
    Let { name: Token, value: Expr },
    Assign { name: Token, value: Expr },
    Call { name: Token, params: Vec<Expr> }
}
