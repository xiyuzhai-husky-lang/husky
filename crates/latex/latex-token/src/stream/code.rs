use crate::{data::coword::LxCowordTokenData, idx::LxCowordTokenIdx, lexer::LxLexer};

pub struct LxCodeTokenStream<'a> {
    lexer: LxLexer<'a>,
}

impl<'a> LxCodeTokenStream<'a> {
    pub fn new(lexer: LxLexer<'a>) -> Self {
        Self { lexer }
    }
}

impl<'a> Iterator for LxCodeTokenStream<'a> {
    type Item = (LxCowordTokenIdx, LxCowordTokenData);

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_coword_token()
    }
}
