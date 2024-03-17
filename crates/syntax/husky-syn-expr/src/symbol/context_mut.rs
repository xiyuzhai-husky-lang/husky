use husky_vfs::ModulePath;

use super::*;

pub struct SynSymbolContextMut<'a> {
    module_symbol_context: ModuleSymbolContext<'a>,
    variable_region: VariableRegionData,
}

impl<'a> SynSymbolContextMut<'a> {
    pub fn new(
        module_symbol_context: ModuleSymbolContext<'a>,
        parent_symbol_region: Option<&VariableRegionData>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> Self {
        Self {
            module_symbol_context,
            variable_region: VariableRegionData::new(
                parent_symbol_region,
                allow_self_type,
                allow_self_value,
            ),
        }
    }

    pub(crate) fn resolve_ident(
        &self,
        db: &::salsa::Db,
        reference_module_path: ModulePath,
        token_idx: RegionalTokenIdx,
        ident: Ident,
    ) -> Option<Symbol> {
        self.variable_region.resolve_ident(token_idx, ident).or(self
            .module_symbol_context
            .resolve_ident(db, reference_module_path, token_idx, ident)
            .map(|e| Symbol::PrincipalEntity(e.principal_entity_path(db))))
    }

    pub(crate) fn into_expr_region(
        self,
        db: &::salsa::Db,
        parent: Option<SynExprRegion>,
        path: SynNodeRegionPath,
        expr_arena: SynExprArena,
        principal_item_path_expr_arena: SynPrincipalEntityPathExprArena,
        pattern_expr_region: SynPatternExprRegion,
        stmt_arena: SynStmtArena,
        syn_pattern_expr_roots: Vec<SynPatternRoot>,
        syn_expr_roots: Vec<SynExprRoot>,
        has_self_lifetime: bool,
        has_self_place: bool,
    ) -> SynExprRegion {
        SynExprRegion::new(
            db,
            SynExprRegionData::new(
                parent,
                path,
                expr_arena,
                principal_item_path_expr_arena,
                stmt_arena,
                pattern_expr_region,
                self.variable_region,
                syn_pattern_expr_roots,
                syn_expr_roots,
                has_self_lifetime,
                has_self_place,
            ),
        )
    }

    pub(crate) fn define_symbol(
        &mut self,
        variable: CurrentSynSymbolEntry,
        ty_constraint: Option<SyndicateTypeConstraint>,
    ) -> CurrentSynSymbolIdx {
        self.variable_region.define_symbol(variable, ty_constraint)
    }

    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSynSymbolEntry>,
        ty_constraint: Option<SyndicateTypeConstraint>,
    ) -> CurrentSynSymbolIdxRange {
        self.variable_region
            .define_symbols(variables, ty_constraint)
    }

    pub(crate) fn symbol_region(&self) -> &VariableRegionData {
        &self.variable_region
    }

    pub(crate) fn set_lambda_variable_access_end(
        &mut self,
        var: CurrentSynSymbolIdx,
        access_end: RegionalTokenIdxRangeEnd,
    ) {
        self.variable_region
            .set_lambda_variable_access_end(var, access_end)
    }
}
