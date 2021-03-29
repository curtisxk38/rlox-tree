use std::env;
use std::fs;
use std::io;
use std::process;

use error::LoxError;
use io::Write;
use resolver::Resolver;
use scan::Scanner;
use tree_walker::TreeWalker;

mod scan;
mod tokens;
mod parse;
mod error;
mod ast;
mod tree_walker;
mod callable;
mod output;
mod native;
mod resolver;
mod class;

struct Interpreter {
    had_error: bool,
    tree_walker: tree_walker::TreeWalker,
    scanner: scan::Scanner,
}

impl Interpreter {

    pub fn new() -> Interpreter {
        return Interpreter { had_error: false, tree_walker: TreeWalker::new(), scanner: Scanner::new() };
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
        match self.scanner.scan(&input) {
            Ok(_) => {
                let mut parser = parse::Parser::new();
                let parsed = parser.parse(&self.scanner.tokens);
                match parsed {
                    Ok(statements) => {

                        let mut resolver = Resolver::new(&mut self.tree_walker);
                        resolver.resolve(&statements);
                        if resolver.errors.len() > 0 {
                            for error in resolver.errors {
                                self.error(error);
                            }
                            return;
                        }
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
            use output::Recorder;
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
            let mut interpreter = TreeWalker::new_from_outputter(outputter);
            
            // standard interpreter run
            let mut scanner = scan::Scanner::new();
            scanner.scan(&contents).expect("scan error");
            let mut parser = parse::Parser::new();
            let statements = parser.parse(&scanner.tokens).expect("parse errors");
            let mut resolver = Resolver::new(&mut interpreter);
            resolver.resolve(&statements);
            if resolver.errors.len() > 0 {
               panic!("error resolving")
            }
            
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
    function_value: "tests/function_value.lox",
    recursive_fib: "tests/recursive_fib.lox",
    closure: "tests/closure.lox",
    print_clock: "tests/print_clock.lox",
    scoping: "tests/scoping.lox",
    class_creation: "tests/class_test.lox",
    class_fields: "tests/class_fields.lox",
    basic_methods: "tests/basic_methods.lox",
);