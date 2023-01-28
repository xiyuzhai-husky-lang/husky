use crate::*;
use husky_entity_tree::ModuleSymbolContext;

#[derive(Debug, Clone)]
pub struct ExprContext<'a> {
    module_symbol_context: ModuleSymbolContext<'a>,
    expr_region_data: &'a ExprRegionData,
}

impl<'a> ExprContext<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        module_symbol_context: ModuleSymbolContext<'a>,
        expr_region: ExprRegion,
    ) -> Self {
        Self {
            module_symbol_context,
            expr_region_data: expr_region.data(db),
        }
    }

    pub fn entity_path_exprs(&self) -> &'a [EntityPathExpr] {
        self.expr_region_data.entity_path_expr_arena().data()
    }

    pub fn exprs(&self) -> &'a [Expr] {
        self.expr_region_data.expr_arena().data()
    }

    pub fn indexed_current_symbol_iter(
        &self,
    ) -> impl Iterator<Item = (CurrentSymbolIdx, &'a CurrentSymbol)> + 'a {
        self.expr_region_data
            .symbol_region()
            .indexed_current_symbol_iter()
    }
}

impl<'a> std::ops::Index<PatternSymbolIdx> for ExprContext<'a> {
    type Output = PatternSymbol;

    fn index(&self, index: PatternSymbolIdx) -> &Self::Output {
        &self.expr_region_data.pattern_expr_region()[index]
    }
}

impl<'a> std::ops::Index<PatternExprIdx> for ExprContext<'a> {
    type Output = PatternExpr;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.expr_region_data.pattern_expr_region()[index]
    }
}

impl<'a> std::ops::Index<ExprIdx> for ExprContext<'a> {
    type Output = Expr;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_region_data.expr_arena()[index]
    }
}
