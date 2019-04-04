use crate::token::{Token, Object};
use crate::expression::Expr;
use crate::statement::Stmt;
use crate::token::token_type::TokenType;

pub fn interpret(statements: Vec<Stmt>) -> Result<(), &'static str> {
    for statement in statements {
        let (print, expr) = match statement {
            Stmt::Expr(expr) => (false, *expr),
            Stmt::Print(expr) => (true, *expr),
        };
        let object = evaluate(expr)?;
        if print { println!("{:?}", object) };
    }
    Ok(())
}

fn evaluate(expression: Expr) -> Result<Object, &'static str> {
    match expression {
        Expr::Literal(object) => Ok(object),
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::Unary(token, expr) => {
            let right = evaluate(*expr)?;
            match token.type_of {
                TokenType::Bang => unary_bang(right),
                TokenType::Minus => unary_minus(right),
                _ => panic!("Could not match unary operator"),
            }
        }
        Expr::Binary(left, token, right) => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;
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
        Expr::Empty => Ok(Object::None),
    }
}

fn unary_bang(right: Object) -> Result<Object, &'static str> {
    match right {
        Object::Nil => Ok(Object::Bool(false)),
        Object::Bool(false) => Ok(Object::Bool(false)),
        _ => Ok(Object::Bool(true)),
    }
}

fn unary_minus(right: Object) -> Result<Object, &'static str> {
    match right {
        Object::Number(number) => Ok(Object::Number(-number)),
        _ => Err("Cannot perform -/1 on a non-number"),
    }
}

fn binary_minus(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left - right)),
        _ => Err("Cannot perform -/2 on a non-number"),
    }
}

fn binary_slash(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left / right)),
        _ => Err("Cannot perform (/)/2 on a non-number"),
    }
}

fn binary_star(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left * right)),
        _ => Err("Cannot perform */2 on a non-number"),
    }
}

fn binary_plus(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left + right)),
        (Object::String(left), Object::String(right)) => Ok(Object::String(format!("{}{}", left, right))),
        _ => Err("Cannot perform +/2 on a non-number or non-string"),
    }
}

fn binary_greater(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left > right)),
        (Object::String(left), Object::String(right)) => Ok(Object::Bool(left > right)),
        _ => Err("Cannot perform >/2 on a non-number or non-string"),
    }
}

fn binary_greater_equal(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left >= right)),
        (Object::String(left), Object::String(right)) => Ok(Object::Bool(left >= right)),
        _ => Err("Cannot perform >=/2 on a non-number or non-string"),
    }
}

fn binary_less(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left < right)),
        (Object::String(left), Object::String(right)) => Ok(Object::Bool(left < right)),
        _ => Err("Cannot perform </2 on a non-number or non-string"),
    }
}

fn binary_less_equal(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left <= right)),
        (Object::String(left), Object::String(right)) => Ok(Object::Bool(left <= right)),
        _ => Err("Cannot perform </2 on a non-number or non-string"),
    }
}

fn binary_equal_equal(left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left == right)),
        (Object::String(left), Object::String(right)) => Ok(Object::Bool(left == right)),
        (Object::Bool(left), Object::Bool(right)) => Ok(Object::Bool(left == right)),
        (Object::Nil, Object::Nil) => Ok(Object::Bool(true)),
        (Object::None, Object::None) => Ok(Object::Bool(true)),
        _ => Err("Cannot perform ==/2 on a non-number or non-string"),
    }
}

fn binary_bang_equal(left: Object, right: Object) -> Result<Object, &'static str> {
    match binary_equal_equal(left, right) {
        Ok(Object::Bool(result)) => Ok(Object::Bool(!result)),
        Err(why) => Err(why),
        _ => Ok(Object::None),
    }
}
