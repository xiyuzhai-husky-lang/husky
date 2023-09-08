use super::*;
use husky_ast::{AstIdx, AstSheet, AstTokenIdxRangeSheet, FugitiveBody};

pub struct StmtContext<'a> {
    expr_context: SynExprContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
}

impl<'a> std::ops::Deref for StmtContext<'a> {
    type Target = SynExprContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.expr_context
    }
}

impl<'a> std::ops::DerefMut for StmtContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expr_context
    }
}

impl<'a> StmtContext<'a> {
    pub fn new(
        expr_parser: SynExprContext<'a>,
        module_path: ModulePath,
        db: &'a dyn SynExprDb,
    ) -> Self {
        Self {
            expr_context: expr_parser,
            ast_sheet: db.ast_sheet(module_path).unwrap(),
            ast_token_idx_range_sheet: db.ast_token_idx_range_sheet(module_path).unwrap(),
            token_sheet_data: db
                .token_sheet_data(module_path)
                .expect("modules should be valid"),
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
