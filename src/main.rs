mod expr;
pub mod scanner;
mod token;
mod token_type;

use scanner::Scanner;

use crate::token::Token;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process::ExitCode;

#[derive(Debug)]
struct Rlox {
    had_error: bool,
}

impl Default for Rlox {
    fn default() -> Rlox {
        Rlox { had_error: false }
    }
}

impl Rlox {
    fn run_file(&mut self, filepath: String) {
        let file = fs::read_to_string(filepath).expect("Unable to read file!");
        self.run(file);
        if self.had_error {
            let _ = ExitCode::from(65);
        }
    }

    fn run_prompt(&mut self) {
        let mut input: String = String::new();

        loop {
            input.clear();
            print!("> ");
            stdout().flush().expect("Unable to flush to stdout!");
            match stdin().read_line(&mut input) {
                Ok(0) => break,
                Ok(_) => self.run(input.clone()),
                Err(e) => panic!("{}", e),
            }
            self.had_error = false;
        }
    }

    fn report(&mut self, line: i32, loc: String, message: &str) {
        eprintln!("[line {}] {}: {}", line, loc, message);
        self.had_error = true;
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, "".to_string(), message);
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source, self);

        let tokens: Vec<Token> = scanner.scan_tokens();

        for token in tokens.iter() {
            println!("{}", token.to_string());
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut rlox: Rlox = Rlox {
        ..Default::default()
    };
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        ExitCode::from(64)
    } else if args.len() == 2 {
        rlox.run_file(args[1].clone());
        ExitCode::SUCCESS
    } else {
        rlox.run_prompt();
        ExitCode::SUCCESS
    }
}
