extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;        // parse input string to S-expresion
mod interpreter;   // interpreter

use std::io::{stdin, stdout};
use std::io::Write;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(version)]
struct Opt {
    #[clap(short = 'f', long = "file", help = "input file")]
    file: Option<String>,

    #[clap(short = 'i', long = "interactive", help = "interactive mode")]
    interactive: bool,
}

fn repl() {
    let mut line = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        run(&line);
    }
}

fn run(program: &str) {
    let sexpr = parser::parse(&program, "init").unwrap();
    interpreter::run(sexpr);
}

fn main() {
    let opt = Opt::parse();

    if let Some(file) = opt.file {
        let program = std::fs::read_to_string(file).unwrap();
        run(&program);
    } else if opt.interactive {
        repl();
    } else {
        repl();
        println!("No input file or interactive mode");
    }
}
