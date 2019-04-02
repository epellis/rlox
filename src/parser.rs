use crate::token::{Token, Object};
use crate::expression::*;
use crate::token::token_type::TokenType;
use std::cell::RefCell;
use std::rc::Rc;

struct Parser {
    tokens: Vec<Token>,
    previous: Option<Token>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Parser {
        let mut tokens = tokens.to_vec();
        tokens.reverse();
        Parser { tokens, previous: None }
    }

    pub fn match_type(&mut self, types: &[TokenType]) -> bool {
        if let Some(tok) = self.peek() {
            if types.contains(&tok.type_of) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) {
        self.previous = self.tokens.pop();
    }

    fn consume_until(&mut self, type_of: TokenType) {

    }

    fn check(&self, type_of: TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        match self.peek() {
            Some(tok) if tok.type_of == type_of => true,
            _ => false,
        }
    }

    fn is_end(&self) -> bool {
        match self.peek() {
            Some(tok) => tok.type_of == TokenType::EOF,
            None => true,
        }
    }

    fn peek(&self) -> Option<&Token> {
        match self.tokens.is_empty() {
            true => None,
            false => Some(&self.tokens[0]),
        }
    }

    fn previous(&self) -> Token {
        match &self.previous {
            Some(tok) => tok.clone(),
            None => panic!("Previous called on a full list"),
        }
    }
}

fn expression(parser: &mut Parser) -> Expr {
    equality(parser)
}

fn equality(parser: &mut Parser) -> Expr {
    let mut expr = comparison(parser);
    let mut operator = parser.previous();
    let mut right: Expr = Rc::new(RefCell::new(Literal { value: Object::None }));

    while parser.match_type(&[TokenType::EqualEqual, TokenType::BangEqual]) {
        operator = parser.previous();
        right = equality(parser);
    }

    let expr = Binary { left: expr, operator, right };
    Rc::new(RefCell::new(expr))
}

fn comparison(parser: &mut Parser) -> Expr {
    let types = [TokenType::GREATER, TokenType::GreaterEqual, TokenType::LESS, TokenType::LessEqual];
    let expr = addition(parser);
    let mut operator = parser.previous();
    let mut right: Expr = Rc::new(RefCell::new(Literal { value: Object::None }));

    while parser.match_type(&types) {
        operator = parser.previous();
        right = addition(parser)
    }

    let expr = Binary { left: expr, operator, right };
    Rc::new(RefCell::new(expr))
}

fn addition(parser: &mut Parser) -> Expr {
    let expr = multiplication(parser);
    let mut operator = parser.previous();
    let mut right: Expr = Rc::new(RefCell::new(Literal { value: Object::None }));

    while parser.match_type(&[TokenType::MINUS, TokenType::PLUS]) {
        operator = parser.previous();
        right = multiplication(parser)
    }

    let expr = Binary { left: expr, operator, right };
    Rc::new(RefCell::new(expr))
}

fn multiplication(parser: &mut Parser) -> Expr {
    let expr = unary(parser);
    let mut operator = parser.previous();
    let mut right: Expr = Rc::new(RefCell::new(Literal { value: Object::None }));

    while parser.match_type(&[TokenType::SLASH, TokenType::STAR]) {
        operator = parser.previous();
        right = unary(parser)
    }

    let expr = Binary { left: expr, operator, right };
    Rc::new(RefCell::new(expr))
}

fn unary(parser: &mut Parser) -> Expr {
    if parser.match_type(&[TokenType::BANG, TokenType::MINUS]) {
        let operator = parser.previous();
        let right = unary(parser);
        let expr = Unary { operator, right };
        return Rc::new(RefCell::new(expr));
    }

    primary(parser)
}

fn primary(parser: &mut Parser) -> Expr {
    if parser.match_type(&[TokenType::FALSE]) {
        let expr = Literal { value: Object::False };
        return Rc::new(RefCell::new(expr));
    } else if parser.match_type(&[TokenType::TRUE]) {
        let expr = Literal { value: Object::True };
        return Rc::new(RefCell::new(expr));
    } else if parser.match_type(&[TokenType::NIL]) {
        let expr = Literal { value: Object::Null };
        return Rc::new(RefCell::new(expr));
    }

    if parser.match_type(&[TokenType::NUMBER, TokenType::STRING]) {
        let tok = parser.previous();
        let expr = Literal { value: tok.literal };
        return Rc::new(RefCell::new(expr));
    }

    if parser.match_type(&[TokenType::LeftParen]) {
        let expr = expression(parser);
        parser.consume_until(TokenType::RightParen);
//        return Rc::new(RefCell::new(expr));
        return expr;
    }

    panic!("Could not match any types!");
}
