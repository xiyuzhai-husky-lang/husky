use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_use_ast(
        &mut self,
        token_verse_idx: TokenVerseIdx,
        visibility_expr: VisibilityExpr,
        state_after_visibility_expr: Option<TokenStreamState>,
    ) -> AstData {
        AstData::Use {
            token_verse_idx,
            visibility_expr,
            state_after_visibility_expr,
        }
    }
}
