use crate::treewalk::token::{Token};
use crate::treewalk::expression::Expr;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Return(Token, Box<Expr>),
    Var(Token, Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Box<Stmt>),
    While(Box<Expr>, Box<Stmt>),
    Function(Token, Vec<Token>, Vec<Box<Stmt>>),
    Break,
}
