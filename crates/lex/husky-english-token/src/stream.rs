use crate::*;

#[derive(Debug)]
pub struct EnglishTokenStream<'a> {
    tokens: &'a [EnglishToken],
    current: usize,
}

impl<'a> EnglishTokenStream<'a> {
    pub fn new(tokens: &'a [EnglishToken]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn next(&mut self) -> Option<&'a EnglishToken> {
        if self.current < self.tokens.len() {
            let index = self.current;
            self.current += 1;
            Some(&self.tokens[index])
        } else {
            None
        }
    }
}
