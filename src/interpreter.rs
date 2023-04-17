use crate::parser::*;
use std::collections::HashMap;

pub struct Env<'a> {
    parent: Option<&'a Env<'a>>,
    values: HashMap<String, Exp>,
}

impl<'a> Env<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }
    pub fn with_parent(parent: &'a Env<'a>) -> Self {
        Self {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }
    fn resolve(&self, identifier: &str) -> Option<&Exp> {
        Some(self.values.get(identifier).unwrap())
    }
    fn insert(&mut self, identifier: String, expression: Exp) {
        self.values.insert(identifier, expression);
    }
}

#[allow(unused)]
pub fn eval(top: Top, env: &mut Env) -> Result<(), String>{
    match top {
        Top::DEC { identifier, expression } => {
            let value = eval_expr(expression, env).ok_or("error")?;
            env.insert(identifier, *value);
        }
        Top::EXP { expression } => {
            eval_expr(expression, env);
        }
    }
    Ok(())
}

fn eval_expr(expr: Exp, env: &mut Env) -> Option<Box<Exp>>{
    match expr {
        Exp::IDENTIFIER(identifier) => {
            // env.resolve(&identifier).cloned()
            None
        }
        Exp::LITERIAL(literal) => {
            // Some(Exp::LITERIAL(literal))
            None
        }
        Exp::CALL { operator, operands } => {
            // match env.resolve(&identifier) {
            //     Some(Exp::LAMBDA { params, body }) => {
            //         let mut new_env = Env::with_parent(env);
            //         for (param, arg) in params.iter().zip(args) {
            //             new_env.insert(param.clone(), eval_expr(arg, env)?);
            //         }
            //         eval_expr(*body, &mut new_env)
            //     }
            //     _ => None,
            // }
            None
        }
        Exp::LAMBDA { parameters, definitions, body } => {
            None
        }
        Exp::COND { test, consequent, alternative } => {
            None
        }
    }
}

#[allow(unused)]
fn apply() {
    unimplemented!();
}