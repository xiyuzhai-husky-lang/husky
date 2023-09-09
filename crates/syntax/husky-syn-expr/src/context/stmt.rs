use super::*;
use husky_ast::{AstIdx, AstSheet, AstTokenIdxRangeSheet, FugitiveBody};
use husky_entity_syn_tree::tokra_region::{HasSynDefnTokraRegion, SynDefnTokraRegionData};

pub struct SynStmtContext<'a> {
    expr_context: SynExprContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
    defn_tokra_region_data: SynDefnTokraRegionData<'a>,
}

impl<'a> std::ops::Deref for SynStmtContext<'a> {
    type Target = SynExprContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.expr_context
    }
}

impl<'a> std::ops::DerefMut for SynStmtContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expr_context
    }
}

impl<'a> SynStmtContext<'a> {
    pub fn new<P>(
        syn_node_path: P,
        decl_expr_region: SynExprRegion,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
        db: &'a dyn SynExprDb,
    ) -> Self
    where
        P: HasSynDefnTokraRegion + Into<ItemSynNodePath>,
    {
        let module_path = syn_node_path.module_path(db);
        let expr_context = SynExprContext::new(
            db,
            RegionPath::Defn(syn_node_path.into()),
            db.module_symbol_context(module_path).unwrap(),
            Some(decl_expr_region),
            allow_self_type,
            allow_self_value,
        );
        Self {
            expr_context,
            ast_sheet: db.ast_sheet(module_path).unwrap(),
            ast_token_idx_range_sheet: db.ast_token_idx_range_sheet(module_path).unwrap(),
            token_sheet_data: db
                .token_sheet_data(module_path)
                .expect("modules should be valid"),
            defn_tokra_region_data: syn_node_path.syn_defn_tokra_region(db).data(db),
        }
    }

    pub fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }

    pub(crate) fn expr_parser<'b>(
        &'b mut self,
        token_group_idx: TokenGroupIdx,
    ) -> SynExprParser<'a, &'b mut SynExprContext<'a>>
    where
        'a: 'b,
    {
        let token_stream = self.token_group_token_stream(token_group_idx, None);
        SynExprParser::new(self, None, token_stream)
    }

    pub fn ast_token_idx_range_sheet(&self) -> &'a AstTokenIdxRangeSheet {
        self.ast_token_idx_range_sheet
    }

    pub fn finish(self) -> SynExprRegion {
        self.expr_context.finish()
    }

    pub(crate) fn token_group_token_stream(
        &self,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: impl Into<Option<TokenStreamState>>,
    ) -> TokenStream<'a> {
        self.token_sheet_data
            .token_group_token_stream(token_group_idx, saved_stream_state)
    }
}
