/// primal doesn't care about uses and impls
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_print_utils::p;
use husky_word::{IdentMap, IdentPairMap, Identifier};
use vec_like::{AsVecMapEntry, VecPairMap};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
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
        impl_blocks: Vec<ImplBlock>,
        use_expr_rules: UseExprRules,
        use_all_rules: UseAllRules,
        errors: Vec<EntityTreeError>,
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
        .ok_or_else(|| EntityTreeError::InvalidModulePath(module_path))
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntityTreeSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        f.debug_struct("EntityTreeSheet")
            .field(
                "module_path",
                &self.module_path.debug_with(db, include_all_fields),
            )
            .field(
                "module_specific_symbols",
                &self.symbols.debug_with(db, include_all_fields),
            )
            .finish()
    }
}
