pub mod token_type;

#[derive(Debug)]
pub struct Token {
    type_of: token_type::TokenType,
    lexeme: String,
    literal: Option<String>,
    line: u32,
}

impl Token {
    pub fn new(type_of: token_type::TokenType, lexeme: &str, line: u32) -> Token {
        let literal = None;
        let lexeme = lexeme.to_string();
        Token { type_of, lexeme, literal, line }
    }

    pub fn new_literal(type_of: token_type::TokenType, lexeme: &str, literal: &str, line: u32) -> Token {
        let literal = Some(literal.to_string());
        let lexeme = lexeme.to_string();
        Token { type_of, lexeme, literal, line }
    }
}
