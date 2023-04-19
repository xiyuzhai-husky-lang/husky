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
        use_expr_rules: UseExprRules,
        use_all_rules: UseAllRules,
        errors: Vec<EntityTreeError>,
        impl_blocks: Vec<ImplBlock>,
    ) -> Self {
        Self {
            module_path,
            symbols,
            impl_blocks,
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

    pub fn impl_blocks(&self) -> &Vec<ImplBlock> {
        &self.impl_blocks
    }

    pub fn all_ty_impl_blocks<'a>(&'a self) -> impl Iterator<Item = TypeImplBlock> + 'a {
        self.impl_blocks
            .iter()
            .copied()
            .filter_map(|impl_block| match impl_block {
                ImplBlock::Type(impl_block) => Some(impl_block),
                ImplBlock::TraitForType(_) => None,
                ImplBlock::IllFormed(_) => None,
            })
    }

    pub fn all_trai_for_ty_impl_blocks<'a>(
        &'a self,
    ) -> impl Iterator<Item = TraitForTypeImplBlock> + 'a {
        self.impl_blocks
            .iter()
            .copied()
            .filter_map(|impl_block| match impl_block {
                ImplBlock::Type(_) => None,
                ImplBlock::TraitForType(impl_block) => Some(impl_block),
                ImplBlock::IllFormed(_) => None,
            })
    }

    pub fn all_ill_formed_impl_blocks<'a>(
        &'a self,
    ) -> impl Iterator<Item = IllFormedImplBlock> + 'a {
        self.impl_blocks
            .iter()
            .copied()
            .filter_map(|impl_block| match impl_block {
                ImplBlock::Type(_) => None,
                ImplBlock::TraitForType(_) => None,
                ImplBlock::IllFormed(impl_block) => Some(impl_block),
            })
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

#[test]
fn entity_tree_sheet_works() {
    DB::default().ast_expect_test_debug_with_db("entity_tree_sheet", |db, module_path| {
        entity_tree_sheet(db, module_path)
    })
}
