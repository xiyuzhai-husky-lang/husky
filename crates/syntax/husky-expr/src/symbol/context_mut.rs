use husky_vfs::ModulePath;

use super::*;

pub struct SymbolContextMut<'a> {
    module_symbol_context: ModuleSymbolContext<'a>,
    symbol_sheet: SymbolSheet,
}

impl<'a> SymbolContextMut<'a> {
    pub fn new(
        module_symbol_context: ModuleSymbolContext<'a>,
        parent_symbol_sheet: Option<&SymbolSheet>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> Self {
        Self {
            module_symbol_context,
            symbol_sheet: SymbolSheet::new(parent_symbol_sheet, allow_self_type, allow_self_value),
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

    pub(crate) fn into_expr_page(
        self,
        db: &dyn ExprDb,
        path: ExprPath,
        expr_arena: ExprArena,
        entity_path_expr_arena: EntityPathExprArena,
        pattern_expr_page: PatternExprSubsheet,
        stmt_arena: StmtArena,
    ) -> ExprPage {
        ExprPage::new(
            db,
            path,
            expr_arena,
            entity_path_expr_arena,
            pattern_expr_page,
            stmt_arena,
            self.symbol_sheet,
        )
    }

    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSymbol>,
    ) -> CurrentSymbolIdxRange {
        self.symbol_sheet.define_symbols(variables)
    }

    pub(crate) fn symbol_sheet(&self) -> &SymbolSheet {
        &self.symbol_sheet
    }
}
