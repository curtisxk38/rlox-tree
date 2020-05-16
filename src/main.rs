use std::env;
use std::fs;
use std::io;

mod scan;

struct Interpreter {
    had_error: bool
}

impl Interpreter {

    pub fn new() -> Interpreter {
        return Interpreter { had_error: false};
    }

    fn run_file(&self, filename: &String) {
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        self.run(&contents);
    }

    fn run_prompt(&self) {
        println!("Hello repl");
        let mut input = String::new();
        
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => { 
                    self.run(&input);
                    input = String::new();
                }
                Err(error) => println!("error: {}", error),
            }
        }
    }

    fn run(&self, input: &String) {
        println!("{}", input);
    }

    fn error(&mut self, line: i32, error_where: &String, message: &String) {
        println!("[line {} ] Error{}: {}", line, error_where, message);
        self.had_error = true;
    }
    
}                                                 

fn main() {
    let args: Vec<String> = env::args().collect();
    let lox = Interpreter::new();
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
