use std::env;

use rlox;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => rlox::run_prompt(),
        2 => rlox::run_file(&args[1]),
        _ => eprintln!("Usage: rlox [source]"),
    }
}

