#![feature(trait_upcasting)]
mod absolute;
mod db;
mod entity_use;
mod error;
mod implementation;
mod node;
mod primal_tree;
mod submodule;
#[cfg(test)]
mod tests;
mod utils;

pub use absolute::*;
pub use db::EntityTreeDb;
pub use entity_use::*;
pub use error::*;

use error::EntityTreeError;
use husky_accessibility::Accessibility;
use husky_ast::AstIdx;
use husky_entity_card::EntityCard;
use husky_entity_path::*;
use husky_package_path::*;
use husky_vfs::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use implementation::ImplementationMap;
use node::*;
use primal_tree::*;
use submodule::*;
#[cfg(test)]
use tests::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    absolute_entity_path,
    primal_entity_tree_sheet,
    submodules,
    entity_node,
    parent_module,
    entity_card,
    entity_accessibility,
    module_use_exprs,
    module_use_atoms,
    module_use_sheet,
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
