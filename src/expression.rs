use crate::token::{Token, Object};


#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Object),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    None    // TODO: This is temporary!!!
}

//pub trait Expressible {}
//
//pub type Expr = Box<dyn Expressible>;
//
////pub struct Binary {
////    pub left: Expr,
////    pub operator: Token,
////    pub right: Expr,
////}
//
//pub struct Literal {
//    pub value: Object,
//}
//
////pub struct Unary {
////    pub operator: Token,
////    pub right: Expr,
////}
//
////impl Expressible for Binary {}
//
////impl Expressible for Grouping {}
//
//impl Expressible for Literal {}
//
////impl Expressible for Unary {}
