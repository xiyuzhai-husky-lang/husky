/// primal doesn't care about uses and impls
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_print_utils::p;
use husky_word::Identifier;
use vec_like::AsVecMapEntry;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeSheet {
    module_path: ModulePath,
    module_symbols: VecMap<EntitySymbol>,
    impl_blocks: ImplBlockIdxRange,
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
        module_symbols: VecMap<EntitySymbol>,
        impl_blocks: ImplBlockIdxRange,
    ) -> Self {
        Self {
            module_path,
            module_symbols,
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
    pub fn module_symbols(&self) -> &VecMap<EntitySymbol> {
        &self.module_symbols
    }

    pub fn module_item_iter<'a>(&'a self) -> impl Iterator<Item = &'a ModuleItem> + 'a {
        self.module_symbols
            .iter()
            .filter_map(|module_symbol| module_symbol.module_item())
    }

    pub fn module_item_path_iter<'a>(&'a self) -> impl Iterator<Item = ModuleItemPath> + 'a {
        self.module_symbols.iter().filter_map(|module_symbol| {
            module_symbol
                .module_item()
                .map(|module_item| module_item.path())
        })
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn impl_blocks(&self) -> ImplBlockIdxRange {
        self.impl_blocks
    }
}

pub(crate) fn entity_tree_sheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<&EntityTreeSheet> {
    let crate_path = module_path.crate_path(db);
    let entity_tree_bundle = entity_tree_bundle(db, crate_path)
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
                &self
                    .module_path
                    .debug_with(db as &dyn VfsDb, include_all_fields),
            )
            .field(
                "module_symbols",
                &self.module_symbols.debug_with(db, include_all_fields),
            )
            .finish()
    }
}
