#![feature(trait_upcasting)]
mod bundle;
mod collector;
mod db;
mod entity_use;
mod error;
mod implementation;
mod module_item;
mod node;
mod presheet;
mod sheet;
mod submodule;
#[cfg(test)]
mod tests;
mod utils;

pub use bundle::*;
pub use db::EntityTreeDb;
pub use error::*;

use collector::*;
use entity_use::*;
use error::EntityTreeError;
use husky_ast::*;
use husky_entity_kind::EntityKind;
use husky_entity_path::*;
use husky_package_path::*;
use husky_vfs::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use module_item::*;
use node::*;
use presheet::*;
use sheet::*;
use submodule::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    entity_tree_presheet,
    entity_tree_bundle,
    submodules,
    all_modules_within_crate,
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
