use crate::token::{Token, Object};
use crate::expression::Expr;
use crate::token::token_type::TokenType;
use std::any::Any;

pub fn interpret(expr: Expr) -> Object {
    match expr {
        Expr::Literal(object) => object,
        Expr::Grouping(expr) => interpret(*expr),
        Expr::Unary(token, expr) => {
            let right = interpret(*expr);
            match token.type_of {
                TokenType::Bang => unary_bang(right),
                TokenType::Minus => unary_minus(right),
                _ => panic!("Could not match unary operator"),
            }
        }
        Expr::Binary(left, token, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            match token.type_of {
                TokenType::Minus => binary_minus(left, right),
                TokenType::Slash => binary_slash(left, right),
                TokenType::Star => binary_star(left, right),
                TokenType::Plus => binary_plus(left, right),
                TokenType::Greater => binary_greater(left, right),
                TokenType::GreaterEqual => binary_greater_equal(left, right),
                TokenType::Less => binary_less(left, right),
                TokenType::LessEqual => binary_less_equal(left, right),
                TokenType::EqualEqual => binary_equal_equal(left, right),
                TokenType::BangEqual => binary_bang_equal(left, right),
                _ => panic!("Could not match binary operator"),
            }
        }
        Expr::Empty => Object::None,
    }
}

fn unary_bang(right: Object) -> Object {
    match right {
        Object::Nil => Object::Bool(false),
        Object::Bool(false) => Object::Bool(false),
        _ => Object::Bool(true),
    }
}

fn unary_minus(right: Object) -> Object {
    match right {
        Object::Number(number) => Object::Number(-number),
        _ => Object::None,
    }
}

fn binary_minus(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Number(left - right),
        _ => Object::None,
    }
}

fn binary_slash(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Number(left / right),
        _ => Object::None,
    }
}

fn binary_star(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Number(left * right),
        _ => Object::None,
    }
}

fn binary_plus(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Number(left + right),
        (Object::String(left), Object::String(right)) => Object::String(format!("{}{}", left, right)),
        _ => Object::None,
    }
}

fn binary_greater(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Bool(left > right),
        (Object::String(left), Object::String(right)) => Object::Bool(left > right),
        _ => Object::None,
    }
}

fn binary_greater_equal(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Bool(left >= right),
        (Object::String(left), Object::String(right)) => Object::Bool(left >= right),
        _ => Object::None,
    }
}

fn binary_less(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Bool(left < right),
        (Object::String(left), Object::String(right)) => Object::Bool(left < right),
        _ => Object::None,
    }
}

fn binary_less_equal(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Bool(left <= right),
        (Object::String(left), Object::String(right)) => Object::Bool(left <= right),
        _ => Object::None,
    }
}

fn binary_equal_equal(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Object::Bool(left == right),
        (Object::String(left), Object::String(right)) => Object::Bool(left == right),
        (Object::Bool(left), Object::Bool(right)) => Object::Bool(left == right),
        (Object::Nil, Object::Nil) => Object::Bool(true),
        (Object::None, Object::None) => Object::Bool(true),
        _ => Object::Bool(false),
    }
}

fn binary_bang_equal(left: Object, right: Object) -> Object {
    match binary_equal_equal(left, right) {
        Object::Bool(result) => Object::Bool(!result),
        _ => Object::Bool(false),
    }
}
