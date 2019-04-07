use crate::token::{Token, Object};
use crate::expression::Expr;
use crate::token::token_type::TokenType;
use crate::statement::Stmt;
use crate::token::token_type::TokenType::{EqualEqual, Identifier};

const EQUALITY_OPS: &'static [TokenType] = &[TokenType::BangEqual, TokenType::EqualEqual];
const COMPARISON_OPS: &'static [TokenType] = &[TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual];
const ADDITION_OPS: &'static [TokenType] = &[TokenType::Plus, TokenType::Minus];
const MULTIPLICATION_OPS: &'static [TokenType] = &[TokenType::Star, TokenType::Slash];
const UNARY_OPS: &'static [TokenType] = &[TokenType::Bang, TokenType::Minus];

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
// Returns whether or not the token was found before EoF
fn consume_until_found(tokens: &mut Vec<Token>, family: &[TokenType]) -> bool {
    match pop_token(tokens).type_of {
        TokenType::Eof => false,
        type_of if family.contains(&type_of) => true,
        _ => consume_until_found(tokens, family),
    }
}

fn try_consume(tokens: &mut Vec<Token>, family: &[TokenType], message: &'static str) {
    if !family.contains(&peek_token(tokens).type_of) {
        panic!(message);
    }
    pop_token(tokens);
}

pub fn parse(tokens: &mut Vec<Token>) -> Vec<Stmt> {
    let mut statements = Vec::new();
    tokens.reverse();   // Treat like a stack
    while peek_token(tokens).type_of != TokenType::Eof {
        statements.push(declaration(tokens));
    }
    statements
}

fn declaration(tokens: &mut Vec<Token>) -> Stmt {
    if consume_match(tokens, &[TokenType::Fun]) {
        function(tokens, "function")
    } else if consume_match(tokens, &[TokenType::Var]) {
        var_declaration(tokens)
    } else {
        statement(tokens)
    }
    // TODO: Synchronize
}

fn function(tokens: &mut Vec<Token>, kind: &'static str) -> Stmt {
    let name = peek_token(tokens);
    try_consume(tokens, &[TokenType::Identifier], "Expected Identifier");
    try_consume(tokens, &[TokenType::LeftParen], "Expected LeftParen");

    let mut parameters = Vec::new();

    if peek_token(tokens).type_of != TokenType::RightParen {
        loop {
            let parameter = peek_token(tokens);
            if !consume_match(tokens, &[TokenType::Identifier]) {
                break;
            }
            parameters.push(parameter);
            if !consume_match(tokens, &[TokenType::Comma]) {
                break;
            }
        }
    }
    try_consume(tokens, &[TokenType::RightParen], "Expected RightParen");
    try_consume(tokens, &[TokenType::LeftBrace], "Expected LeftBrace");

    let body = block(tokens);

    Stmt::Function(name, parameters, body)
}

fn var_declaration(tokens: &mut Vec<Token>) -> Stmt {
    let name = pop_token(tokens);
    if name.type_of != TokenType::Identifier {
        panic!("Expected identifier");
    }

    let initializer = if consume_match(tokens, &[TokenType::Equal]) {
        expression(tokens)
    } else {
        Expr::Empty
    };

    try_consume(tokens, &[TokenType::Semicolon], "Couldn't find ';' at end of statement");
    Stmt::Var(name, Box::new(initializer))
}

fn statement(tokens: &mut Vec<Token>) -> Stmt {
    // Dispatch Print
    if consume_match(tokens, &[TokenType::Print]) {
        let expr = expression(tokens);
        let stmt = Stmt::Print(Box::new(expr));
        try_consume(tokens, &[TokenType::Semicolon], "Couldn't find ';' at end of statement");
        return stmt;
    }

    // Dispatch Blocked Statements
    let stmt = if consume_match(tokens, &[TokenType::LeftBrace]) {
        Stmt::Block(block(tokens))
    } else if consume_match(tokens, &[TokenType::If]) {
        if_statement(tokens)
    } else if consume_match(tokens, &[TokenType::If]) {
        if_statement(tokens)
    } else if consume_match(tokens, &[TokenType::While]) {
        while_statement(tokens)
    } else if consume_match(tokens, &[TokenType::For]) {
        for_statement(tokens)
    } else if consume_match(tokens, &[TokenType::Return]) {
        return_statement(tokens)
    } else if consume_match(tokens, &[TokenType::Break]) {
        try_consume(tokens, &[TokenType::Semicolon], "Couldn't find ';' at end of statement");
        Stmt::Break
    } else {
        let expr = expression(tokens);
        let stmt = Stmt::Expr(Box::new(expr));
        try_consume(tokens, &[TokenType::Semicolon], "Couldn't find ';' at end of statement");
        stmt
    };
    stmt
}

fn return_statement(tokens: &mut Vec<Token>) -> Stmt {
    let keyword = Token::new_keyword(TokenType::Return, 1);
    let mut value = Expr::Empty;
    if peek_token(tokens).type_of != TokenType::Semicolon {
        value = expression(tokens);
    }

    try_consume(tokens, &[TokenType::Semicolon], "Expect ';' after return");
    Stmt::Return(keyword, Box::new(value))
}

fn for_statement(tokens: &mut Vec<Token>) -> Stmt {
    try_consume(tokens, &[TokenType::LeftParen], "Expect '(' after for");

    let initializer = if consume_match(tokens, &[TokenType::Semicolon]) {
        Stmt::Expr(Box::new(Expr::Empty))
    } else if consume_match(tokens, &[TokenType::Var]) {
        var_declaration(tokens)
    } else {
        Stmt::Expr(Box::new(expression(tokens)))
    };


    let condition = if peek_token(tokens).type_of == TokenType::Semicolon {
        Expr::Literal(Object::Bool(true))
    } else {
        expression(tokens)
    };
    try_consume(tokens, &[TokenType::Semicolon], "Expect ';' after loop");

    let increment = if peek_token(tokens).type_of == TokenType::RightParen {
        Expr::Empty
    } else {
        expression(tokens)
    };
    try_consume(tokens, &[TokenType::RightParen], "Expect ')' after for");

    let mut body = statement(tokens);

    if increment != Expr::Empty {
        body = Stmt::Block(vec![
            Box::new(body),
            Box::new(Stmt::Expr(Box::new(increment))),
        ]);
    }

    body = Stmt::While(Box::new(condition), Box::new(body));

    if initializer != Stmt::Expr(Box::new(Expr::Empty)) {
        body = Stmt::Block(vec![
            Box::new(initializer),
            Box::new(body),
        ]);
    }

    body
}

fn while_statement(tokens: &mut Vec<Token>) -> Stmt {
    try_consume(tokens, &[TokenType::LeftParen], "Expect '(' after while");
    let condition = expression(tokens);
    try_consume(tokens, &[TokenType::RightParen], "Expect ')' after while");
    let body = statement(tokens);

    Stmt::While(Box::new(condition), Box::new(body))
}

fn if_statement(tokens: &mut Vec<Token>) -> Stmt {
    try_consume(tokens, &[TokenType::LeftParen], "Expect '(' after if");
    let condition = expression(tokens);
    try_consume(tokens, &[TokenType::RightParen], "Expect ')' after condition");

    let then_branch = statement(tokens);
    let else_branch = if consume_match(tokens, &[TokenType::Else]) {
        statement(tokens)
    } else {
        Stmt::Expr(Box::new(Expr::Empty))
    };

    Stmt::If(Box::new(condition), Box::new(then_branch), Box::new(else_branch))
}

fn block(tokens: &mut Vec<Token>) -> Vec<Box<Stmt>> {
    let mut statements = Vec::new();

    while peek_token(tokens).type_of != TokenType::RightBrace {
        statements.push(Box::new(declaration(tokens)));

        if peek_token(tokens).type_of == TokenType::Eof {
            panic!("Could not find matching '}'");
        }
    }

    consume_until_found(tokens, &[TokenType::RightBrace]);

    statements
}

fn expression(tokens: &mut Vec<Token>) -> Expr {
    assignment(tokens)
}

fn assignment(tokens: &mut Vec<Token>) -> Expr {
    let expr = or(tokens);
    let token = peek_token(tokens);

    if consume_match(tokens, &[TokenType::Equal]) {
        let value = assignment(tokens);

        // TODO: The left always needs to be an l-value. If the left is an
        //  r-value, then it needs to be converted for assignment to work.

        if let Expr::Variable(token) = &expr {
            return Expr::Assign(token.clone(), Box::new(value.clone()));
        } else {
            panic!("Invalid assignment target");
        }
    }

    expr
}

fn or(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = and(tokens);
    let mut token = peek_token(tokens);

    while consume_match(tokens, &[TokenType::Or]) {
        let right = and(tokens);
        expr = Expr::Logical(Box::new(expr.clone()), token.clone(), Box::new(right.clone()));
        token = peek_token(tokens);
    }

    expr
}

fn and(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = equality(tokens);
    let mut token = peek_token(tokens);

    while consume_match(tokens, &[TokenType::And]) {
        let right = equality(tokens);
        expr = Expr::Logical(Box::new(expr.clone()), token.clone(), Box::new(right.clone()));
        token = peek_token(tokens);
    }

    expr
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
        call(tokens)
    }
}

fn call(tokens: &mut Vec<Token>) -> Expr {
    let mut expr = primary(tokens);

    loop {
        if consume_match(tokens, &[TokenType::LeftParen]) {
            expr = finish_call(tokens, expr.clone());
        } else {
            break;
        }
    }

    expr
}

fn finish_call(tokens: &mut Vec<Token>, callee: Expr) -> Expr {
    let mut arguments = Vec::new();
    if peek_token(tokens).type_of != TokenType::RightParen {
        loop {
            arguments.push(Box::new(expression(tokens)));
            if !consume_match(tokens, &[TokenType::Comma]) {
                break;
            }
        }
    }

    if arguments.len() >= 8 {
        panic!("Cannot have more than 8 arguments per function");
    }

    let token = peek_token(tokens);
    try_consume(tokens, &[TokenType::RightParen], "Expect ')' after arguments");
    Expr::Call(Box::new(callee), token, arguments)
}

fn primary(tokens: &mut Vec<Token>) -> Expr {
    let token = pop_token(tokens);
    match token.type_of {
        TokenType::Number => Expr::Literal(token.literal),
        TokenType::String => Expr::Literal(token.literal),
        TokenType::False => Expr::Literal(Object::Bool(false)),
        TokenType::True => Expr::Literal(Object::Bool(true)),
        TokenType::Nil => Expr::Literal(token.literal),
        TokenType::LeftParen => {
            let expr = Expr::Grouping(Box::new(expression(tokens)));
            if !consume_until_found(tokens, &[TokenType::RightParen]) {
                panic!("Couldn't find ')' for Grouping");
            }
            expr
        }
        TokenType::Identifier => Expr::Variable(token),
        TokenType::Eof => Expr::Empty,
        _ => {
            eprintln!("Couldn't Match! {:?}", token.type_of);
            Expr::Empty
        }
    }
}
