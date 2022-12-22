/// primal doesn't care about uses and impls
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_print_utils::p;
use husky_word::Identifier;
use vec_like::AsVecMapEntry;

use crate::*;

#[salsa::tracked(jar = EntitySymbolJar)]
pub struct EntityTreeSheet {
    pub module_path: ModulePath,
    #[return_ref]
    pub module_items: VecMap<EntityNode>,
}

impl EntityTreeSheet {
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
}

pub(crate) fn entity_tree_sheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<EntityTreeSheet> {
    todo!()
}
