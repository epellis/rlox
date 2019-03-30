use std::env;
use std::io;
use std::io::Write;

mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => eprintln!("Usage: rlox [source]"),
    }
}

fn run_prompt() {
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

fn run_file(path: &str) {}

fn run(source: &str) -> Result<(), &'static str> {
    let tokens: Vec<String> = vec![];

    for token in tokens {
        println!("{}", token);
    }

//    Ok(())
    Err("Your program sucks ;)")
}

fn report(line: u32, source: &str, message: &str) {
    eprintln!("Line: {} Error: {} : {}", line, source.trim(), message);
}
