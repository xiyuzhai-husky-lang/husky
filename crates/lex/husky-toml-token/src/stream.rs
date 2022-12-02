use crate::*;

#[derive(Debug)]
pub struct TomlTokenStream<'a> {
    tokens: &'a [TomlToken],
    current: usize,
}

impl<'a> TomlTokenStream<'a> {
    pub fn new(tokens: &'a [TomlToken]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn next(&mut self) -> Option<&'a TomlToken> {
        if self.current < self.tokens.len() {
            let index = self.current;
            self.current += 1;
            Some(&self.tokens[index])
        } else {
            None
        }
    }
}
