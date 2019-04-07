mod token;
mod scanner;
mod expression;
mod statement;
mod parser;
mod interpreter;
mod environment;
mod resolver;

use std::io;
use std::io::Write;
use std::fs;

#[macro_use]
extern crate lazy_static;

pub fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Couldn't Read");

        if let Err(why) = run(&line, true) {
            report(0, &line, why);
        }

        io::stdout().flush().unwrap();
    }
}

pub fn run_file(path: &str) {
    let mut line = String::new();
    let contents = fs::read_to_string(path)
        .expect("Couldn't open file!");

    if let Err(why) = run(&contents, false) {
        report(0, &line, why);
    }
}

fn run(source: &str, is_repl: bool) -> Result<(), &'static str> {
    let scanner = scanner::Scanner::new(source.trim());
    let mut tokens = scanner.scan_tokens();
    println!("Scanned Tokens: {:?}", tokens.clone());

    let expressions = parser::parse(&mut tokens);
    println!("Parsed Expression: {:?}", expressions.clone());

    interpreter::interpret(expressions, is_repl)
}

// TODO: Make into a macro?
fn report(line: u32, source: &str, message: &str) {
    eprintln!("Line: {} Error: {} : {}", line, source.trim(), message);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::token::token_type::TokenType;
    use crate::token::Token;
    use crate::parser::*;
    use crate::interpreter::*;

    #[test]
    fn test_eof() {
        let input = "";
        let scanner = Scanner::new(input);
        let tok = scanner.scan_tokens();
        assert_eq!(tok[0], Token::new(TokenType::Eof, "", 1));
    }

    #[test]
    fn test_single_char() {
        let input = "=";
        let scanner = Scanner::new(input);
        let tok = scanner.scan_tokens();
        assert_eq!(tok[0], Token::new(TokenType::Equal, "", 1));
    }

    #[test]
    fn test_double_char() {
        let input = "==";
        let scanner = Scanner::new(input);
        let tok = scanner.scan_tokens();
        assert_eq!(tok[0], Token::new(TokenType::EqualEqual, "", 1));
    }

    #[test]
    fn test_multi_char() {
        let input = "= !=";
        let scanner = Scanner::new(input);
        let tok = scanner.scan_tokens();
        assert_eq!(tok[0], Token::new(TokenType::Equal, "", 1));
        assert_eq!(tok[1], Token::new(TokenType::BangEqual, "", 1));
    }

    #[test]
    fn test_number() {
        let input = "1";
        let scanner = Scanner::new(input);
        let tok = scanner.scan_tokens();
        assert_eq!(tok[0], Token::new_number(1.0, 1));
    }

    #[test]
    fn test_number_decimal() {
        let input = "1.23";
        let scanner = Scanner::new(input);
        let tok = scanner.scan_tokens();
        assert_eq!(tok[0], Token::new_number(1.23, 1));
    }

    #[test]
    fn test_string() {
        let input = "\"heya\"";
        let scanner = Scanner::new(input);
        let tok = scanner.scan_tokens();
        assert_eq!(tok[0], Token::new_string("heya", 1));
    }

    #[test]
    fn test_keyword() {
        let input = "and or while";
        let scanner = Scanner::new(input);
        let tok = scanner.scan_tokens();
        assert_eq!(tok[0], Token::new_keyword(TokenType::And, 1));
        assert_eq!(tok[1], Token::new_keyword(TokenType::Or, 1));
        assert_eq!(tok[2], Token::new_keyword(TokenType::While, 1));
    }
}
