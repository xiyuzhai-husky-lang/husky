use crate::*;
use husky_entity_tree::ModuleSymbolContext;

#[derive(Debug, Clone)]
pub struct ExprContext<'a> {
    module_symbol_context: ModuleSymbolContext<'a>,
    expr_arena: &'a ExprArena,
    entity_path_expr_arena: &'a EntityPathExprArena,
    pattern_expr_region: &'a PatternExprRegion,
    stmt_arena: &'a StmtArena,
    symbol_region: &'a SymbolRegion,
}

impl<'a> ExprContext<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        module_symbol_context: ModuleSymbolContext<'a>,
        expr_region: ExprRegion,
    ) -> Self {
        Self {
            module_symbol_context,
            expr_arena: expr_region.expr_arena(db),
            entity_path_expr_arena: expr_region.entity_path_expr_arena(db),
            pattern_expr_region: expr_region.pattern_expr_region(db),
            stmt_arena: expr_region.stmt_arena(db),
            symbol_region: expr_region.symbol_region(db),
        }
    }

    pub fn entity_path_exprs(&self) -> &'a [EntityPathExpr] {
        self.entity_path_expr_arena.data()
    }

    pub fn exprs(&self) -> &'a [Expr] {
        self.expr_arena.data()
    }

    pub fn indexed_current_symbol_iter(
        &self,
    ) -> impl Iterator<Item = (CurrentSymbolIdx, &'a CurrentSymbol)> + 'a {
        self.symbol_region.indexed_current_symbol_iter()
    }
}

impl<'a> std::ops::Index<PatternSymbolIdx> for ExprContext<'a> {
    type Output = PatternSymbol;

    fn index(&self, index: PatternSymbolIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}

impl<'a> std::ops::Index<PatternExprIdx> for ExprContext<'a> {
    type Output = PatternExpr;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}

impl<'a> std::ops::Index<ExprIdx> for ExprContext<'a> {
    type Output = Expr;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_arena[index]
    }
}
