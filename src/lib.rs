extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod builtin;
pub mod interpreter;
pub mod parser;

use interpreter::*;
use parser::*;
use std::cell::RefCell;
use std::rc::Rc;

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
        let ast = parse(&program);
        let mut res = Vec::new();
        for top in ast.tops {
            res.push(interpreter::eval(top, &self.env).unwrap());
        }
        res
    }
    pub fn eval_to_str(&self, program: &str) -> String {
        let res = self.eval(program);
        res.into_iter()
            .map(|v| match v {
                Some(v) => format!("{}", v),
                None => String::from("None"),
            })
            .collect()
    }
    pub fn eval_all(&self, program: &str) -> ResIterator {
        ResIterator::new(parse(&program), self)
    }
}

pub struct ResIterator<'a> {
    katsu: &'a Katsu,
    tops: std::vec::IntoIter<Top>,
}

impl<'a> ResIterator<'a> {
    fn new(ast: Ast, katsu: &'a Katsu) -> Self {
        Self {
            katsu,
            tops: ast.tops.into_iter(),
        }
    }
}

impl<'a> Iterator for ResIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let res = interpreter::eval(self.tops.next().unwrap(), &self.katsu.env).unwrap();
        Some(match res {
            Some(v) => format!("{}", v),
            None => String::from("None"),
        })
    }
}
