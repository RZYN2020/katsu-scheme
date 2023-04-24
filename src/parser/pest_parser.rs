use pest::iterators::{Pair, Pairs};
use pest::Parser;

pub struct PestParser;

use super::*;

#[derive(Parser)]
#[grammar = "parser/scheme.pest"]
pub struct PestDriver;

macro_rules! handle_literals {
    ($item:expr) => {
        match $item.as_rule() {
            Rule::number => {
                let number = $item.as_str().parse::<i64>().ok()?;
                return Some(Datum::PRIMITIVE(Rc::new(Primitive::NUMBER(number))));
            }
            Rule::boolean => {
                let boolean = $item.as_str();
                if (boolean == "#t") {
                    return Some(Datum::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true))));
                } else {
                    return Some(Datum::PRIMITIVE(Rc::new(Primitive::BOOLEAN(false))));
                }
            }
            Rule::string => {
                let string = $item.as_str().to_string();
                return Some(Datum::PRIMITIVE(Rc::new(Primitive::STRING(string))));
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

impl PestParser {
    pub fn new() -> Self {
        Self {}
    }

    fn build_compounds(&self, pair: Pair<Rule>) -> Option<Datum> {
        let pair = inner1!(pair);
        match pair.as_rule() {
            Rule::pure_list => {
                let mut pairs = pair.into_inner().rev();
                let mut curr = Datum::PRIMITIVE(Rc::new(Primitive::NIL));
                while let Some(p) = pairs.next() {
                    let datum = self.build_datum(p)?;
                    curr = Datum::PAIR((Some(Rc::new(datum)), Some(Rc::new(curr))));
                }
                Some(curr)
            }
            Rule::list_pair => {
                let mut pairs = pair.into_inner().rev();
                let mut curr = self.build_datum(pairs.next()?)?;
                while let Some(p) = pairs.next() {
                    let datum = self.build_datum(p)?;
                    curr = Datum::PAIR((Some(Rc::new(datum)), Some(Rc::new(curr))));
                }
                Some(curr)
            }
            _ => unreachable!(),
        }
    }

    fn build_datum(&self, pair: Pair<Rule>) -> Option<Datum> {
        match pair.as_rule() {
            Rule::simple_datum => {
                let simple_datum = inner1!(pair);
                match simple_datum.as_rule() {
                    Rule::number | Rule::boolean | Rule::string => {
                        handle_literals!(simple_datum);
                    }
                    Rule::symbol => {
                        let symbol = simple_datum.as_str().to_string();
                        return Some(Datum::PRIMITIVE(Rc::new(Primitive::SYMBOL(symbol))));
                    }
                    _ => unreachable!(),
                }
            }
            Rule::compound_datum => {
                let compound_datum = inner1!(pair);
                match compound_datum.as_rule() {
                    Rule::list => self.build_compounds(compound_datum),
                    Rule::abbr => {
                        let abbr = inner1!(compound_datum);
                        self.build_datum(abbr)
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn build_literal(&self, pair: Pair<Rule>) -> Option<Datum> {
        match pair.as_rule() {
            Rule::quotation => {
                return self.build_datum(inner1!(pair));
            }
            Rule::self_evaluating => {
                let literal = inner1!(pair);
                handle_literals!(literal);
            }
            _ => unreachable!(),
        }
    }

    fn build_call(&self, pair: Pair<Rule>) -> Option<Exp> {
        let mut pairs = pair.into_inner();
        let operator = self.build_exp(inner2!(pairs.next()?))?;
        let mut operands = Vec::new();
        for pair in pairs {
            operands.push(Rc::new(self.build_exp(inner2!(pair))?));
        }
        Some(Exp::CALL {
            operator: Rc::new(operator),
            operands,
        })
    }

    fn build_lambda(&self, pair: Pair<Rule>) -> Option<Exp> {
        let mut pairs = pair.into_inner();
        let mut parameters = Vec::new();
        let formal = inner1!(pairs.next()?);
        let ifvarlen = formal.as_rule() == Rule::varlen;
        for pair in formal.into_inner() {
            parameters.push(pair.as_str().to_string());
        }
        let mut definitions = Vec::new();
        for pair in pairs.next()?.into_inner() {
            let mut pairs = pair.into_inner();
            let identifier = pairs.next()?.as_str().to_string();
            let expression = Rc::new(self.build_exp(pairs.next()?)?);
            definitions.push((identifier, expression));
        }
        let body = Rc::new(self.build_exp(inner1!(pairs.next()?))?);
        Some(Exp::LITERIAL(Rc::new(Datum::LAMBDA(Rc::new(Lambda {
            ifvarlen,
            parameters,
            definitions,
            body,
        })))))
    }

    fn build_cond(&self, pair: Pair<Rule>) -> Option<Exp> {
        let mut pairs = pair.into_inner();
        let mut alternative = None;
        let test = Some(Rc::new(self.build_exp(inner2!(pairs.next()?))?));
        let consequent = Some(Rc::new(self.build_exp(inner2!(pairs.next()?))?));
        if let Some(pair) = pairs.next() {
            alternative = Some(Rc::new(self.build_exp(inner2!(pair))?));
        }
        return Some(Exp::COND {
            test: test?,
            consequent: consequent?,
            alternative,
        });
    }

    fn build_derived(&self, pair: Pair<Rule>) -> Option<Exp> {
        let pair = pair.into_inner().next()?;
        match pair.as_rule() {
            Rule::no_else_cond => {
                let mut current = Some(Exp::LITERIAL(Rc::new(Datum::PRIMITIVE(Rc::new(Primitive::NIL)))));
                let mut conds = pair.into_inner().rev();
                while let Some(pair) = conds.next() {
                    let mut pairs = pair.into_inner();
                    let test = Rc::new(self.build_exp(inner2!(pairs.next()?))?);
                    let consequent = Rc::new(self.build_exp(inner1!(pairs.next()?))?);
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
                let mut current = Some(self.build_exp(inner1!(pairs.next()?))?);
                let mut conds = pairs.next()?.into_inner().rev();
                while let Some(pair) = conds.next() {
                    let mut pairs = pair.into_inner();
                    let test = Rc::new(self.build_exp(inner2!(pairs.next()?))?);
                    let consequent = Rc::new(self.build_exp(inner1!(pairs.next()?))?);
                    current = Some(Exp::COND {
                        test,
                        consequent,
                        alternative: Some(Rc::new(current?)),
                    });
                }
                current
            }
            Rule::and => {
                let mut current = Some(Exp::LITERIAL(Rc::new(Datum::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true))))));
                let mut pairs = pair.into_inner().rev();
                while let Some(pair) = pairs.next() {
                    let test = Rc::new(self.build_exp(inner2!(pair))?);
                    current = Some(Exp::COND {
                        test,
                        consequent: Rc::new(current?),
                        alternative: Some(Rc::new(Exp::LITERIAL(Rc::new(Datum::PRIMITIVE(Rc::new(Primitive::BOOLEAN(false))))))),
                    });
                }
                current
            }
            Rule::or => {
                let mut current = Some(Exp::LITERIAL(Rc::new(Datum::PRIMITIVE(Rc::new(Primitive::BOOLEAN(false))))));
                let mut pairs = pair.into_inner().rev();
                while let Some(pair) = pairs.next() {
                    let test = Rc::new(self.build_exp(inner2!(pair))?);
                    current = Some(Exp::COND {
                        test,
                        consequent: Rc::new(Exp::LITERIAL(Rc::new(Datum::PRIMITIVE(Rc::new(Primitive::BOOLEAN(true)))))),
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
                    let expression = Rc::new(self.build_exp(pairs.next()?)?);
                    definitions.push((identifier, expression));
                }
                let body = Rc::new(self.build_exp(inner1!(pairs.next()?))?);
                Some(Exp::LITERIAL(Rc::new(Datum::LAMBDA(Rc::new(Lambda {
                    ifvarlen: false,
                    parameters: Vec::new(),
                    definitions,
                    body,
                })))))
            }
            _ => unreachable!(),
        }
    }

    // IO can be classfied as (native) procedure
    // Derived Form can be transformed to normal expression
    fn build_exp(&self, pair: Pair<Rule>) -> Option<Exp> {
        match pair.as_rule() {
            Rule::identifier => {
                let identifier = pair.as_str().to_string();
                Some(Exp::IDENTIFIER(identifier))
            }
            Rule::literal => {
                let literal = self.build_literal(inner1!(pair))?;
                Some(Exp::LITERIAL(Rc::new(literal)))
            }
            Rule::call => self.build_call(pair),
            Rule::lambda => self.build_lambda(pair),
            Rule::cond => self.build_cond(pair),
            Rule::derived => self.build_derived(pair),
            _ => unreachable!(),
        }
    }

    // Traverse the parse tree to build the AST
    fn build_ast(&self, pairs: Pairs<Rule>) -> Option<Vec<Top>> {
        let mut tops = Vec::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::exp => {
                    let pair = inner1!(pair);
                    tops.push(Top::EXP {
                        expression: Rc::new(self.build_exp(pair)?),
                    });
                }
                Rule::def => {
                    let mut pairs = pair.into_inner();
                    let identifier = pairs.next()?.as_str().to_string();
                    let expression = Rc::new(self.build_exp(inner1!(pairs.next()?))?);
                    tops.push(Top::DEC {
                        identifier,
                        expression,
                    });
                }
                Rule::EOI => {
                    return Some(tops);
                }
                _ => unreachable!(),
            }
        }
        Some(tops)
    }
}

impl SchemeParser for PestParser {
    fn parse(&self, program: &str) -> Ast {
        let mut pairs = PestDriver::parse(Rule::prog, program);
        match pairs {
            Ok(ref mut pairs) => {
                let result = pairs
                    .find(|pair| pair.as_rule() == Rule::prog)
                    .map(|pair| self.build_ast(pair.into_inner()));
                match result {
                    Some(Some(tops)) => Ast::new(tops),
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
