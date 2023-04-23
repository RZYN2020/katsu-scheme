use pest::iterators::{Pairs, Pair};
use pest::Parser;
pub struct PestParser;

use super::*;

#[derive(Parser)]
#[grammar = "scheme.pest"]
pub struct PestDriver;

impl PestParser {
    pub fn new () -> Self {
        Self {}
    }
}

macro_rules! handle_literals {
    ($item:expr) => {
        match $item.as_rule() {
            Rule::number => {
                let number = $item.as_str().parse::<i64>().ok()?;
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

macro_rules! inner1 {
    ($item:expr) => {
        $item.into_inner().next()?
    };
}

macro_rules! inner2 {
    ($item:expr) => {
        $item.into_inner().next()?.into_inner().next()?
    };
}


fn build_compound(pair: Pair<Rule>) -> Option<Datum> {
    let pair = inner1!(pair);
    match pair.as_rule() {
        Rule::pure_list => {
            let mut pairs = pair.into_inner().rev();
            let mut curr = Datum::NIL;
            while let Some(p) = pairs.next() {
                let datum = build_datum(p)?;
                curr = Datum::PAIR((Some(Rc::new(datum)), Some(Rc::new(curr))));
            }
            Some(curr)
        }
        Rule::list_pair => {
            let mut pairs = pair.into_inner().rev();
            let mut curr = build_datum(pairs.next()?)?;
            while let Some(p) = pairs.next() {
                let datum = build_datum(p)?;
                curr = Datum::PAIR((Some(Rc::new(datum)), Some(Rc::new(curr))));
            }
            Some(curr)
        }
        _ => unreachable!(),
    }
}

fn build_datum(pair: Pair<Rule>) -> Option<Datum> {
    match pair.as_rule() {
        Rule::simple_datum => {
            let simple_datum = inner1!(pair);
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
            let compound_datum = inner1!(pair);
            match compound_datum.as_rule() {
                Rule::list => build_compound(compound_datum),
                Rule::abbr => {
                    let abbr = inner1!(compound_datum);
                    let datum = build_datum(abbr)?;
                    Some(Datum::ABBR(Rc::new(datum)))
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
            return build_datum(inner1!(pair));
        }
        Rule::self_evaluating => {
            let literal = inner1!(pair);
            handle_literals!(literal);
        }
        _ => unreachable!(),
    }
}

fn build_call(pair: Pair<Rule>) -> Option<Exp> {
    let mut pairs = pair.into_inner();
    let operator = build_exp(inner2!(pairs.next()?))?;
    let mut operands = Vec::new();
    for pair in pairs {
        operands.push(Rc::new(build_exp(inner2!(pair))?));
    }
    Some(Exp::CALL {
        operator: Rc::new(operator),
        operands,
    })
}

fn build_lambda(pair: Pair<Rule>) -> Option<Exp> {
    let mut pairs = pair.into_inner();
    let mut parameters = Vec::new();
    for pair in pairs.next()?.into_inner() {
        parameters.push(pair.as_str().to_string());
    }
    let mut definitions = Vec::new();
    for pair in pairs.next()?.into_inner() {
        let mut pairs = pair.into_inner();
        let identifier = pairs.next()?.as_str().to_string();
        let expression = Rc::new(build_exp(pairs.next()?)?);
        definitions.push((identifier, expression));
    }
    let body = Rc::new(build_exp(inner1!(pairs.next()?))?);
    Some(Exp::LITERIAL(Rc::new(Datum::LAMBDA(Rc::new(Lambda {
        parameters,
        definitions,
        body,
    })))))
}

fn build_cond(pair: Pair<Rule>) -> Option<Exp> {
    let mut pairs = pair.into_inner();
    let mut alternative = None;
    let test = Some(Rc::new(build_exp(inner2!(pairs.next()?))?));
    let consequent = Some(Rc::new(build_exp(inner2!(pairs.next()?))?));
    if let Some(pair) = pairs.next() {
        alternative = Some(Rc::new(build_exp(inner2!(pair))?));
    }
    return Some(Exp::COND {
        test: test?,
        consequent: consequent?,
        alternative,
    });
}

fn build_derived(pair: Pair<Rule>) -> Option<Exp> {
    let pair = pair.into_inner().next()?;
    match pair.as_rule() {
        Rule::no_else_cond => {
            let mut current = Some(Exp::LITERIAL(Rc::new(Datum::NIL)));
            let mut conds = pair.into_inner().rev();
            while let Some(pair) = conds.next() {
                let mut pairs = pair.into_inner();
                let test = Rc::new(build_exp(inner2!(pairs.next()?))?);
                let consequent = Rc::new(build_exp(inner1!(pairs.next()?))?);
                current = Some(Exp::COND {
                    test,
                    consequent,
                    alternative: Some(Rc::new(current?)),
                });
            }
            current
        }
        Rule::else_cond => {
            let mut pairs = pair.into_inner();
            let mut current = Some(build_exp(inner1!(pairs.next()?))?);
            let mut conds = pairs.next()?.into_inner().rev();
            while let Some(pair) = conds.next() {
                let mut pairs = pair.into_inner();
                let test = Rc::new(build_exp(inner2!(pairs.next()?))?);
                let consequent = Rc::new(build_exp(inner1!(pairs.next()?))?);
                current = Some(Exp::COND {
                    test,
                    consequent,
                    alternative: Some(Rc::new(current?)),
                });
            }
            current
        }
        Rule::and => {
            let mut current = Some(Exp::LITERIAL(Rc::new(Datum::BOOLEAN(true))));
            let mut pairs = pair.into_inner().rev();
            while let Some(pair) = pairs.next() {
                let test = Rc::new(build_exp(inner2!(pair))?);
                current = Some(Exp::COND {
                    test,
                    consequent: Rc::new(current?),
                    alternative: Some(Rc::new(Exp::LITERIAL(Rc::new(
                        Datum::BOOLEAN(false),
                    )))),
                });
            }
            current
        }
        Rule::or => {
            let mut current = Some(Exp::LITERIAL(Rc::new(Datum::BOOLEAN(false))));
            let mut pairs = pair.into_inner().rev();
            while let Some(pair) = pairs.next() {
                let test = Rc::new(build_exp(inner2!(pair))?);
                current = Some(Exp::COND {
                    test,
                    consequent: Rc::new(Exp::LITERIAL(Rc::new(Datum::BOOLEAN(
                        true,
                    )))),
                    alternative: Some(Rc::new(current?)),
                });
            }
            current
        }
        Rule::let_exp => {
            let mut pairs = pair.into_inner();
            let mut definitions = Vec::new();
            for pair in pairs.next()?.into_inner() {
                let mut pairs = pair.into_inner();
                let identifier = pairs.next()?.as_str().to_string();
                let expression = Rc::new(build_exp(pairs.next()?)?);
                definitions.push((identifier, expression));
            }
            let body = Rc::new(build_exp(inner1!(pairs.next()?))?);
            Some(Exp::LITERIAL(Rc::new(Datum::LAMBDA(
                Rc::new(Lambda {
                    parameters: Vec::new(),
                    definitions,
                    body,
                }),
            ))))
        }
        _ => unreachable!(),
    }
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
            let literal = build_literal(inner1!(pair))?;
            Some(Exp::LITERIAL(Rc::new(literal)))
        }
        Rule::call => build_call(pair),
        Rule::lambda => build_lambda(pair),
        Rule::cond => build_cond(pair),
        Rule::derived => build_derived(pair),
        _ => unreachable!(),
    }
}

// Traverse the parse tree to build the AST
fn build_ast(pairs: Pairs<Rule>) -> Option<Ast> {
    let mut ast = Ast {
        tops: Vec::new(),
    };
    for pair in pairs {
        match pair.as_rule() {
            Rule::exp => {
                let pair = inner1!(pair);
                ast.tops.push(Top::EXP {
                    expression: Rc::new(build_exp(pair)?),
                });
            }
            Rule::def => {
                let mut pairs = pair.into_inner();
                let identifier = pairs.next()?.as_str().to_string();
                let expression = Rc::new(build_exp(inner1!(pairs.next()?))?);
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


impl SchemeParser for PestParser {
    fn parse(&self, program: &str) -> Ast {
        let mut pairs = PestDriver::parse(Rule::prog, program);
        match pairs {
            Ok(ref mut pairs) => {
                let result = pairs
                    .find(|pair| pair.as_rule() == Rule::prog)
                    .map(|pair| build_ast(pair.into_inner()));
                match result {
                    Some(Some(ast)) => ast,
                    _ => panic!("AST Building Error!!!"),
                }
            }
            Err(e) => {
                println!("{}", e);
                panic!("Parse Error!!!");
            }
        }
    }
}