use crate::token::{Token, Object};
use crate::expression::Expr;
use crate::statement::Stmt;
use crate::token::token_type::TokenType;
use crate::environment::Environment;

pub fn interpret(statements: Vec<Stmt>, is_repl: bool) -> Result<(), &'static str> {
    let mut environment = Environment::new_root();
    for statement in statements {
        if let Err(why) = execute(statement, &mut environment, is_repl) {
            return Err(why);
        }
    }
    Ok(())
}

fn execute(statement: Stmt, env: &mut Environment, is_repl: bool) -> Result<(), &'static str> {
    let expr = match &statement {
        Stmt::Expr(expr) => *expr.clone(),
        Stmt::Print(expr) => *expr.clone(),
        Stmt::Var(_, expr) => *expr.clone(),
        Stmt::Block(_) => Expr::Empty,
        Stmt::If(expr, _, _) => *expr.clone(),
        Stmt::While(expr, _) => *expr.clone(),
    };
    let object = evaluate(expr, env)?;
//    let object = evaluate(expr.clone(), env)?;
//    println!("{:?} -> {:?}", expr, object);

    match statement {
        Stmt::Expr(_) if is_repl => println!("{:?}", object),
        Stmt::Expr(_) => {},
        Stmt::Print(_) => println!("{:?}", object),
        Stmt::Var(token, _) => env.define(token.lexeme, object),
        Stmt::Block(block) => {
            execute_block(block, env);
        },
        Stmt::If(_, then_branch, else_branch) => {
            match object {
                Object::Bool(true) => execute(*then_branch, env, false)?,
                Object::Bool(false) => execute(*else_branch, env, false)?,
                _ => panic!("Conditional did not evaluate to true or false"),
            };
        },
        Stmt::While(expr, body) => {
            while truthiness(&evaluate(*expr.clone(), env)?) {
                execute(*body.clone(), env, false)?;
            }
        },
    }
    Ok(())
}

fn execute_block(statements: Vec<Box<Stmt>>, parent_env: &Environment) -> Result<(), &'static str> {
    let mut env = Environment::new_child(parent_env);
    for statement in statements {
        execute(*statement, &mut env, false)?;
    }
    Ok(())
}

fn evaluate(expression: Expr, env: &mut Environment) -> Result<Object, &'static str> {
    match expression {
        Expr::Literal(object) => Ok(object),
        Expr::Grouping(expr) => evaluate(*expr, env),
        Expr::Unary(token, expr) => {
            let right = evaluate(*expr, env)?;
            match token.type_of {
                TokenType::Bang => unary_bang(right),
                TokenType::Minus => unary_minus(right),
                _ => panic!("Could not match unary operator"),
            }
        }
        Expr::Logical(left, token, right) => {
            let left = evaluate(*left, env)?;

            let truth = truthiness(&left);

            let object = match token.type_of {
                TokenType::Or if truth => left,
                TokenType::Or if !truth => evaluate(*right, env)?,
                TokenType::And if !truth => left,
                TokenType::And if truth => evaluate(*right, env)?,
                _ => panic!("Conditional did not evaluate to true or false"),
            };
            Ok(object)
        }

        Expr::Binary(left, token, right) => {
            let left = evaluate(*left, env)?;
            let right = evaluate(*right, env)?;
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
        Expr::Variable(token) => Ok(env.get(token.lexeme)),
        Expr::Assign(token, expr) => {
            let object = evaluate(*expr, env)?;
            env.assign(token.lexeme, object);
            Ok(Object::None)
        }
        Expr::Empty => Ok(Object::None),
    }
}

fn unary_bang(right: Object) -> Result<Object, &'static str> {
    match right {
        Object::Nil => Ok(Object::Bool(false)),
        Object::Bool(value) => Ok(Object::Bool(!value)),
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
    match binary_equal_equal(left, right)? {
        Object::Bool(result) => Ok(Object::Bool(!result)),
        _ => Ok(Object::None),
    }
}

fn truthiness(object: &Object) -> bool {
    match object {
        Object::Bool(truthiness) => *truthiness,
        _ => panic!("Object was not true or false"),
    }
}
