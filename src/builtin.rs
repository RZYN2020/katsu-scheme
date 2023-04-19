use crate::interpreter::*;
use crate::parser::*;
use std::rc::Rc;

// Get all the builtin functions

pub fn get_builtins() -> Vec<(&'static str, fn(Vec<Rc<Value>>) -> Option<Rc<Value>>)> {
    vec![
        ("number?", if_number),
        ("=", equal_number),
        ("+", plus_number),
        ("-", minus_number),
        ("*", times_number),
        ("/", divide_number),
        ("<", less_number),
        (">", greater_number),
    ]
}

// Equivalence predicates

// Numbers

pub fn if_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    match &*args[0] {
        Value::DATUM(datum) => match datum.as_ref() {
            Datum::NUMBER(_) => Some(Rc::new(Value::DATUM(Rc::new(Datum::BOOLEAN(true))))),
            _ => Some(Rc::new(Value::DATUM(Rc::new(Datum::BOOLEAN(false))))),
        },
        _ => None,
    }
}

macro_rules! binop_number {
    ($args:expr, $op:expr) => {
        if $args.len() != 2 {
            return None;
        } else {
            match $args.as_slice() {
                [n1, n2] => match (&**n1, &**n2) {
                    (Value::DATUM(d1), Value::DATUM(d2)) => match (&**d1, &**d2) {
                        (Datum::NUMBER(n1), Datum::NUMBER(n2)) => {
                            Some(Rc::new(Value::DATUM(Rc::new($op(n1, n2)))))
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            }
        }
    };
}

pub fn equal_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Datum::BOOLEAN(n1 == n2))
}

pub fn less_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Datum::BOOLEAN(n1 < n2))
}

pub fn greater_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Datum::BOOLEAN(n1 > n2))
}

pub fn plus_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Datum::NUMBER(n1 + n2))
}

pub fn minus_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Datum::NUMBER(n1 - n2))
}

pub fn times_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Datum::NUMBER(n1 * n2))
}

pub fn divide_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Datum::NUMBER(n1 / n2))
}

// Booleans
