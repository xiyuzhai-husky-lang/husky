use crate::*;
use husky_entity_tree::ModuleSymbolContext;

#[derive(Debug, Clone)]
pub struct ExprContext<'a> {
    module_symbol_context: ModuleSymbolContext<'a>,
    expr_arena: &'a ExprArena,
    entity_path_expr_arena: &'a EntityPathExprArena,
    pattern_expr_page: &'a PatternExprSubsheet,
    stmt_arena: &'a StmtArena,
    symbol_sheet: &'a SymbolSheet,
}

impl<'a> ExprContext<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        module_symbol_context: ModuleSymbolContext<'a>,
        expr_page: ExprPage,
    ) -> Self {
        Self {
            module_symbol_context,
            expr_arena: expr_page.expr_arena(db),
            entity_path_expr_arena: expr_page.entity_path_expr_arena(db),
            pattern_expr_page: expr_page.pattern_expr_page(db),
            stmt_arena: expr_page.stmt_arena(db),
            symbol_sheet: expr_page.symbol_sheet(db),
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
        self.symbol_sheet.indexed_current_symbol_iter()
    }
}

impl<'a> std::ops::Index<PatternSymbolIdx> for ExprContext<'a> {
    type Output = PatternSymbol;

    fn index(&self, index: PatternSymbolIdx) -> &Self::Output {
        &self.pattern_expr_page[index]
    }
}

impl<'a> std::ops::Index<PatternExprIdx> for ExprContext<'a> {
    type Output = PatternExpr;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_page[index]
    }
}

impl<'a> std::ops::Index<ExprIdx> for ExprContext<'a> {
    type Output = Expr;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_arena[index]
    }
}
