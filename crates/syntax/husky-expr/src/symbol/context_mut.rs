use super::*;

pub struct SymbolContextMut<'a> {
    module_prelude: ModulePrelude<'a>,
    symbol_sheet: SymbolSheet,
}

impl<'a> SymbolContextMut<'a> {
    pub fn new(module_prelude: ModulePrelude<'a>) -> Self {
        Self {
            module_prelude,
            symbol_sheet: SymbolSheet::new(),
        }
    }

    pub(crate) fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        self.symbol_sheet
            .resolve_ident(token_idx, ident)
            .map(|(local_symbol_idx, local_symbol_kind)| {
                Symbol::Local(local_symbol_idx, local_symbol_kind)
            })
            .or(self
                .module_prelude
                .resolve_ident(token_idx, ident)
                .map(|e| Symbol::Entity(e.entity_path())))
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
