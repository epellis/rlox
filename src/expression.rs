use crate::token::{Token, Object};

#[derive(Clone)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Object),
    Unary(Token, Box<Expr>),
    Variable(Token),
    Grouping(Box<Expr>),
    Empty    // TODO: This is temporary!!!
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Assign(token, expr) => {
                write!(f, "(Assign {:?}, {:?})", token, expr)
            },
            Expr::Binary(left, token, right) => {
                write!(f, "({:?} {:?} {:?})", token, left, right)
            },
            Expr::Literal(object) => write!(f, "{:?}", object),
            Expr::Unary(token, expr) => write!(f, "({:?} {:?})", token, expr),
            Expr::Variable(token) => write!(f, "{:?}", token),
            Expr::Grouping(expr) => write!(f, "({:?})", expr),
            Expr::Empty => write!(f, "()"),
        }
    }
}
