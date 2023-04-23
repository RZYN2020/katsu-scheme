use std::fmt::Display;
use std::rc::Rc;

mod hand_parser;
mod pest_parser;

trait SchemeParser {
    fn parse(&self, input: &str) -> Ast;
}

pub fn parse(input: &str) -> Ast {
    match std::env::var("PARSER").as_ref().map(|s| s.as_str()) {
        Ok("pest") => pest_parser::PestParser::new().parse(input),
        Ok("hand") => hand_parser::HandParser::new().parse(input),
        _ => panic!("The environment variable PARSER is not set."),
    }
}

#[derive(Debug)]
pub struct Ast {
    pub tops: Vec<Top>,
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
    LAMBDA(Rc<Lambda>),
    NUMBER(i64),
    BOOLEAN(bool),
    STRING(String),
    SYMBOL(String),
    ABBR(Rc<Datum>),
    PAIR((Option<Rc<Datum>>, Option<Rc<Datum>>)),
    NIL,
}

impl Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Datum::LAMBDA(lambda) => write!(f, "#<lambda {:?}>", lambda),
            Datum::NUMBER(number) => write!(f, "{}", number),
            Datum::BOOLEAN(boolean) => write!(f, "{}", boolean),
            Datum::STRING(string) => write!(f, "{}", string),
            Datum::SYMBOL(symbol) => write!(f, "{}", symbol),
            Datum::ABBR(datum) => write!(f, "'{}", datum),
            Datum::PAIR((Some(car), Some(cdr))) => write!(f, "({} . {})", car, cdr),
            Datum::PAIR((Some(car), None)) => write!(f, "({})", car),
            Datum::PAIR((None, Some(cdr))) => write!(f, "({})", cdr),
            Datum::PAIR((None, None)) => write!(f, "()"),
            Datum::NIL => write!(f, "()"),
        }
    }
}

#[derive(Debug)]
pub struct Lambda {
    pub parameters: Vec<String>,
    pub definitions: Vec<(String, Rc<Exp>)>,
    pub body: Rc<Exp>,
}
