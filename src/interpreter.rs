use crate::token::{Token, Object};
use crate::expression::Expr;
use crate::statement::Stmt;
use crate::token::token_type::TokenType;

pub fn interpret(statements: Vec<Stmt>) -> Result<(), &'static str> {
    for statement in statements {
        if let Err(why) = execute(statement) {
            return Err(why);
        }
    }
    Ok(())
}

fn execute(statement: Stmt) -> Result<(), &'static str> {
    let (print, expr) = match statement {
        Stmt::Expr(expr) => (false, *expr),
        Stmt::Print(expr) => (true, *expr),
    };
    let object = evaluate(expr)?;
    if print { println!("{:?}", object) };
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
                TokenType::Minus => binary_arithmetic(Some(minus), None, left, right),
                TokenType::Slash => binary_arithmetic(Some(slash), None, left, right),
                TokenType::Star => binary_arithmetic(Some(star), None, left, right),
                TokenType::Plus => binary_arithmetic(Some(plus), Some(concat), left, right),
                TokenType::Greater => binary_compare(greater, longer, left, right),
                TokenType::GreaterEqual => binary_compare(greater_equal, longer_equal, left, right),
                TokenType::Less => binary_compare(less, shorter, left, right),
                TokenType::LessEqual => binary_compare(less_equal, shorter_equal, left, right),
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

fn binary_arithmetic(num_fn: Option<fn(f64, f64) -> f64>, str_fn: Option<fn(&str, &str) -> String>, left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(l), Object::Number(r)) if num_fn.is_some() => Ok(Object::Number(num_fn.unwrap()(l, r))),
        (Object::String(ref l), Object::String(ref r)) if str_fn.is_some() => Ok(Object::String(str_fn.unwrap()(l, r))),
        _ => Err("Couldn't perform binary arithmetic because types didn't match/weren't supported")
    }
}

fn minus(l: f64, r: f64) -> f64 { l - r }

fn slash(l: f64, r: f64) -> f64 { l / r }

fn star(l: f64, r: f64) -> f64 { l * r }

fn plus(l: f64, r: f64) -> f64 { l + r }

fn concat(l: &str, r: &str) -> String { format!("{}{}", l, r) }

fn binary_compare(num_fn: fn(f64, f64) -> bool, str_fn: fn(&str, &str) -> bool, left: Object, right: Object) -> Result<Object, &'static str> {
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(num_fn(l, r))),
        (Object::String(ref l), Object::String(ref r)) => Ok(Object::Bool(str_fn(l, r))),
        _ => Err("Couldn't perform binary arithmetic because types didn't match/weren't supported")
    }
}

fn greater(l: f64, r: f64) -> bool { l > r }

fn longer(l: &str, r: &str) -> bool { l > r }

fn greater_equal(l: f64, r: f64) -> bool { l >= r }

fn longer_equal(l: &str, r: &str) -> bool { l >= r }

fn less(l: f64, r: f64) -> bool { l < r }

fn shorter(l: &str, r: &str) -> bool { l < r }

fn less_equal(l: f64, r: f64) -> bool { l <= r }

fn shorter_equal(l: &str, r: &str) -> bool { l <= r }

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
