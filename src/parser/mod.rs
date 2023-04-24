use std::fmt::Display;
use std::rc::Rc;

mod syu_parser;
mod pest_parser;

trait SchemeParser {
    fn parse(&self, input: &str) -> Ast;
}

pub fn parse(input: &str) -> Ast {
    match std::env::var("PARSER").as_ref().map(|s| s.as_str()) {
        Ok("pest") => pest_parser::PestParser::new().parse(input),
        Ok("syu") => syu_parser::SyuParser::new().parse(input),
        _ => panic!("The environment variable PARSER is not set."),
    }
}

#[derive(Debug)]
pub struct Ast {
    pub tops: Vec<Top>,
}

impl Ast {
    pub fn new(tops: Vec<Top>) -> Self {
        Self { tops }
    }
}

#[derive(Debug)]
pub enum Top {
    DEC {
        identifier: String,
        expression: Rc<Exp>,
    },
    EXP {
        expression: Rc<Exp>,
    },
}

#[derive(Debug)]
pub enum Exp {
    IDENTIFIER(String),
    LITERIAL(Rc<Datum>),
    CALL {
        operator: Rc<Exp>,
        operands: Vec<Rc<Exp>>,
    },
    COND {
        test: Rc<Exp>,
        consequent: Rc<Exp>,
        alternative: Option<Rc<Exp>>,
    },
}

#[derive(Debug)]
pub enum Datum {
    PRIMITIVE(Rc<Primitive>),
    LAMBDA(Rc<Lambda>),
    PAIR((Option<Rc<Datum>>, Option<Rc<Datum>>)),
}

// primitives are shared by compiler and interpreter
#[derive(Debug)]
pub enum Primitive {
    NUMBER(i64),
    BOOLEAN(bool),
    STRING(String),
    SYMBOL(String),
    NIL,
}

impl Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Datum::LAMBDA(lambda) => write!(f, "#<lambda {:?}>", lambda),

            Datum::PAIR((Some(car), Some(cdr))) => write!(f, "({} . {})", car, cdr),
            Datum::PAIR((Some(car), None)) => write!(f, "({})", car),
            Datum::PAIR((None, Some(cdr))) => write!(f, "({})", cdr),
            Datum::PAIR((None, None)) => write!(f, "()"),
            Datum::PRIMITIVE(primitive) => write!(f, "{}", primitive),
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::NUMBER(number) => write!(f, "{}", number),
            Primitive::BOOLEAN(boolean) => write!(f, "{}", boolean),
            Primitive::STRING(string) => write!(f, "{}", string),
            Primitive::SYMBOL(symbol) => write!(f, "{}", symbol),
            Primitive::NIL => write!(f, "NIL"),
        }
    }
}

#[derive(Debug)]
pub struct Lambda {
    pub ifvarlen: bool,
    pub parameters: Vec<String>,
    pub definitions: Vec<(String, Rc<Exp>)>,
    pub body: Rc<Exp>,
}
