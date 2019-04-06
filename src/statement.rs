use crate::token::{Token, Object};
use crate::expression::Expr;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Box<Stmt>),
    While(Box<Expr>, Box<Stmt>),
}
