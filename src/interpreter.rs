use crate::parser::*;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    values: HashMap<String, Rc<Value>>,
}

#[derive(Debug)]
pub struct Clojure {
    proto: Rc<Lambda>,
    env: Rc<RefCell<Env>>,
}

#[derive(Debug)]
pub enum Value {
    DATUM(Rc<Datum>),
    CLOSURE(Rc<Clojure>),
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }
    pub fn with_parent(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }
    fn resolve(&self, identifier: &str) -> Option<Rc<Value>> {
        self.values
            .get(identifier)
            .map(|value| (*value).clone())
            .or_else(|| {
                self.parent
                    .as_ref()
                    .and_then(|parent| (**parent).borrow().resolve(identifier))
            })
    }

    fn insert(&mut self, identifier: String, value: Rc<Value>) {
        self.values.insert(identifier, value);
    }
}

#[allow(unused)]
pub fn eval(top: Top, env: &Rc<RefCell<Env>>) -> Result<Option<Rc<Value>>, String> {
    let res = match top {
        Top::DEC {
            identifier,
            expression,
        } => {
            let value = eval_expr(expression, env).ok_or("error")?;
            env.borrow_mut().insert(identifier, value);
            None
        }
        Top::EXP { expression } => eval_expr(expression, env),
    };
    Ok(res)
}

// I can match RC instead
fn eval_expr(expr: Rc<Exp>, env: &Rc<RefCell<Env>>) -> Option<Rc<Value>> {
    match &*expr {
        Exp::IDENTIFIER(identifier) => (**env).borrow().resolve(&identifier),
        Exp::LITERIAL(datum) => match (**datum).borrow() {
            Datum::LAMBDA(lambda) => Some(Rc::new(Value::CLOSURE(Rc::new(Clojure {
                proto: lambda.clone(),
                env: env.clone(),
            })))),
            _ => Some(Rc::new(Value::DATUM(datum.clone()))),
        },
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
            if let Value::DATUM(datum) = (*test).borrow() {
                if let Datum::BOOLEAN(true) = (*datum).borrow() {
                    return eval_expr(consequent.clone(), env);
                } else {
                    if let Some(alternative) = alternative {
                        return eval_expr(alternative.clone(), env);
                    }
                }
            }
            None
        }
    }
}

#[allow(unused)]
fn apply(
    operator: Rc<Value>,
    operands: Vec<Rc<Value>>,
    env: &Rc<RefCell<Env>>,
) -> Option<Rc<Value>> {
    match &*operator {
        Value::CLOSURE(clojure) => {
            let Lambda {
                parameters,
                definitions,
                body,
            } = &*clojure.proto;
            let env = &clojure.env;
            if parameters.len() != operands.len() {
                println!("wrong number of arguments!");
                return None;
            }
            // should use lexical scope
            let mut env = Rc::new(RefCell::new(Env::with_parent(env.clone())));
            for (parameter, operand) in parameters.iter().zip(operands) {
                env.borrow_mut().insert(parameter.clone(), operand);
            }
            for (identifier, expression) in definitions {
                let value = eval_expr(expression.clone(), &env)?;
                env.borrow_mut().insert(identifier.clone(), value);
            }
            eval_expr(body.clone(), &env)
        }
        _ => {
            println!("call on non-function!");
            None
        }
    }
}
