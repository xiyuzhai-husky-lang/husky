use husky_vfs::ModulePath;

use super::*;

pub struct SymbolContextMut<'a> {
    module_symbol_context: ModuleSymbolContext<'a>,
    symbol_region: SymbolRegion,
}

impl<'a> SymbolContextMut<'a> {
    pub fn new(
        module_symbol_context: ModuleSymbolContext<'a>,
        parent_symbol_region: Option<&SymbolRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> Self {
        Self {
            module_symbol_context,
            symbol_region: SymbolRegion::new(
                parent_symbol_region,
                allow_self_type,
                allow_self_value,
            ),
        }
    }

    pub(crate) fn resolve_ident(
        &self,
        db: &dyn ExprDb,
        reference_module_path: ModulePath,
        token_idx: TokenIdx,
        ident: Ident,
    ) -> Option<Symbol> {
        self.symbol_region.resolve_ident(token_idx, ident).or(self
            .module_symbol_context
            .resolve_ident(db, reference_module_path, token_idx, ident)
            .map(|e| Symbol::PrincipalEntity(e.path(db))))
    }

    fn exprs(&self) -> &[SynExpr] {
        todo!()
    }

    pub(crate) fn into_expr_region(
        self,
        db: &dyn ExprDb,
        parent: Option<ExprRegion>,
        path: RegionPath,
        expr_arena: ExprArena,
        principal_entity_path_expr_arena: PrincipalEntityPathExprArena,
        pattern_expr_region: PatternExprRegion,
        stmt_arena: StmtArena,
        roots: Vec<ExprRoot>,
    ) -> ExprRegion {
        ExprRegion::new(
            db,
            ExprRegionData::new(
                parent,
                path,
                expr_arena,
                principal_entity_path_expr_arena,
                stmt_arena,
                pattern_expr_region,
                self.symbol_region,
                roots,
            ),
        )
    }

    pub(crate) fn define_symbol(
        &mut self,
        variable: CurrentSymbol,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> CurrentSymbolIdx {
        self.symbol_region.define_symbol(variable, ty_constraint)
    }

    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSymbol>,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> CurrentSymbolIdxRange {
        self.symbol_region.define_symbols(variables, ty_constraint)
    }

    pub(crate) fn symbol_region(&self) -> &SymbolRegion {
        &self.symbol_region
    }
}
