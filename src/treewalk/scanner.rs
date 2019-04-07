use crate::treewalk::token::{Token, token_type::TokenType};
use std::collections::HashMap;

pub struct Scanner {
    source: String,
}

use lazy_static;

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        let source = source.to_string();
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut line: u32 = 1;

        let mut source: Vec<char> = self.source.chars().collect();
        source.reverse();

        while let Some(c) = source.pop() {
            if let Some(tok) = Scanner::scan_token(c, &mut source, &mut line) {
                tokens.push(tok);
            }
        }

        tokens.push(Token::new(TokenType::Eof, "", line));

        tokens
    }

    fn scan_token(c: char, source: &mut Vec<char>, line: &mut u32) -> Option<Token> {
        let next_c = match source.pop() {
            Some(c) => {
                source.push(c);
                c
            }
            None => ' ',
        };

        let token_type = match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            '!' if next_c == '=' => Some(TokenType::BangEqual),
            '!' => Some(TokenType::Bang),
            '=' if next_c == '=' => Some(TokenType::EqualEqual),
            '=' => Some(TokenType::Equal),
            '<' if next_c == '=' => Some(TokenType::LessEqual),
            '<' => Some(TokenType::Less),
            '>' if next_c == '=' => Some(TokenType::GreaterEqual),
            '>' => Some(TokenType::Greater),

            // Inline and Block Comment
            '/' if next_c == '/' => {
                eat_line(source);
                return None;
            }
            '/' if next_c == '*' => {
                eat_block_comment(source);
                return None;
            }

            '/' => Some(TokenType::Slash),

            // Whitespace
            ' ' => None,
            '\r' => None,
            '\t' => None,
            '\n' => {
                *line += 1;
                None
            }

            // String Literals
            '"' => {
                let literal = eat_string(source);
                return Some(Token::new_string(&literal, *line));
            }

            // Number Literals
            '0'...'9' => {
                source.push(c);
                let literal = eat_number(source);
                return Some(Token::new_number(literal, *line));
            }

            // Alphabetic words
            'a'...'z' | 'A'...'Z' => {
                source.push(c);
                let lexeme = eat_identifier(source);

                match KEYWORDS.get(&lexeme) {
                    Some(type_of) => {
                        let type_of = *type_of;
                        return Some(Token::new_keyword(type_of, *line));
                    }
                    None => {
                        return Some(Token::new_identifier(&lexeme, *line));
                    }
                }
            }

            // Couldn't Match
            _ => {
                eprintln!("Unexpected character {}", c);
                return None;
            }
        };

        match token_type {
            Some(TokenType::BangEqual) => { source.pop(); }
            Some(TokenType::EqualEqual) => { source.pop(); }
            Some(TokenType::LessEqual) => { source.pop(); }
            Some(TokenType::GreaterEqual) => { source.pop(); }
            _ => {}
        }

        match token_type {
            Some(type_of) => Some(Token::new(type_of, "", *line)),
            None => None,
        }
    }
}

// Consumes the vec until a new line is found
fn eat_line(source: &mut Vec<char>) {
    while let Some(c) = source.pop() {
        if c == '\n' {
            return;
        }
    }
}

fn eat_block_comment(source: &mut Vec<char>) {
    let mut last_seen = '/';

    while let Some(c) = source.pop() {
        println!("{} {}", last_seen, c);
        if c == '/' && last_seen == '*' {
            return;
        }
        last_seen = c;
    }
}


// Scan until another quotation mark is found or end of stack
fn eat_string(source: &mut Vec<char>) -> String {
    let mut literal = String::new();

    while let Some(c) = source.pop() {
        if c == '"' {
            return literal;
        } else {
            literal.push(c);
        }
    }

    literal
}

// Consume until a non-integer character is found
fn eat_number(source: &mut Vec<char>) -> f64 {
    let mut literal = String::new();
    let mut dot_count = 0;

    while let Some(c) = source.pop() {
        if c.is_digit(10) {
            literal.push(c);
        } else if c == '.' && dot_count == 0 {
            dot_count += 1;
            literal.push(c);
        } else {
            source.push(c);
            break;
        }
    }

    literal.parse().unwrap()
}

// Consume until no more alphanumeric chars are in stack
fn eat_identifier(source: &mut Vec<char>) -> String {
    let mut literal = String::new();

    while let Some(c) = source.pop() {
        if c.is_alphanumeric() {
            literal.push(c);
        } else {
            source.push(c);
            break;
        }
    }

    literal
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".to_string(), TokenType::And);
        m.insert("break".to_string(), TokenType::Break);
        m.insert("class".to_string(), TokenType::Class);
        m.insert("else".to_string(), TokenType::Else);
        m.insert("for".to_string(), TokenType::For);
        m.insert("fun".to_string(), TokenType::Fun);
        m.insert("if".to_string(), TokenType::If);
        m.insert("nil".to_string(), TokenType::Nil);
        m.insert("or".to_string(), TokenType::Or);
        m.insert("print".to_string(), TokenType::Print);
        m.insert("return".to_string(), TokenType::Return);
        m.insert("super".to_string(), TokenType::Super);
        m.insert("this".to_string(), TokenType::This);
        m.insert("true".to_string(), TokenType::True);
        m.insert("false".to_string(), TokenType::False);
        m.insert("var".to_string(), TokenType::Var);
        m.insert("while".to_string(), TokenType::While);
        m
    };
}
