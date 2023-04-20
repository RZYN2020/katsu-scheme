extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser; 
mod interpreter;
mod builtin;

use clap::Parser;
use interpreter::{Env, Value};
use std::io::Write;
use std::io::{stdin, stdout};
use std::rc::Rc;
use std::cell::RefCell;

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
    let env = interpreter::Env::get_initialized_env();

    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        if line.trim().eq("quit") || line.eq("q") {
            break;
        } else if line.is_empty() {
            continue;
        }
        let res = run(&line, &env);
        if let Some(res) = res {
            println!("{}", res);
        } else {
            println!("No result");
        }
    }
}

fn run(program: &str, env: &Rc<RefCell<Env>>) -> Option<Rc<Value>> {
    let ast = parser::parse(&program, "init");
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
        let env = interpreter::Env::get_initialized_env();
        run(&program, &env);
    } else if opt.interactive {
        repl();
    } else {
        repl();
        println!("No input file or interactive mode");
    }
}

#[cfg(test)]
mod test {
}
