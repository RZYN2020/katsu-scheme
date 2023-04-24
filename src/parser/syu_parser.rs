use super::*;

pub struct SyuParser;

impl SyuParser {
    pub fn new () -> Self {
        Self {}
    }
}

impl SchemeParser for SyuParser {
    fn parse(&self, _program: &str) -> Ast {
        Ast {
            tops: Vec::new(),
        }
    }
}
