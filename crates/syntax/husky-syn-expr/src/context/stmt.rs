use super::*;
use husky_defn_ast::{DefnAstArenaRef, DefnAstIdx, DefnAstIdxRange};
use husky_entity_tree::helpers::tokra_region::DefnTokraRegionDataRef;

impl<'a> SynExprContext<'a> {
    pub(crate) fn parse_root_body(&mut self) -> SynExprIdx {
        let body = self.defn_tokra_region_data().root_body();
        let stmts = self.parse_stmts(body);
        let expr = SynExprData::Block { stmts };
        let expr = self.alloc_expr(expr);
        self.add_expr_root(SynExprRootKind::BlockExpr, expr);
        expr
    }

    pub(crate) fn token_verse_token_stream(
        &self,
        regional_token_verse_idx: RegionalTokenVerseIdx,
    ) -> RegionalTokenStream<'a> {
        self.defn_tokra_region_data()
            .token_stream(regional_token_verse_idx)
    }

    pub(crate) fn asts(&self) -> DefnAstArenaRef<'a> {
        self.defn_tokra_region_data().ast_arena()
    }

    pub(crate) fn ast_token_idx_range(&self, defn_ast_idx: DefnAstIdx) -> RegionalTokenIdxRange {
        self.defn_tokra_region_data()
            .ast_token_idx_range(defn_ast_idx)
    }

    pub(crate) fn form_body_end(&self, body: DefnAstIdxRange) -> RegionalTokenIdxRangeEnd {
        self.defn_tokra_region_data()
            .ast_token_idx_range(body.end() - 1)
            .end()
    }

    #[track_caller]
    pub fn defn_tokra_region_data(&self) -> DefnTokraRegionDataRef<'a> {
        match self.tokra_region_data {
            TokraRegionDataRef::Snippet(_) => todo!(),
            TokraRegionDataRef::Decl(_) => todo!(),
            TokraRegionDataRef::Defn(tokra_region_data) => tokra_region_data,
        }
    }
}
