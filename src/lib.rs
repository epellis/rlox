pub mod token;
pub mod scanner;

use std::io;
use std::io::Write;
use std::any::Any;

pub fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Couldn't Read");

        if let Err(why) = run(&line) {
            report(0, &line, why);
        }

        io::stdout().flush().unwrap();
    }
}

pub fn run_file(path: &str) {
    dbg!(path);
}

fn run(source: &str) -> Result<(), &'static str> {
    let scanner = scanner::Scanner::new(source.trim());
    let tokens = scanner.scan_tokens();

//    dbg!(source);

    for token in tokens {
        println!("Token: {:?}", token);
    }

    Ok(())
//    Err("Your program sucks ;)")
}

// TODO: Make into a macro?
fn report(line: u32, source: &str, message: &str) {
    eprintln!("Line: {} Error: {} : {}", line, source.trim(), message);
}
