use super::*;

impl<'a> ExprParser<'a> {
    pub(crate) fn alloc_expr(&mut self, expr: Expr) -> ExprIdx {
        self.expr_arena.alloc_one(expr)
    }

    pub(super) fn alloc_stmts(&mut self, stmts: Vec<Stmt>) -> StmtIdxRange {
        self.stmt_arena.alloc_batch(stmts)
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn alloc_expr_batch(
        &mut self,
        exprs: impl IntoIterator<Item = Expr>,
    ) -> ExprIdxRange {
        self.parser.expr_arena.alloc_batch(exprs)
    }

    pub(crate) fn alloc_expr(&mut self, expr: Expr) -> ExprIdx {
        self.parser.alloc_expr(expr)
    }

    pub(crate) fn alloc_pattern_expr(
        &mut self,
        expr: PatternExpr,
        env: PatternExprInfo,
    ) -> PatternExprIdx {
        self.parser.pattern_expr_page.alloc_one(expr, env)
    }

    pub(super) fn alloc_entity_path_expr(&mut self, expr: EntityPathExpr) -> EntityPathExprIdx {
        self.parser.entity_path_expr_arena.alloc_one(expr)
    }
}
