use crate::token::{Token, Object};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Expressible {}

pub type Expr = Rc<RefCell<dyn Expressible>>;

pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

pub struct Grouping {
    pub expression: Box<dyn Expressible>,
}

pub struct Literal {
    pub value: Object,
}

pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

impl Expressible for Binary {}

impl Expressible for Grouping {}

impl Expressible for Literal {}

impl Expressible for Unary {}
