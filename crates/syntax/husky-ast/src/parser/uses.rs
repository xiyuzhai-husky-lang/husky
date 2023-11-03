use husky_opr::{BinaryClosedOpr, Bracket};
use husky_token::{TokenStream, TokenStreamParser};
use parsec::StreamWrapper;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_use_ast(
        &mut self,
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        state_after_visibility_expr: Option<TokenStreamState>,
    ) -> Ast {
        Ast::Use {
            token_group_idx,
            visibility_expr,
            state_after_visibility_expr,
        }
    }
}
