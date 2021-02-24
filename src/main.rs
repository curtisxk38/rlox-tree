use std::env;
use std::fs;
use std::io;
use std::process;

use error::LoxError;
use io::Write;
use tree_walker::TreeWalker;

mod scan;
mod tokens;
mod parse;
mod error;
mod ast;
mod tree_walker;
mod callable;
mod output;

struct Interpreter {
    had_error: bool,
    tree_walker: tree_walker::TreeWalker,
}

impl  Interpreter {

    pub fn new() -> Interpreter {
        return Interpreter { had_error: false, tree_walker: TreeWalker::new() };
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

    fn run<'b>(&mut self, input: &'b String) {
        let mut scanner = scan::Scanner::new(&input);
        match scanner.scan() {
            Ok(_) => {
                let mut parser = parse::Parser::new();
                let parsed = parser.parse(&scanner.tokens);
                match parsed {
                    Ok(statements) => {
                        for statement in statements {
                            let interpreted = self.tree_walker.visit_statement(&statement);
                            match interpreted {
                                Ok(_) => {},
                                Err(e) => {
                                    self.error(e);
                                    break;
                                }
                            }
                        }
                        
                    },
                    Err(_) => {
                        for error in parser.errors {
                            self.error(error);
                        }
                    }
                }

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


macro_rules! program_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            use std::rc::Rc;
            use output::Recorder;
            use tree_walker::Environment;
            use std::cell::RefCell;
            // read file
            let contents = fs::read_to_string($value)
                    .expect("Something went wrong reading the file");
            let lines:Vec<&str> = contents.split("\n").collect();
            let mut output = Vec::new();
            // parse expected output from comments at the start of .lox file
            for line in lines {
                if line.len() < 3 || &line[0..2] != "//" {
                    break;
                }
                output.push(String::from(&line[2..]))
            }
            // set up interpreter for running the test program
            let outputter = Recorder{outputted: Vec::new()};
            let mut interpreter = TreeWalker{ outputter, environment: Rc::new(RefCell::new(Environment::new())) };

            // standard interpreter run
            let mut scanner = scan::Scanner::new(&contents);
            scanner.scan().expect("scan error");
            let mut parser = parse::Parser::new();
            let statements = parser.parse(&scanner.tokens).expect("parse errors");
            
            for statement in statements {
                let interpreted = interpreter.visit_statement(&statement);
                interpreted.expect("runtime error");
            }

            assert_eq!(output, interpreter.outputter.outputted);
        }
    )*
    }
}

program_tests!(
    basic_operation: "tests/basic_operation.lox",
    basic_function: "tests/basic_function.lox",
    less_fun: "tests/less_fun.lox",
    fun_in_for: "tests/fun_in_for.lox",
    recursive_fib: "tests/recursive_fib.lox",
    closure: "tests/closure.lox",
);