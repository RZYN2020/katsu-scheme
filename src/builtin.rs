use crate::interpreter::*;
use crate::parser::*;
use std::rc::Rc;

// Get all the builtin functions

pub fn get_builtins() -> Vec<(&'static str, fn(Vec<Rc<Value>>) -> Option<Rc<Value>>)> {
    vec![
        ("eqv?", eqv),
        ("number?", if_number),
        ("=", equal_number),
        ("+", plus_number),
        ("-", minus_number),
        ("*", times_number),
        ("/", divide_number),
        ("<", less_number),
        (">", greater_number),
        ("!", not_boolean),
        ("boolean?", if_boolean),
        ("pair?", if_pair),
        ("cons", cons),
        ("car", car),
        ("cdr", cdr),
        ("symbol?", if_symbol),
        ("procedure?", if_procedure),
    ]
}

// Equivalence predicates

pub fn eqv(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 2 {
        return None;
    } else {
        match args.as_slice() {
            [v1, v2] => Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(
                **v1 == **v2,
            ))))),
            _ => panic!("eqv called with non-array arguments"),
        }
    }
}



// Numbers

pub fn if_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    match &*args[0] {
        Value::PRIMITIVE(datum) => match datum.as_ref() {
            Primitive::NUMBER(_) => {
                Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true)))))
            }
            _ => Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(
                false,
            ))))),
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
                    (Value::PRIMITIVE(d1), Value::PRIMITIVE(d2)) => match (&**d1, &**d2) {
                        (Primitive::NUMBER(n1), Primitive::NUMBER(n2)) => {
                            Some(Rc::new(Value::PRIMITIVE(Rc::new($op(n1, n2)))))
                        }
                        _ => panic!("binop_number! called with non-number arguments"),
                    },
                    _ => panic!("binop_number! called with non-primitive arguments"),
                },
                _ => panic!("binop_number! called with non-array arguments"),
            }
        }
    };
}

pub fn equal_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Primitive::BOOLEAN(n1 == n2))
}

pub fn less_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Primitive::BOOLEAN(n1 < n2))
}

pub fn greater_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Primitive::BOOLEAN(n1 > n2))
}

pub fn plus_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Primitive::NUMBER(n1 + n2))
}

pub fn minus_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Primitive::NUMBER(n1 - n2))
}

pub fn times_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Primitive::NUMBER(n1 * n2))
}

pub fn divide_number(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    binop_number!(args, |n1, n2| Primitive::NUMBER(n1 / n2))
}

// Booleans

pub fn not_boolean(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 1 {
        return None;
    } else {
        match &*args[0] {
            Value::PRIMITIVE(datum) => match datum.as_ref() {
                Primitive::BOOLEAN(b) => {
                    Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(!b)))))
                }
                _ => panic!("not_boolean: not a boolean"),
            },
            _ => panic!("not_boolean: not a boolean"),
        }
    }
}

pub fn if_boolean(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 1 {
        return None;
    } else {
        match &*args[0] {
            Value::PRIMITIVE(datum) => match datum.as_ref() {
                Primitive::BOOLEAN(_) => {
                    Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true)))))
                }
                _ => Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(
                    false,
                ))))),
            },
            _ => Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true))))),
        }
    }
}

// Pairs and lists

pub fn if_pair(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 1 {
        return None;
    } else {
        match &*args[0] {
            Value::PAIR { car: _, cdr: _ } => {
                Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true)))))
            }
            _ => Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(
                false,
            ))))),
        }
    }
}

pub fn cons(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 2 {
        return None;
    } else {
        Some(Rc::new(Value::PAIR {
            car: Some(args[0].clone()),
            cdr: Some(args[1].clone()),
        }))
    }
}

pub fn car(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 1 {
        return None;
    } else {
        match &*args[0] {
            Value::PAIR {
                car: Some(car),
                cdr: _,
            } => Some(car.clone()),
            _ => panic!("car: not a pair"),
        }
    }
}

pub fn cdr(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 1 {
        return None;
    } else {
        match &*args[0] {
            Value::PAIR {
                car: _,
                cdr: Some(cdr),
            } => Some(cdr.clone()),
            _ => panic!("cdr: not a pair"),
        }
    }
}

// Symbols

pub fn if_symbol(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 1 {
        return None;
    } else {
        match &*args[0] {
            Value::PRIMITIVE(primitive) => match primitive.as_ref() {
                Primitive::SYMBOL(_) => {
                    Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true)))))
                }
                _ => Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(
                    false,
                ))))),
            },
            _ => Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(
                false,
            ))))),
        }
    }
}


// Control features

pub fn if_procedure(args: Vec<Rc<Value>>) -> Option<Rc<Value>> {
    if args.len() != 1 {
        return None;
    } else {
        match &*args[0] {
            Value::CLOSURE(_) | Value::BUILTIN(_) => {
                Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true)))))
            }
            _ => Some(Rc::new(Value::PRIMITIVE(Rc::new(Primitive::BOOLEAN(
                false,
            ))))),
        }
    }
}
