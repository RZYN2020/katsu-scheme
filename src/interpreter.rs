use crate::builtin::get_builtins;
use crate::parser::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
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
    PRIMITIVE(Rc<Primitive>),
    CLOSURE(Rc<Clojure>),
    BUILTIN(fn(Vec<Rc<Value>>) -> Option<Rc<Value>>),
    PAIR {
        car: Option<Rc<Value>>,
        cdr: Option<Rc<Value>>,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::PRIMITIVE(d1), Value::PRIMITIVE(d2)) => d1 == d2,
            (Value::CLOSURE(_), Value::CLOSURE(_)) => false,
            (Value::BUILTIN(_), Value::BUILTIN(_)) => false,
            (Value::PAIR { car: c1, cdr: d1 }, Value::PAIR { car: c2, cdr: d2 }) => {
                c1 == c2 && d1 == d2
            }
            _ => false,
        }
    }
}

impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Primitive::BOOLEAN(b1), Primitive::BOOLEAN(b2)) => b1 == b2,
            (Primitive::NUMBER(i1), Primitive::NUMBER(i2)) => i1 == i2,
            (Primitive::STRING(s1), Primitive::STRING(s2)) => s1 == s2,
            (Primitive::SYMBOL(s1), Primitive::SYMBOL(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::PRIMITIVE(datum) => write!(f, "{}", datum),
            Value::CLOSURE(clojure) => write!(f, "#<clojure {:?}>", clojure),
            Value::BUILTIN(_) => write!(f, "#<builtin>"),
            Value::PAIR { car, cdr } => match (car, cdr) {
                (Some(car), Some(cdr)) => write!(f, "({} . {})", car, cdr),
                (Some(car), None) => write!(f, "({})", car),
                (None, Some(cdr)) => write!(f, "({})", cdr),
                (None, None) => write!(f, "()"),
            },
        }
    }
}

impl Env {
    pub fn get_initialized_env() -> Rc<RefCell<Env>> {
        let env = Rc::new(RefCell::new(Env::new()));
        for (name, func) in get_builtins() {
            env.borrow_mut()
                .insert(name.to_string(), Rc::new(Value::BUILTIN(func)));
        }
        env
    }
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
                    .and_then(|parent| (parent.borrow().resolve(identifier)))
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

fn eval_expr(expr: Rc<Exp>, env: &Rc<RefCell<Env>>) -> Option<Rc<Value>> {
    // println!("eval_expr: {:?}", expr);
    match &*expr {
        Exp::IDENTIFIER(identifier) => env.borrow().resolve(&identifier),
        Exp::LITERIAL(datum) => eval_datum(datum.clone(), env),
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
            if let Value::PRIMITIVE(datum) = &*test {
                if let Primitive::BOOLEAN(true) = &**datum {
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

fn eval_datum(datum: Rc<Datum>, env: &Rc<RefCell<Env>>) -> Option<Rc<Value>> {
    match &*datum {
        Datum::LAMBDA(lambda) => Some(Rc::new(Value::CLOSURE(Rc::new(Clojure {
            proto: lambda.clone(),
            env: env.clone(),
        })))),
        Datum::PRIMITIVE(primitive) => Some(Rc::new(Value::PRIMITIVE(primitive.clone()))),
        Datum::PAIR((car, cdr)) => {
            let car = car.as_ref().and_then(|car| eval_datum(car.clone(), env));
            let cdr = cdr.as_ref().and_then(|cdr| eval_datum(cdr.clone(), env));
            Some(Rc::new(Value::PAIR { car, cdr }))
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
        Value::BUILTIN(builtin) => builtin(operands),
        Value::CLOSURE(clojure) => {
            let Lambda {
                ifvarlen,
                parameters,
                definitions,
                body,
            } = &*clojure.proto;
            let env = &clojure.env;
            // should use lexical scope
            let mut env = Rc::new(RefCell::new(Env::with_parent(env.clone())));
            if !ifvarlen {
                if parameters.len() != operands.len() {
                    println!("wrong number of arguments!");
                    return None;
                }
                for (parameter, operand) in parameters.iter().zip(operands) {
                    env.borrow_mut().insert(parameter.clone(), operand);
                }
            } else {
                let oprand = operands.into_iter().rev().fold(
                    Rc::new(Value::PRIMITIVE(Rc::new(Primitive::NIL))),
                    |acc, oprand| {
                        Rc::new(Value::PAIR {
                            car: Some(oprand),
                            cdr: Some(acc),
                        })
                    },
                );
                env.borrow_mut().insert(parameters[0].clone(), oprand);
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
