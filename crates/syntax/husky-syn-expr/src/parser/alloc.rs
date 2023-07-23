use super::*;

impl<'a> ExprParser<'a> {
    pub(crate) fn alloc_expr(&mut self, expr: SynExpr) -> SynExprIdx {
        self.expr_arena.alloc_one(expr)
    }

    pub(super) fn alloc_stmts(&mut self, stmts: Vec<SynStmt>) -> SynStmtIdxRange {
        self.stmt_arena.alloc_batch(stmts)
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn alloc_expr_batch(
        &mut self,
        exprs: impl IntoIterator<Item = SynExpr>,
    ) -> SynExprIdxRange {
        self.parser.expr_arena.alloc_batch(exprs)
    }

    pub(crate) fn alloc_expr(&mut self, expr: SynExpr) -> SynExprIdx {
        self.parser.alloc_expr(expr)
    }

    pub(crate) fn alloc_pattern_expr(
        &mut self,
        expr: PatternSynExpr,
        env: PatternSynExprInfo,
    ) -> PatternSynExprIdx {
        self.parser
            .pattern_expr_region
            .alloc_one_pattern_expr(expr, env)
    }

    pub(crate) fn alloc_entity_path_expr(
        &mut self,
        expr: PrincipalEntityPathExpr,
    ) -> PrincipalEntityPathExprIdx {
        self.parser.principal_entity_path_expr_arena.alloc_one(expr)
    }
}
