use crate::token::{Token, Object};
use crate::expression::Expr;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::EqualEqual;

const EQUALITY_OPS: &'static [TokenType] = &[TokenType::Equal, TokenType::EqualEqual];
const COMPARISON_OPS: &'static [TokenType] = &[TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual];
const ADDITION_OPS: &'static [TokenType] = &[TokenType::Plus, TokenType::Minus];
const MULTIPLICATION_OPS: &'static [TokenType] = &[TokenType::Star, TokenType::Slash];

pub fn parse(tokens: &mut Vec<Token>) -> Expr {
    expression(tokens)
}

fn pop_token(tokens: &mut Vec<Token>) -> Token {
    println!("Pop: Tokens: {:?}", tokens.clone());
    match tokens.pop() {
        Some(token) => token,
        None => Token::new_keyword(TokenType::Eof, 1)
    }
}

fn peek_token(tokens: &mut Vec<Token>) -> Token {
    println!("Peek: Tokens: {:?}", tokens.clone());
    match tokens.last() {
        Some(token) => token.clone(),
        None => Token::new_keyword(TokenType::Eof, 1)
    }
}

fn expression(tokens: &mut Vec<Token>) -> Expr {
    println!("Expression");
    equality(tokens)
}

fn equality(tokens: &mut Vec<Token>) -> Expr {
    println!("Equality");
    let mut expr = comparison(tokens);
    let mut token = peek_token(tokens);

    while token.type_of != TokenType::Eof && EQUALITY_OPS.contains(&token.type_of) {
        println!("Equality Loop");
        pop_token(tokens);
        let right = comparison(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn comparison(tokens: &mut Vec<Token>) -> Expr {
    println!("Comparison");
    let mut expr = addition(tokens);
    let mut token = peek_token(tokens);

    while token.type_of != TokenType::Eof && COMPARISON_OPS.contains(&token.type_of) {
        println!("Comparison Loop");
        pop_token(tokens);
        let right = addition(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn addition(tokens: &mut Vec<Token>) -> Expr {
    println!("Addition");
    let mut expr = multiplication(tokens);
    let mut token = peek_token(tokens);

    while token.type_of != TokenType::Eof && ADDITION_OPS.contains(&token.type_of) {
        println!("Addition Loop");
        pop_token(tokens);
        let right = multiplication(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn multiplication(tokens: &mut Vec<Token>) -> Expr {
    println!("Multiplication");
    let mut expr = unary(tokens);
    let mut token = peek_token(tokens);

    while token.type_of != TokenType::Eof && MULTIPLICATION_OPS.contains(&token.type_of) {
        println!("Multiplication Loop");
        pop_token(tokens);
        let right = unary(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn unary(tokens: &mut Vec<Token>) -> Expr {
    println!("Unary");
    let token = pop_token(tokens);
    match token.type_of {
        TokenType::Bang => Expr::Unary(token.clone(), Box::new(expression(tokens))),
        TokenType::Minus => Expr::Unary(token.clone(), Box::new(expression(tokens))),
        _ => {
            tokens.push(token);
            primary(tokens)
        }
    }
}

fn primary(tokens: &mut Vec<Token>) -> Expr {
    println!("Primary");
    let token = pop_token(tokens);
    match token.type_of {
        TokenType::Number => Expr::Literal(token.literal),
        TokenType::String => Expr::Literal(token.literal),
        TokenType::False => Expr::Literal(token.literal),
        TokenType::True => Expr::Literal(token.literal),
        TokenType::Nil => Expr::Literal(token.literal),
        TokenType::LeftParen => {
            let expr = Expr::Grouping(Box::new(expression(tokens)));
            match &tokens.pop() {
                Some(token) if token.type_of == TokenType::RightParen => (),
                Some(token) => eprintln!("Expected ')', Got: {:?}", token.clone()),
                None => panic!("Couldn't find ')' for Grouping"),
            }
            expr
        }
        TokenType::Eof => {
            println!("Encountered EOF Token");
            Expr::None
        }
        _ => {
            eprintln!("Couldn't Match! {:?}", token.type_of);
            Expr::None
        }
    }
}
