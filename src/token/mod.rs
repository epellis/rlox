pub mod token_type;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Token {
    pub type_of: token_type::TokenType,
    lexeme: String,
    pub literal: Object,
    line: u32,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Object {
    None,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

impl Token {
    pub fn new(type_of: token_type::TokenType, lexeme: &str, line: u32) -> Token {
        let literal = Object::None;
        let lexeme = lexeme.to_string();
        Token { type_of, lexeme, literal, line }
    }

    pub fn new_string(literal: &str, line: u32) -> Token {
        let type_of = token_type::TokenType::STRING;
        let literal = Object::String(literal.to_string());
        let lexeme = String::new();
        Token { type_of, literal, lexeme, line }
    }

    pub fn new_number(literal: f64, line: u32) -> Token {
        let type_of = token_type::TokenType::NUMBER;
        let literal = Object::Number(literal);
        let lexeme = String::new();
        Token { type_of, literal, lexeme, line }
    }

    pub fn new_identifier(lexeme: &str, line: u32) -> Token {
        let type_of = token_type::TokenType::IDENTIFIER;
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
