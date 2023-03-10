use husky_opn_syntax::{BinaryClosedOpr, BinaryOpr, Bracket};
use husky_token::{TokenParseContext, TokenStream};
use parsec::StreamWrapper;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_use_ast(&mut self, token_group_idx: TokenGroupIdx, ctx: &Context) -> Ast {
        Ast::Use { token_group_idx }
    }
}
