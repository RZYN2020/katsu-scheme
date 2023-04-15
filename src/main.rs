extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;        // parse input string to S-expresion
mod transformer;   // expand macro
mod irgenerator;   // generator ir for bril

use std::io::{stdin, stdout};
use bril_rs::output_program;
use std::io::Write;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(version)]
struct Opt {
    #[clap(short = 'f', long = "file", help = "input file")]
    file: String,

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
    let ir = irgenerator::generate(sexpr).unwrap();
    output_program(&ir);
}

fn main() {
    let opt = Opt::parse();

    if opt.interactive {
        repl();
    } else {
        let program = std::fs::read_to_string(opt.file).unwrap();
        run(&program);
    }
}
