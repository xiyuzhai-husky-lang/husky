use super::*;
use husky_token::Token;

impl<'a> AstParser<'a> {
    pub(super) fn parse_impls(&mut self, token_group: TokenGroupIdx, indent: u32) -> Ast {
        todo!()
        // Ast::Use { token_group }
    }
}

struct ImplHeadParser<'a> {
    token_iter: core::iter::Peekable<core::slice::Iter<'a, Token>>,
}

impl<'a> ImplHeadParser<'a> {}
