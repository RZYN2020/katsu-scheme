extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;        // parse input string to S-expresion
mod interpreter;   // interpreter

use std::io::{stdin, stdout};
use std::io::Write;
use clap::Parser;
use interpreter::Env;

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
    let mut env = Env::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        if line.trim() == "quit" {
            break;
        } else if line.is_empty() {
            continue;
        }
        run(&line, &mut env);
    }
}

fn run(program: &str, env: &mut Env) {
    let ast = parser::parse(&program, "init").unwrap();
    for top in ast.tops {
       interpreter::eval(top, env).unwrap();
    }
}


fn main() {
    let opt = Opt::parse();

    if let Some(file) = opt.file {
        let program = std::fs::read_to_string(file).unwrap();
        let mut env = Env::new();
        run(&program, &mut env);
    } else if opt.interactive {
        repl();
    } else {
        println!("No input file or interactive mode");
    }
}


mod test {
    #[test]
    fn test0() {
    }
}
