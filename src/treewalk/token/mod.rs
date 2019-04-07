pub mod token_type;

use crate::treewalk::statement::Stmt;
use crate::treewalk::environment::Environment;

#[derive(PartialOrd, PartialEq, Clone)]
pub struct Token {
    pub type_of: token_type::TokenType,
    pub lexeme: String,
    pub literal: Object,
    line: u32,
}

#[derive(PartialOrd, PartialEq, Clone)]
pub enum Object {
    None,
    String(String),
    Number(f64),
    Bool(bool),
    Function(Vec<Token>, Vec<Box<Stmt>>, Environment),
    Nil,
}

impl Token {
    pub fn new(type_of: token_type::TokenType, lexeme: &str, line: u32) -> Token {
        let literal = Object::None;
        let lexeme = lexeme.to_string();
        Token { type_of, lexeme, literal, line }
    }

    pub fn new_string(literal: &str, line: u32) -> Token {
        let type_of = token_type::TokenType::String;
        let literal = Object::String(literal.to_string());
        let lexeme = String::new();
        Token { type_of, literal, lexeme, line }
    }

    pub fn new_number(literal: f64, line: u32) -> Token {
        let type_of = token_type::TokenType::Number;
        let literal = Object::Number(literal);
        let lexeme = String::new();
        Token { type_of, literal, lexeme, line }
    }

    pub fn new_identifier(lexeme: &str, line: u32) -> Token {
        let type_of = token_type::TokenType::Identifier;
        let literal = Object::None;
        let lexeme = lexeme.to_string();
        Token { type_of, lexeme, literal, line }
    }

    pub fn new_keyword(type_of: token_type::TokenType, line: u32) -> Token {
        let literal = Object::None;
        let lexeme = String::new();
        Token { type_of, lexeme, literal, line }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.literal {
            Object::None if self.lexeme != "" => write!(f, "{}", self.lexeme),
            Object::None => write!(f, "Token::{:?}", self.type_of),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Bool(true) => write!(f, "true"),
            Object::Bool(false) => write!(f, "false"),
            Object::Nil => write!(f, "nil"),
            Object::Function(parameters, block, closure) => {
                write!(f, "Function: {:?} {:?} {:?}", parameters, block, closure)
            },
        }
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Object::None => write!(f, "None"),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Bool(true) => write!(f, "true"),
            Object::Bool(false) => write!(f, "false"),
            Object::Nil => write!(f, "nil"),
            Object::Function(parameters, block, closure) => {
                write!(f, "Function: {:?} {:?} {:?}", parameters, block, closure)
            },
        }
    }
}
