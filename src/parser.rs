use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Parser)]
#[grammar = "scheme.pest"]
pub struct SchemeParser;

macro_rules! handle_literals {
    ($item:expr) => {
        match $item.as_rule() {
            Rule::number => {
                let number = $item.as_str().parse::<i64>().unwrap();
                return Some(Datum::NUMBER(number));
            }
            Rule::boolean => {
                let boolean = $item.as_str();
                if (boolean == "#t") {
                    return Some(Datum::BOOLEAN(true));
                } else {
                    return Some(Datum::BOOLEAN(false));
                }
            }
            Rule::string => {
                let string = $item.as_str().to_string();
                return Some(Datum::STRING(string));
            }
            _ => unreachable!(),
        }
    };
}

#[derive(Debug)]
pub struct Ast {
    pub name: String,
    pub tops: Vec<Top>,
}

#[derive(Debug)]
pub enum Top {
    DEC { identifier: String, expression: Exp },
    EXP { expression: Exp },
}

#[derive(Debug)]
pub enum Exp {
    IDENTIFIER(String),
    LITERIAL(Datum),
    CALL {
        operator: Box<Exp>,
        operands: Vec<Exp>,
    },
    LAMBDA {
        parameters: Vec<String>,
        definitions: Vec<(String, Exp)>,
        body: Box<Exp>,
    },
    COND {
        test: Box<Exp>,
        consequent: Box<Exp>,
        alternative: Option<Box<Exp>>,
    },
}

#[derive(Debug)]
pub enum Datum {
    NUMBER(i64),
    BOOLEAN(bool),
    STRING(String),
    SYMBOL(String),
    ABBR(Box<Datum>),
    COMPOUND((Option<Box<Datum>>, Option<Box<Datum>>)),
    NULL,
}

fn build_compound(pair: Pair<Rule>) -> Option<Datum> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::pure_list => {
            let mut pairs = pair.into_inner().rev();
            let mut curr = Datum::NULL;
            while let Some(p) = pairs.next() {
                let datum = build_datum(p).unwrap();
                curr = Datum::COMPOUND((Some(Box::new(datum)), Some(Box::new(curr))));
            }
            Some(curr)
        }
        Rule::list_pair => {
            let mut pairs = pair.into_inner().rev();
            let mut curr = build_datum(pairs.next().unwrap()).unwrap();
            while let Some(p) = pairs.next() {
                let datum = build_datum(p).unwrap();
                curr = Datum::COMPOUND((Some(Box::new(datum)), Some(Box::new(curr))));
            }
            Some(curr)
        }
        _ => unreachable!(),
    }
}

fn build_datum(pair: Pair<Rule>) -> Option<Datum> {
    match pair.as_rule() {
        Rule::simple_datum => {
            let simple_datum = pair.into_inner().next().unwrap();
            match simple_datum.as_rule() {
                Rule::number | Rule::boolean | Rule::string => {
                    handle_literals!(simple_datum);
                }
                Rule::symbol => {
                    let symbol = simple_datum.as_str().to_string();
                    return Some(Datum::SYMBOL(symbol));
                }
                _ => unreachable!(),
            }
        }
        Rule::compound_datum => {
            let compound_datum = pair.into_inner().next().unwrap();
            match compound_datum.as_rule() {
                Rule::list => build_compound(compound_datum),
                Rule::abbr => {
                    let abbr = compound_datum.into_inner().next().unwrap();
                    let datum = build_datum(abbr).unwrap();
                    Some(Datum::ABBR(Box::new(datum)))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn build_literal(pair: Pair<Rule>) -> Option<Datum> {
    match pair.as_rule() {
        Rule::quotation => {
            return build_datum(pair.into_inner().next().unwrap());
        }
        Rule::self_evaluating => {
            let literal = pair.into_inner().next().unwrap();
            handle_literals!(literal);
        }
        _ => unreachable!(),
    }
}

fn build_call(pair: Pair<Rule>) -> Option<Exp> {
    let mut pairs = pair.into_inner();
    let operator = build_exp(pairs.next().unwrap().into_inner().next().unwrap().into_inner().next().unwrap()).unwrap();
    let mut operands = Vec::new();
    for pair in pairs {
        operands.push(build_exp(pair.into_inner().next().unwrap().into_inner().next().unwrap()).unwrap());
    }
    Some(Exp::CALL {
        operator: Box::new(operator),
        operands,
    })
}

fn build_lambda(pair: Pair<Rule>) -> Option<Exp> {
    let mut pairs = pair.into_inner();
    let mut parameters = Vec::new();
    for pair in pairs.next().unwrap().into_inner() {
        parameters.push(pair.as_str().to_string());
    }
    let mut definitions = Vec::new();
    for pair in pairs.next().unwrap().into_inner() {
        let mut pairs = pair.into_inner();
        let identifier = pairs.next().unwrap().as_str().to_string();
        let expression = build_exp(pairs.next().unwrap()).unwrap();
        definitions.push((identifier, expression));
    }
    let body = Box::new(build_exp(pairs.next().unwrap().into_inner().next().unwrap()).unwrap());
    return Some(Exp::LAMBDA {
        parameters,
        definitions,
        body,
    });
}

fn build_cond(pair: Pair<Rule>) -> Option<Exp> {
    let mut pairs = pair.into_inner();
    let mut alternative = None;
    let test = Some(Box::new(build_exp(pairs.next().unwrap().into_inner().next().unwrap().into_inner().next().unwrap()).unwrap()));
    let consequent = Some(Box::new(build_exp(pairs.next().unwrap().into_inner().next().unwrap().into_inner().next().unwrap()).unwrap()));
    if let Some(pair) = pairs.next() {
        alternative = Some(Box::new(build_exp(pair.into_inner().next().unwrap().into_inner().next().unwrap()).unwrap()));
    }
    return Some(Exp::COND {
        test: test.unwrap(),
        consequent: consequent.unwrap(),
        alternative,
    });
}

// IO can be classfied as (native) procedure
// Derived Form can be transformed to normal expression
fn build_exp(pair: Pair<Rule>) -> Option<Exp> {
    match pair.as_rule() {
        Rule::identifier => {
            let identifier = pair.as_str().to_string();
            Some(Exp::IDENTIFIER(identifier))
        }
        Rule::literal => {
            let literal = build_literal(pair.into_inner().next().unwrap()).unwrap();
            Some(Exp::LITERIAL(literal))
        }
        Rule::call => {
            build_call(pair)
        }
        Rule::lambda => {
            build_lambda(pair)
        }
        Rule::cond => {
            build_cond(pair)
        }
        Rule::derived => {
            None
        }
        _ => unreachable!(),
    }
}

// Traverse the parse tree to build the AST
fn build_ast(pairs: Pairs<Rule>, name: &str) -> Option<Ast> {
    let mut ast = Ast {
        name: String::from(name),
        tops: Vec::new(),
    };
    for pair in pairs {
        match pair.as_rule() {
            Rule::exp => {
                let pair = pair.into_inner().next().unwrap();
                ast.tops.push(Top::EXP {
                    expression: build_exp(pair).unwrap(),
                });
            }
            Rule::def => {
                let mut pairs = pair.into_inner();
                let identifier = pairs.next().unwrap().as_str().to_string();
                let expression = build_exp(pairs.next().unwrap().into_inner().next().unwrap()).unwrap();
                ast.tops.push(Top::DEC {
                    identifier,
                    expression,
                });
            }
            Rule::EOI => {
                return Some(ast);
            }
            _ => unreachable!(),
        }
    }
    Some(ast)
}

pub fn parse(program: &str, name: &str) -> Option<Ast> {
    let mut pairs = SchemeParser::parse(Rule::prog, program).ok().unwrap();
    pairs
        .find(|pair| pair.as_rule() == Rule::prog)
        .map(|pair| build_ast(pair.into_inner(), name)).unwrap()
}

mod test {
    #[allow(unused)]
    use crate::parser::*;
    #[test]
    fn test_fib() {
        // fib only use if
        let program = "(define fib (lambda (n) (if (= n 0) 0 (if (= n 1) 1 (+ (fib (- n 1)) (fib (- n 2)))))))";
        let mut pairs = SchemeParser::parse(Rule::prog, program).unwrap();
        println!(
            "{:?}",
            build_ast(pairs.next().unwrap().into_inner(), "name")
        );
    }
}
