#![feature(trait_upcasting)]
mod absolute;
mod builder;
mod db;
mod error;
mod implementation;
mod module_use;
mod node;
mod submodule;
#[cfg(test)]
mod tests;

pub use absolute::*;
pub use db::EntityTreeDb;
pub use error::*;
pub use module_use::*;

use builder::*;
use error::EntityTreeError;
use husky_accessibility::Accessibility;
use husky_ast::AstIdx;
use husky_entity_card::EntityCard;
use husky_entity_path::*;
use husky_package_path::*;
use husky_vfs::VfsResult;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use implementation::ImplementationMap;
use module_use::*;
use node::*;
use submodule::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    absolute_entity_path,
    entity_tree_sheet,
    submodules,
    module_level_use_alls,
    module_level_use_ones,
    entity_node,
    parent_module,
    entity_card,
    entity_accessibility,
    module_use_sheet,
);

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeSheet {
    arena: EntityTreeArena,
    top_level_entities: EntityTreeIdxRange,
    isolate_entities: EntityTreeIdxRange,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTree {
    node: EntityNode,
    ast_idx: Option<AstIdx>,
    subentities: EntityTreeIdxRange,
}

pub type EntityTreeArena = Arena<EntityTree>;
pub type EntityTreeIdx = ArenaIdx<EntityTree>;
pub type EntityTreeIdxRange = ArenaIdxRange<EntityTree>;

impl EntityTreeSheet {
    fn get(&self, entity_path: EntityPath) -> Option<&EntityTree> {
        self.arena
            .data()
            .iter()
            .find(|node| node.entity_path() == entity_path)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn entity_tree_sheet(db: &dyn EntityTreeDb, module: EntityPath) -> VfsResult<EntityTreeSheet> {
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    EntityTreeBuilder::new(db, module, ast_sheet).build()
}
