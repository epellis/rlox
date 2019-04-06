use crate::token::{Token, Object};
use crate::expression::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Box<Expr>)
}
