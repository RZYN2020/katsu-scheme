extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;
pub mod interpreter;
pub mod builtin;

use std::rc::Rc;
use std::cell::RefCell;
use parser::*;
use interpreter::*;

// Scheme-Rust Interface

pub struct Katsu {
    env: Rc<RefCell<Env>>,
}

impl Katsu {
    pub fn new() -> Self {
        let env = Env::get_initialized_env();
        Self { env }
    }
    pub fn eval(&self, program: &str) -> Vec<Option<Rc<Value>>> {
        let ast = parse(&program, "init");
        let mut res = Vec::new();
        for top in ast.tops {
            res.push(interpreter::eval(top, &self.env).unwrap());
        }
        res
    }
    pub fn eval_to_str(&self, program: &str) -> Vec<Option<String>> {
        let res = self.eval(program);
        res.into_iter().map(|v| v.map(|v| v.to_string())).collect()
    }
}
