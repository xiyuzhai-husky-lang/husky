/// primal doesn't care about uses and impls
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_print_utils::p;
use husky_word::Identifier;
use vec_like::AsVecMapEntry;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeSheet {
    module_path: ModulePath,
    module_items: Vec<ModuleSymbol>,
}

impl AsVecMapEntry<ModulePath> for EntityTreeSheet {
    fn key(&self) -> ModulePath
    where
        ModulePath: Copy,
    {
        self.module_path
    }

    fn key_ref(&self) -> &ModulePath {
        &self.module_path
    }
}

impl EntityTreeSheet {
    // pub(crate) fn get(&self, entity_path: EntityPath) -> Option<&EntityTree> {
    //     self.arena
    //         .data()
    //         .iter()
    //         .find(|node| node.entity_path() == entity_path)
    // }

    // pub(crate) fn top_level_entities<'a>(
    //     &'a self,
    // ) -> impl Iterator<Item = (EntityTreeIdx, Accessibility, EntityCard, EntityPath)> + 'a {
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
}
