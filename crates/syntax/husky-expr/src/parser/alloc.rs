use super::*;

impl<'a, S> ExprParser<'a, S>
where
    S: SymbolContextMut,
{
    pub(crate) fn alloc_expr(&mut self, expr: Expr) -> ExprIdx {
        self.expr_arena.alloc_one(expr)
    }

    pub(super) fn alloc_stmts(&mut self, stmts: Vec<Stmt>) -> StmtIdxRange {
        self.stmt_arena.alloc_batch(stmts)
    }
}

impl<'a, 'b, S> ExprParseContext<'a, 'b, S>
where
    S: SymbolContextMut,
{
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
        env: PatternInfo,
    ) -> PatternExprIdx {
        self.parser.pattern_expr_sheet.alloc_one(expr, env)
    }
}
