use std::env;
use std::fs;
use std::io;
use std::process;

use error::LoxError;
use io::{Write};

mod scan;
mod tokens;
mod parse;
mod error;

struct Interpreter {
    had_error: bool
}

impl Interpreter {

    pub fn new() -> Interpreter {
        return Interpreter { had_error: false};
    }

    fn run_file(&mut self, filename: &String) {
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        self.run(&contents);
        if self.had_error {
            process::exit(1);
        }
    }

    fn run_prompt(&mut self) {
        println!("Welcome to Lox REPL!");
        let mut input = String::new();
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().ok().expect("Couldn't flush stdout");
            input.clear();
            let read = stdin.read_line(&mut input);
            match read {
                Ok(chars_read) => { 
                    if chars_read == 0 {
                        break;
                    }
                    self.run(&input);
                    // last run had error, but new run may be fine
                    self.had_error = false;
                }
                Err(error) => println!("error: {}", error),
            }
        }
    }

    fn run(&mut self, input: &String) {
        let mut scanner = scan::Scanner::new(input);
        let result = scanner.scan();
        match result {
            Ok(_) => {
                println!("{:?}", scanner.tokens)
            },
            Err(e) => self.error(e)
        }
    }

    fn error(&mut self, error: LoxError) {
        println!("{:?}", error);
        self.had_error = true;
    }
    
}                                                 

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Interpreter::new();
    if args.len() > 2 {
        panic!("usage: rlox [script]");
    }
    else if args.len() == 2 {
        lox.run_file(&args[1]);
    }
    else {
        lox.run_prompt();
    }
}
