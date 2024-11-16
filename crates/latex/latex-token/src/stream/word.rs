use crate::{idx::LxCowordTokenIdx, lexer::LxLexer, token::word::LxWordTokenData};

pub struct LxWordTokenStream<'a> {
    lexer: LxLexer<'a>,
}

impl<'a> LxWordTokenStream<'a> {
    pub fn new(lexer: LxLexer<'a>) -> Self {
        Self { lexer }
    }
}

impl<'a> Iterator for LxWordTokenStream<'a> {
    type Item = (LxCowordTokenIdx, LxWordTokenData);

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_coword_token()
    }
}
