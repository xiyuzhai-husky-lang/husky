use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntityTreeSheet {
    module_path: ModulePath,
    symbols: EntitySymbolTable,
    impl_blocks: Vec<ImplBlock>,
    use_expr_rules: UseExprRules,
    use_all_rules: UseAllRules,
    errors: Vec<EntityTreeError>,
}

impl vec_like::AsVecMapEntry for EntityTreeSheet {
    type K = ModulePath;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.module_path
    }

    fn key_ref(&self) -> &Self::K {
        &self.module_path
    }
}

impl EntityTreeSheet {
    pub(crate) fn new(
        module_path: ModulePath,
        symbols: EntitySymbolTable,
        impls: Vec<ImplBlock>,
        use_expr_rules: UseExprRules,
        use_all_rules: UseAllRules,
        errors: Vec<EntityTreeError>,
    ) -> Self {
        Self {
            module_path,
            symbols,
            impl_blocks: impls,
            use_expr_rules,
            use_all_rules,
            errors,
        }
    }

    pub fn module_symbols<'a>(&'a self) -> EntitySymbolTableRef<'a> {
        self.symbols.as_ref()
    }

    pub fn module_item_path_iter<'a>(
        &'a self,
        db: &'a dyn EntityTreeDb,
    ) -> impl Iterator<Item = ModuleItemPath> + 'a {
        self.symbols
            .data()
            .iter()
            .filter_map(|entry| match entry.symbol() {
                EntitySymbol::ModuleItem(symbol) => Some(symbol.path(db)),
                _ => None,
            })
    }

    pub fn errors(&self) -> &[EntityTreeError] {
        &self.errors
    }

    pub fn use_expr_rule_indexed_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (UseExprRuleIdx, &'a UseExprRule)> + 'a {
        self.use_expr_rules.indexed_iter()
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn impl_blocks(&self) -> &[ImplBlock] {
        &self.impl_blocks
    }
}

pub(crate) fn entity_tree_sheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<&EntityTreeSheet> {
    let crate_path = module_path.crate_path(db);
    let entity_tree_bundle = entity_tree_crate_bundle(db, crate_path)
        .as_ref()
        .map_err(|e| e.clone())?;
    entity_tree_bundle
        .get_sheet(module_path)
        .ok_or_else(|| DerivedEntityTreeError::InvalidModulePath(module_path).into())
}
