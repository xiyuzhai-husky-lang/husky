use super::*;
use husky_token::Token;
use std::iter::Peekable;

impl<'a> AstParser<'a> {
    pub(super) fn parse_impls(&mut self, token_group: TokenGroupIdx, indent: u32) -> Ast {
        todo!()
    }
}

struct ImplHeadParser<'a> {
    token_iter: Peekable<core::slice::Iter<'a, Token>>,
}
