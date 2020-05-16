use std::env;
use std::fs;
use std::io;

fn run_file(filename: &String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    run(&contents);
}

fn run_prompt() {
    println!("Hello repl");
    let mut input = String::new();
    
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => { 
                run(&input);
                input = String::new();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn run(input: &String) {
    println!("{}", input);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("usage: rlox [script]");
    }
    else if args.len() == 2 {
        run_file(&args[1]);
    }
    else {
        run_prompt();
    }
}
