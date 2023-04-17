extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;        // parse input string to S-expresion
mod interpreter;   // interpreter

use std::io::{stdin, stdout};
use std::io::Write;
use clap::Parser;
use interpreter::Env;
use std::rc::Rc;
use parser::Exp;

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
        if line.trim() == "quit" || line.trim() == "q" {
            break;
        } else if line.is_empty() {
            continue;
        }
        let res = run(&line, &mut env);
        if let Some(res) = res {
            println!("{:?}", res);
        }
    }
}

fn run(program: &str, env: &mut Env) -> Option<Rc<Exp>>{
    let ast = parser::parse(&program, "init").unwrap();
    let mut res = None;
    for top in ast.tops {
       res = interpreter::eval(top, env).unwrap();
    }
    res
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
