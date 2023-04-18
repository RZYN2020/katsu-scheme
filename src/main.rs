extern crate pest;
#[macro_use]
extern crate pest_derive;

mod interpreter;
mod parser; // parse input string to S-expresion // interpreter

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
    let env = Rc::new(RefCell::new(Env::new()));
    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        if line.trim() == "quit" || line.trim() == "q" {
            break;
        } else if line.is_empty() {
            continue;
        }
        let res = run(&line, &env);
        if let Some(res) = res {
            println!("{:?}", res);
        }
    }
}

fn run(program: &str, env: &Rc<RefCell<Env>>) -> Option<Rc<Value>> {
    let ast = parser::parse(&program, "init").unwrap();
    let mut res = None;
    for top in ast.tops {
        res = interpreter::eval(top, &env).unwrap();
    }
    res
}

fn main() {
    let opt = Opt::parse();

    if let Some(file) = opt.file {
        let program = std::fs::read_to_string(file).unwrap();
        let env = Rc::new(RefCell::new(Env::new()));
        run(&program, &env);
    } else if opt.interactive {
        repl();
    } else {
        println!("No input file or interactive mode");
    }
}

mod test {

    enum Foo {
        A(Box<i32>),
        B(String),
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            match self {
                Foo::B(s) => {
                    println!("{}", s);
                }
                Foo::A(i) => {
                    println!("{}", i);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test0() {
        let a = Foo::A(Box::new(1));
        let b = Foo::B("hello".to_string());
        let c = Foo::A(Box::new(2));
    }
}
