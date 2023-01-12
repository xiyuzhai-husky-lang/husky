/// primal doesn't care about uses and impls
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_print_utils::p;
use husky_word::{IdentMap, IdentPairMap, Identifier};
use vec_like::{AsVecMapEntry, VecPairMap};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeModuleSheet {
    module_path: ModulePath,
    symbols: IdentMap<EntitySymbolEntry>,
    impl_blocks: Vec<ImplBlock>,
}

impl vec_like::AsVecMapEntry for EntityTreeModuleSheet {
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

impl EntityTreeModuleSheet {
    pub(crate) fn new(
        module_path: ModulePath,
        symbols: IdentMap<EntitySymbolEntry>,
        impl_blocks: Vec<ImplBlock>,
    ) -> Self {
        Self {
            module_path,
            symbols,
            impl_blocks,
        }
    }

    // pub(crate) fn get(&self, entity_path: EntityPath) -> Option<&EntitySymbol> {
    //     self.arena
    //         .data()
    //         .iter()
    //         .find(|node| node.entity_path() == entity_path)
    // }

    // pub(crate) fn top_level_entities<'a>(
    //     &'a self,
    // ) -> impl Iterator<Item = (EntitySymbolIdx, Accessibility, EntityCard, EntityPath)> + 'a {
    //     self[&self.top_level_entities_idx_range]
    //         .iter()
    //         .enumerate()
    //         .map(|(i, tree)| {
    //             (
    //                 self.top_level_entities_idx_range.start() + i,
    //                 tree.node.accessibility(),
    //                 tree.node.card(),
    //                 tree.node.entity_path(),
    //             )
    //         })
    // }
    pub fn module_symbols(&self) -> &[EntitySymbolEntry] {
        &self.symbols
    }

    // pub fn module_item_iter<'a>(&'a self) -> impl Iterator<Item = &'a ModuleItem> + 'a {
    //     self.module_specific_symbols
    //         .iter()
    //         .filter_map(|module_symbol| module_symbol.module_item())
    // }

    pub fn module_item_path_iter<'a>(
        &'a self,
        db: &'a dyn EntityTreeDb,
    ) -> impl Iterator<Item = ModuleItemPath> + 'a {
        self.symbols
            .iter()
            .filter_map(|entry| match entry.symbol() {
                EntitySymbol::CrateRoot(_) => todo!(),
                EntitySymbol::Submodule(_) => todo!(),
                EntitySymbol::ModuleItem(symbol) => Some(symbol.path(db)),
                EntitySymbol::Use(_) => todo!(),
            })
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn impl_blocks(&self) -> &[ImplBlock] {
        &self.impl_blocks
    }
}

pub(crate) fn module_entity_tree(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<&EntityTreeModuleSheet> {
    let crate_path = module_path.crate_path(db);
    let entity_tree_bundle = entity_tree_crate_bundle(db, crate_path)
        .as_ref()
        .map_err(|e| e.clone())?;
    entity_tree_bundle
        .get_sheet(module_path)
        .ok_or_else(|| EntityTreeError::InvalidModulePath(module_path))
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntityTreeModuleSheet {
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
                &(&self.symbols).debug_with(db, include_all_fields),
            )
            .finish()
    }
}
