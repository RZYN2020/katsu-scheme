use crate::parser::*;
use std::collections::HashMap;

pub struct Env {
    map: HashMap<String, Exp>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn resolve(&self, identifier: &str) -> Option<&Exp> {
        Some(self.map.get(identifier).unwrap())
    }
    fn insert(&mut self, identifier: String, expression: Exp) {
        self.map.insert(identifier, expression);
    }
}

#[allow(unused)]
pub fn eval(top: Top, env: Env) -> Option<Env> {
    match top {
        Top::DEC { identifier, expression } => {
            // let mut env = Env { map: env.map };
            // let res = eval_expr(expression, env).unwrap();
            // env.insert(identifier, res);
            // Some(env)
        }
        Top::EXP { expression } => {
            eval_expr(expression, env);
            // Some(env)
        }
    }
    None
}

fn eval_expr(expr: Exp, env: Env) -> Option<Exp>{
    None
}

#[allow(unused)]
fn apply() {
    unimplemented!();
}