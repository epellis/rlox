use crate::token::{Token, token_type::TokenType};
use std::sync::atomic::Ordering::SeqCst;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}


impl Scanner {
    pub fn new(source: &str) -> Scanner {
        let source = source.to_string();
        let tokens = Vec::new();
        Scanner { source, tokens }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let line = 1;
        let mut scan_error = false;

        let mut source: Vec<char> = self.source.chars().collect();
        source.reverse();


        while let Some(c) = source.pop() {
            match Scanner::scan_token(c, &mut source) {
                Ok(tok) => {
                    let tok = Token::new(tok, "", line);
                    tokens.push(tok);
                }
                Err(why) => {
                    scan_error = true;
                    println!("{} Char: {} Error: {}", line, c, why);
                }
            }
        }

        tokens.push(Token::new(TokenType::EOF, "", line));

        tokens
    }

    fn scan_token(c: char, source: &mut Vec<char>) -> Result<TokenType, &'static str> {
        let source_empty = source.is_empty();
        let next_c = match source.pop() {
            Some(c) => c,
            None => ' ',
        };

        let token = match c {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::RightBrace),
            '}' => Ok(TokenType::LeftBrace),
            ',' => Ok(TokenType::COMMA),
            '.' => Ok(TokenType::DOT),
            '-' => Ok(TokenType::MINUS),
            '+' => Ok(TokenType::PLUS),
            ';' => Ok(TokenType::SEMICOLON),
            '*' => Ok(TokenType::STAR),
            '!' if next_c == '=' => Ok(TokenType::BangEqual),
            '!' => Ok(TokenType::BANG),
            '=' if next_c == '=' => Ok(TokenType::EqualEqual),
            '=' => Ok(TokenType::EQUAL),
            '<' if next_c == '=' => Ok(TokenType::LessEqual),
            '<' => Ok(TokenType::LESS),
            '>' if next_c == '=' => Ok(TokenType::GreaterEqual),
            '>' => Ok(TokenType::GREATER),
            _ => Err("Unexpected character"),
        };

        match token {
            Ok(TokenType::BangEqual) => {}
            Ok(TokenType::EqualEqual) => {}
            Ok(TokenType::LessEqual) => {}
            Ok(TokenType::GreaterEqual) => {}
            _ if !source_empty => source.push(next_c),
            _ => {}
        }

        token
    }
}
