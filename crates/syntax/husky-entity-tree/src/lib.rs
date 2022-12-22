#![feature(trait_upcasting)]
mod bundle;
mod collector;
mod context;
mod db;
mod error;
mod implementation;
mod prelude;
mod presheet;
mod sheet;
mod submodule;
mod symbol;
#[cfg(test)]
mod tests;
mod utils;

pub use bundle::*;
pub use db::EntityTreeDb;
pub use error::*;

use collector::*;
use context::*;
use error::EntityTreeError;
use husky_ast::*;
use husky_entity_kind::EntityKind;
use husky_entity_path::*;
use husky_package_path::*;
use husky_vfs::*;
use husky_word::{Identifier, WordDb};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use prelude::*;
use presheet::*;
use salsa::DebugWithDb;
use sheet::*;
use submodule::*;
use symbol::*;
#[cfg(test)]
use tests::*;
use vec_like::{AsVecMapEntry, VecMap};

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    entity_tree_presheet,
    entity_tree_bundle,
    submodules,
    all_modules_within_crate,
    crate_specific_prelude,
);
