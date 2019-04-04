use crate::token::{Token, Object};
use crate::expression::Expr;
use crate::token::token_type::TokenType;
use crate::statement::Stmt;

const EQUALITY_OPS: &'static [TokenType] = &[TokenType::BangEqual, TokenType::EqualEqual];
const COMPARISON_OPS: &'static [TokenType] = &[TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual];
const ADDITION_OPS: &'static [TokenType] = &[TokenType::Plus, TokenType::Minus];
const MULTIPLICATION_OPS: &'static [TokenType] = &[TokenType::Star, TokenType::Slash];
const SYNCHRONIZE_OPS: &'static [TokenType] = &[TokenType::Class, TokenType::Fun, TokenType::Var, TokenType::For, TokenType::If, TokenType::While, TokenType::Print, TokenType::Return];
//const PRINT_OPS: &'static [TokenType] = &[TokenType::Print];

pub fn parse(tokens: &mut Vec<Token>) -> Vec<Stmt> {
    let mut statements = Vec::new();
    while peek_token(tokens).type_of != TokenType::Eof {
//        let expr = Box::new(expression(tokens));
//        statements.push(Stmt::Expr(expr));
        statements.push(statement(tokens));
    }
    statements
}

fn pop_token(tokens: &mut Vec<Token>) -> Token {
    // TODO: Refactor with .unwrap_or()
    match tokens.pop() {
        Some(token) => token,
        None => Token::new_keyword(TokenType::Eof, 1)
    }
}

fn peek_token(tokens: &mut Vec<Token>) -> Token {
    // TODO: Refactor with .unwrap_or()
    match tokens.last() {
        Some(token) => token.clone(),
        None => Token::new_keyword(TokenType::Eof, 1)
    }
}

// If the parser encounters an error, skip to next statement
fn synchronize(tokens: &mut Vec<Token>) {
    let mut token = pop_token(tokens);
    while token.type_of != TokenType::Semicolon || token.type_of != TokenType::Eof {
        if SYNCHRONIZE_OPS.contains(&token.type_of) {
            return;
        }
    }
}

fn statement(tokens: &mut Vec<Token>) -> Stmt {
    let token = peek_token(tokens);

    match token.type_of {
        TokenType::Print => {
            pop_token(tokens);
            print_statement(tokens)
        }
        _ => expression_statement(tokens)
    }
}

fn print_statement(tokens: &mut Vec<Token>) -> Stmt {
    let expr = expression(tokens);
    match pop_token(tokens).type_of {
        TokenType::Semicolon => (),
        _ => eprintln!("Warning! Expected ';'"),
    }
    Stmt::Print(Box::new(expr))
}

fn expression_statement(tokens: &mut Vec<Token>) -> Stmt {
    let expr = expression(tokens);
    match pop_token(tokens).type_of {
        TokenType::Semicolon => (),
        _ => eprintln!("Warning! Expected ';'"),
    }
    Stmt::Expr(Box::new(expr))
}

fn expression(tokens: &mut Vec<Token>) -> Expr {
    equality(tokens)
}

fn equality(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = comparison(tokens);
    let mut token = peek_token(tokens);

    while token.type_of != TokenType::Eof && EQUALITY_OPS.contains(&token.type_of) {
        pop_token(tokens);
        let right = comparison(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn comparison(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = addition(tokens);
    let mut token = peek_token(tokens);

    while token.type_of != TokenType::Eof && COMPARISON_OPS.contains(&token.type_of) {
        pop_token(tokens);
        let right = addition(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn addition(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = multiplication(tokens);
    let mut token = peek_token(tokens);

    while token.type_of != TokenType::Eof && ADDITION_OPS.contains(&token.type_of) {
        pop_token(tokens);
        let right = multiplication(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn multiplication(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = unary(tokens);
    let mut token = peek_token(tokens);

    while token.type_of != TokenType::Eof && MULTIPLICATION_OPS.contains(&token.type_of) {
        pop_token(tokens);
        let right = unary(tokens);
        expr = Expr::Binary(Box::new(expr.clone()), token.clone(), Box::new(right));
        token = peek_token(tokens);
    }

    expr
}

fn unary(tokens: &mut Vec<Token>) -> Expr {
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
        TokenType::Eof => Expr::Empty,
        _ => {
            eprintln!("Couldn't Match! {:?}", token.type_of);
            Expr::Empty
        }
    }
}
