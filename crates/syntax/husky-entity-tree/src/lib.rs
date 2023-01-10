#![feature(trait_upcasting)]
mod associated_item;
mod collector;
mod context;
mod crate_bundle;
mod db;
mod error;
mod impl_block;
mod module_sheet;
mod prelude;
mod presheet;
mod principal_path;
mod subentity;
mod submodule;
mod symbol;
#[cfg(test)]
mod tests;
mod utils;

pub use associated_item::*;
pub use associated_item::*;
pub use crate_bundle::*;
pub use db::EntityTreeDb;
pub use error::EntityTreeError;
pub use error::*;
pub use impl_block::*;
pub use module_sheet::*;
pub use prelude::*;
pub use principal_path::*;
pub use submodule::*;
pub use symbol::*;

use collector::*;
use context::*;
use husky_accessibility::*;
use husky_ast::*;
use husky_entity_path::*;
use husky_entity_taxonomy::EntityKind;
use husky_package_path::*;
use husky_vfs::*;
use husky_word::{Identifier, WordDb};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use presheet::*;
use salsa::DebugWithDb;
use subentity::*;
#[cfg(test)]
use tests::*;
use vec_like::{AsVecMapEntry, VecMap};

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    entity_tree_presheet,
    entity_tree_crate_bundle,
    submodules,
    module_subentity_path,
    all_modules_within_crate,
    crate_specific_prelude,
    ty_impl_blocks,
    ty_associated_items,
);
