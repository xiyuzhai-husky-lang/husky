use husky_opn_syntax::{BinaryClosedOpr, BinaryOpr, Bracket};
use husky_token::{TokenParseContext, TokenStream};
use parsec::StreamWrapper;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_use_ast(
        &mut self,
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        saved_stream_state: Option<TokenIdx>,
    ) -> Ast {
        Ast::Use {
            token_group_idx,
            visibility_expr,
            state_after_visibility_expr: saved_stream_state,
        }
    }
}
