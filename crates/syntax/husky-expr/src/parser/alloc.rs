use super::*;

impl<'a, 'b, 'c> ExprParser<'a, 'b> {
    pub(crate) fn alloc_expr_batch(
        &mut self,
        exprs: impl IntoIterator<Item = Expr>,
    ) -> ExprIdxRange {
        self.expr_arena.alloc_batch(exprs)
    }

    pub(crate) fn alloc_expr(&mut self, expr: Expr) -> ExprIdx {
        self.expr_arena.alloc_one(expr)
    }

    pub(crate) fn alloc_pattern_expr(&mut self, expr: PatternExpr) -> PatternExprIdx {
        self.pattern_expr_arena.alloc_one(expr)
    }
}
