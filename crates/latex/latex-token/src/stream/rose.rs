use crate::{data::rose::LxRoseTokenData, idx::LxTokenIdx, lexer::LxLexer};

pub struct LxRoseTokenStream<'a> {
    lexer: LxLexer<'a>,
}

impl<'a> LxRoseTokenStream<'a> {
    pub fn new(lexer: LxLexer<'a>) -> Self {
        Self { lexer }
    }
}

impl<'a> Iterator for LxRoseTokenStream<'a> {
    type Item = (LxTokenIdx, LxRoseTokenData);

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_rose_token()
    }
}
