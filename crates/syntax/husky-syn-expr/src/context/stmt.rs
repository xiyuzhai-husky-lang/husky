use super::*;
use husky_ast::{AstIdx, AstSheet, AstTokenIdxRangeSheet, FugitiveBody};

pub struct StmtContext<'a> {
    expr_context: SynExprContext<'a>,
    ast_sheet: &'a AstSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
    env: Option<ExprEnvironment>,
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
        ast_sheet: &'a AstSheet,
        ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
        env: Option<ExprEnvironment>,
    ) -> Self {
        Self {
            expr_context: expr_parser,
            ast_sheet,
            ast_token_idx_range_sheet,
            env,
        }
    }

    pub fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }

    pub(crate) fn token_group_parser<'b>(
        &'b mut self,
        token_stream: TokenStream<'a>,
    ) -> SynExprParser<'a, &'b mut SynExprContext<'a>>
    where
        'a: 'b,
    {
        let env = self.env;
        SynExprParser::new(self, env, token_stream)
    }

    pub fn ast_token_idx_range_sheet(&self) -> &'a AstTokenIdxRangeSheet {
        self.ast_token_idx_range_sheet
    }

    pub fn finish(self) -> SynExprRegion {
        self.expr_context.finish()
    }
}
