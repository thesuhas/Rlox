use std::{env};
use std::process::ExitCode;
use std::fs;
use std::io::{stdin, stdout, Write};

fn run_file(filepath: String) {
    let file = fs::read_to_string(filepath).expect("Unable to read file!");
    println!("File Contents: {}", file);
}

fn run_prompt() {
    let mut input: String = String::new();

    loop {
        input.clear();
        print!("> ");
        stdout().flush().expect("Unable to flush to stdout!");
        stdin().read_line(&mut input).expect("Unable to read line!");
        if input == "\n" {
            break;
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        ExitCode::from(64)
    } else if args.len() == 2 {
        run_file(args[1].clone());
        ExitCode::SUCCESS
    } else {
        run_prompt();
        ExitCode::SUCCESS
    }
}
