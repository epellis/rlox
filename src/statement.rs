use crate::token::{Token, Object};
use crate::expression::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Expr>),
}
