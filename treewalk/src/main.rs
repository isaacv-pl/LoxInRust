use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::ExitCode;
use std::str;

mod token_type;
mod tokens;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    pub fn report(&mut self, line: i32, wher: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, wher, message);
        self.had_error = true;
    }

    pub fn error(&mut self, line: i32, message: &str) {
        Self::report(self, line, "", message);
    }

    pub fn run(source: &str) {}

    pub fn run_prompt(self) {
        let mut line = String::new();

        while (true) {
            println!("> ");
            match io::stdin().read_line(&mut line) {
                Ok(_) => {
                    if (line == Null) {
                        break;
                    }
                    Self::run(&line);
                    self.had_error = false;
                }
                Err(error) => println!("error {error}"),
            }
        }
    }

    pub fn run_file(&self, path: &str) {
        let mut file = match File::open(&path) {
            Ok(input) => input,
            Err(e) => return (),
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(input) => input,
            Err(e) => return (),
        };
        Self::run(&s);

        if (self.had_error) {
            ExitCode::from(101);
        }
    }

    fn main() {
        let args: Vec<String> = env::args().collect();
        if (args.len() > 1) {
            println!("Usage: Rloxtree [script]");
            ExitCode::from(101);
        } else if (args.len() == 1) {
            Self::run_file(self, &args[0]);
        } else {
            Self::run_prompt(self);
        }
    }
}
