use super::*;

pub struct HandParser;

impl HandParser {
    pub fn new () -> Self {
        Self {}
    }
}

impl SchemeParser for HandParser {
    fn parse(&self, program: &str) -> Ast {
        Ast {
            tops: Vec::new(),
        }
    }
}
