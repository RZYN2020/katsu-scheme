use crate::parser::*;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Env<'a> {
    parent: Option<&'a Env<'a>>,
    values: HashMap<String, Rc<Exp>>,
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
    fn resolve(&self, identifier: &str) -> Option<&Rc<Exp>> {
        self.values
            .get(identifier)
            .or_else(|| self.parent.and_then(|parent| parent.resolve(identifier)))
    }
    fn insert(&mut self, identifier: String, expression: Rc<Exp>) {
        self.values.insert(identifier, expression);
    }
}

#[allow(unused)]
pub fn eval(top: Top, env: &mut Env) -> Result<Option<Rc<Exp>>, String> {
    let res = match top {
        Top::DEC {
            identifier,
            expression,
        } => {
            let value = eval_expr(expression, env).ok_or("error")?;
            env.insert(identifier, value);
            None
        }
        Top::EXP { expression } => {
            eval_expr(expression, env)
        }
    };
    Ok(res)
}

fn eval_expr(expr: Rc<Exp>, env: &mut Env) -> Option<Rc<Exp>> {
    match &*expr {
        Exp::IDENTIFIER(identifier) => env.resolve(&identifier).cloned(),
        Exp::LITERIAL(_) | Exp::LAMBDA { .. } => Some(expr),
        Exp::CALL { operator, operands } => {
            let operator = eval_expr(operator.clone(), env)?;
            let operands = operands
                .into_iter()
                .map(|operand| eval_expr(operand.clone(), env))
                .collect::<Option<Vec<_>>>()?;
            apply(operator, operands, env)
        }
        Exp::COND {
            test,
            consequent,
            alternative,
        } => {
            let test = eval_expr(test.clone(), env)?;
            if let Exp::LITERIAL(Datum::BOOLEAN(true)) = *test {
                eval_expr(consequent.clone(), env)
            } else {
                if let Some(alternative) = alternative {
                    eval_expr(alternative.clone(), env)
                } else {
                    None
                }
            }
        }
    }
}

#[allow(unused)]
fn apply(operator: Rc<Exp>, operands: Vec<Rc<Exp>>, env: &mut Env) -> Option<Rc<Exp>> {
    match &*operator {
        Exp::LAMBDA {
            parameters,
            definitions,
            body,
        } => {
            if parameters.len() != operands.len() {
                println!("wrong number of arguments!");
                return None;
            }
            let mut env = Env::with_parent(env);
            for (parameter, operand) in parameters.iter().zip(operands) {
                env.insert(parameter.clone(), operand);
            }
            for (identifier, expression) in definitions {
                let value = eval_expr(expression.clone(), &mut env)?;
                env.insert(identifier.clone(), value);
            }
            eval_expr(body.clone(), &mut env)
        }
        _ => {
            println!("call on non-function!");
            None
        }
    }
}
