use super::*;

pub struct SymbolContextMut<'a> {
    module_symbol_context: ModuleSymbolContext<'a>,
    symbol_sheet: SymbolSheet,
}

impl<'a> SymbolContextMut<'a> {
    pub fn new(
        module_symbol_context: ModuleSymbolContext<'a>,
        parent_symbol_sheet: Option<&SymbolSheet>,
    ) -> Self {
        Self {
            module_symbol_context,
            symbol_sheet: SymbolSheet::new(parent_symbol_sheet),
        }
    }

    pub(crate) fn resolve_ident(
        &self,
        db: &dyn ExprDb,
        token_idx: TokenIdx,
        ident: Identifier,
    ) -> Option<Symbol> {
        self.symbol_sheet.resolve_ident(token_idx, ident).or(self
            .module_symbol_context
            .resolve_ident(token_idx, ident)
            .map(|e| Symbol::Entity(e.path(db))))
    }

    fn exprs(&self) -> &[Expr] {
        todo!()
    }

    pub(crate) fn into_expr_sheet(
        self,
        db: &dyn ExprDb,
        expr_arena: ExprArena,
        entity_path_expr_arena: EntityPathExprArena,
        pattern_expr_sheet: PatternExprSheet,
        stmt_arena: StmtArena,
    ) -> ExprSheet {
        ExprSheet::new(
            db,
            expr_arena,
            entity_path_expr_arena,
            pattern_expr_sheet,
            stmt_arena,
            self.symbol_sheet,
        )
    }

    pub(crate) fn define_variables(&mut self, variables: Vec<LocalSymbol>) -> LocalSymbolIdxRange {
        self.symbol_sheet.define_variables(variables)
    }
}
