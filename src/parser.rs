use crate::token::{Token, Object};
use crate::expression::Expr;
use crate::token::token_type::TokenType;
use crate::statement::Stmt;
use crate::token::token_type::TokenType::EqualEqual;

const EQUALITY_OPS: &'static [TokenType] = &[TokenType::BangEqual, TokenType::EqualEqual];
const COMPARISON_OPS: &'static [TokenType] = &[TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual];
const ADDITION_OPS: &'static [TokenType] = &[TokenType::Plus, TokenType::Minus];
const MULTIPLICATION_OPS: &'static [TokenType] = &[TokenType::Star, TokenType::Slash];
const UNARY_OPS: &'static [TokenType] = &[TokenType::Bang, TokenType::Minus];
const PRINT_OPS: &'static [TokenType] = &[TokenType::Print];

pub fn parse(tokens: &mut Vec<Token>) -> Vec<Stmt> {
    let mut statements = Vec::new();
    tokens.reverse();   // Treat like a stack
    while peek_token(tokens).type_of != TokenType::Eof {
        statements.push(statement(tokens));
    }
    statements
}

fn peek_token(tokens: &mut Vec<Token>) -> Token {
    tokens.last()
        .unwrap_or(&Token::new_keyword(TokenType::Eof, 1))
        .clone()
}

fn pop_token(tokens: &mut Vec<Token>) -> Token {
    let expr = peek_token(tokens);
    tokens.pop();
    expr
}

// If the top token is in the family, pop from stack and return true; else false
fn consume_match(tokens: &mut Vec<Token>, family: &[TokenType]) -> bool {
    match peek_token(tokens).type_of {
        TokenType::Eof => false,
        type_of if family.contains(&type_of) => {
            pop_token(tokens);
            true
        }
        _ => false,
    }
}

// Keeps popping values from the stack until it is empty or the token is found
fn consume_until_found(tokens: &mut Vec<Token>, family: &[TokenType]) -> bool {
    match pop_token(tokens).type_of {
        TokenType::Eof => false,
        type_of if family.contains(&type_of) => true,
        _ => consume_until_found(tokens, family),
    }
}

fn statement(tokens: &mut Vec<Token>) -> Stmt {
    let stmt = if consume_match(tokens, PRINT_OPS) {
        let expr = expression(tokens);
        Stmt::Print(Box::new(expr))
    } else {
        let expr = expression(tokens);
        Stmt::Expr(Box::new(expr))
    };

    if !consume_until_found(tokens, &[TokenType::Semicolon]) {
        eprintln!("Did not find ';' at end of statement");
    }
    stmt
}

fn expression(tokens: &mut Vec<Token>) -> Expr {
    equality(tokens)
}

fn equality(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = comparison(tokens);
    let mut token = peek_token(tokens);

    while consume_match(tokens, EQUALITY_OPS) {
        let right = comparison(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn comparison(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = addition(tokens);
    let mut token = peek_token(tokens);

    while consume_match(tokens, COMPARISON_OPS) {
        let right = addition(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn addition(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = multiplication(tokens);
    let mut token = peek_token(tokens);

    while consume_match(tokens, ADDITION_OPS) {
        let right = multiplication(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn multiplication(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = unary(tokens);
    let mut token = peek_token(tokens);

    while consume_match(tokens, MULTIPLICATION_OPS) {
        let right = unary(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn unary(tokens: &mut Vec<Token>) -> Expr {
    let token = peek_token(tokens);

    if consume_match(tokens, UNARY_OPS) {
        Expr::Unary(token.clone(), Box::new(expression(tokens)))
    } else {
        primary(tokens)
    }
}

fn primary(tokens: &mut Vec<Token>) -> Expr {
    let token = pop_token(tokens);
    match token.type_of {
        TokenType::Number => Expr::Literal(token.literal),
        TokenType::String => Expr::Literal(token.literal),
        TokenType::False => Expr::Literal(token.literal),
        TokenType::True => Expr::Literal(token.literal),
        TokenType::Nil => Expr::Literal(token.literal),
        TokenType::LeftParen => {
            let expr = Expr::Grouping(Box::new(expression(tokens)));
            if !consume_until_found(tokens, &[TokenType::RightParen]) {
                panic!("Couldn't find ')' for Grouping");
            }
            expr
        }
        TokenType::Eof => Expr::Empty,
        _ => {
            eprintln!("Couldn't Match! {:?}", token.type_of);
            Expr::Empty
        }
    }
}
