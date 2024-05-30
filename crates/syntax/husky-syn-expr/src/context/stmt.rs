use super::*;
use husky_entity_tree::helpers::tokra_region::{
    crate_decl::CrateDeclTokraRegionDataRef, ItemDefnTokraRegionDataRef,
};
use husky_item_defn_ast::{ItemDefnAstArenaRef, ItemDefnAstIdx, ItemDefnAstIdxRange};

impl<'a> SynExprContext<'a> {
    pub(crate) fn parse_root_body(&mut self) -> SynExprIdx {
        let body = self.item_defn_tokra_region_data().root_body();
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
        match self.tokra_region_data {
            TokraRegionDataRef::CrateDecl(tokra_region_data) => {
                tokra_region_data.token_stream(regional_token_verse_idx)
            }
            TokraRegionDataRef::ItemDecl(_) => {
                unreachable!("it doesn't make sense to have regional_token_verse_idx in decl")
            }
            TokraRegionDataRef::ItemDefn(tokra_region_data) => {
                tokra_region_data.token_stream(regional_token_verse_idx)
            }
        }
    }

    pub(crate) fn asts(&self) -> ItemDefnAstArenaRef<'a> {
        self.item_defn_tokra_region_data().ast_arena()
    }

    pub(crate) fn ast_token_idx_range(
        &self,
        defn_ast_idx: ItemDefnAstIdx,
    ) -> RegionalTokenIdxRange {
        self.item_defn_tokra_region_data()
            .ast_token_idx_range(defn_ast_idx)
    }

    pub(crate) fn form_body_end(&self, body: ItemDefnAstIdxRange) -> RegionalTokenIdxRangeEnd {
        self.item_defn_tokra_region_data()
            .ast_token_idx_range(body.end() - 1)
            .end()
    }

    #[track_caller]
    pub fn crate_decl_tokra_region_data(&self) -> CrateDeclTokraRegionDataRef<'a> {
        match self.tokra_region_data {
            TokraRegionDataRef::CrateDecl(tokra_region_data) => tokra_region_data,
            TokraRegionDataRef::ItemDecl(_) | TokraRegionDataRef::ItemDefn(_) => {
                unreachable!()
            }
        }
    }

    #[track_caller]
    pub fn item_defn_tokra_region_data(&self) -> ItemDefnTokraRegionDataRef<'a> {
        match self.tokra_region_data {
            TokraRegionDataRef::CrateDecl(_) | TokraRegionDataRef::ItemDecl(_) => unreachable!(),
            TokraRegionDataRef::ItemDefn(tokra_region_data) => tokra_region_data,
        }
    }
}
