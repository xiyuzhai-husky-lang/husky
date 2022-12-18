#![feature(trait_upcasting)]
mod absolute;
mod db;
mod error;
mod implementation;
mod module_use;
mod node;
mod primal_module_use;
mod primal_tree;
mod submodule;
#[cfg(test)]
mod tests;

pub use absolute::*;
pub use db::EntityTreeDb;
pub use error::*;
pub use primal_module_use::*;

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
use primal_module_use::*;
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
    module_use_alls,
    module_use_ones,
    entity_node,
    parent_module,
    entity_card,
    entity_accessibility,
    primal_module_use_sheet,
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
