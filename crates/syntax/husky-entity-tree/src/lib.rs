#![feature(trait_upcasting)]
mod db;
mod entity_use;
mod error;
mod implementation;
mod module_item;
mod node;
mod sheet;
mod submodule;
#[cfg(test)]
mod tests;
mod utils;

pub use db::EntityTreeDb;
pub use entity_use::*;
pub use error::*;

use error::EntityTreeError;
use husky_ast::AstIdx;
use husky_entity_card::EntityCard;
use husky_entity_path::*;
use husky_package_path::*;
use husky_vfs::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

use module_item::*;
use node::*;
use sheet::*;
use submodule::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    entity_tree_sheet,
    submodules,
    entity_node,
    parent_module,
    entity_card,
    entity_accessibility,
    module_use_exprs,
    module_items_map,
    all_modules_within_crate,
    crate_prelude,
    ModuleItemMap,
);

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTree {
    node: EntityNode,
    ast_idx: Option<AstIdx>,
    subentities: EntityTreeIdxRange,
}

pub type EntityTreeArena = Arena<EntityTree>;
pub type EntityTreeIdx = ArenaIdx<EntityTree>;
pub type EntityTreeIdxRange = ArenaIdxRange<EntityTree>;

impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntityTree {
    fn fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
