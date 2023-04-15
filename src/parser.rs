use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "scheme.pest"]
pub struct SchemeParser;

#[derive(Debug)]
pub struct Ast {
    pub name: String,
    pub args: Vec<Top>,
}

#[derive(Debug)]
pub enum Top {
   DEF {
    identifier: String,
    expression: Exp,
   },
   EXP {
    expression: Exp,
   }, 
}


// IO can be classfied as (native) procedure
// Derived Form can be transformed to normal expression
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
        body: Vec<Exp>,
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
    // compound ara actually pair
    // list is a special case of pair
    COMPOUND((Option<Box<Datum>>, Option<Box<Datum>>)),
}

// Traverse the parse tree to build the AST
fn build_sexpr(pair: Pair<Rule>) -> Option<Ast> {
    match pair.as_rule() {
        _ => None
    }
}


#[allow(unused)]
pub fn parse(program: &str) -> Option<Ast> {
    let mut pairs = 
        SchemeParser::parse(Rule::prog, program).ok()?;
    pairs.find(|pair| pair.as_rule() == Rule::prog)
        .map(|pair| build_sexpr(pair))?
}

mod test {
    #[allow(unused)]
    use crate::parser::*;
    #[test]
    fn pt() {
    }
}
