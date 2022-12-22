#![feature(trait_upcasting)]
mod bundle;
mod collector;
mod db;
mod error;
mod implementation;
mod module_symbol;
mod prelude;
mod presheet;
mod sheet;
mod submodule;
#[cfg(test)]
mod tests;
mod utils;

pub use bundle::*;
pub use db::EntitySymbolDb;
pub use error::*;

use collector::*;
use error::EntitySymbolError;
use husky_ast::*;
use husky_entity_kind::EntityKind;
use husky_entity_path::*;
use husky_package_path::*;
use husky_vfs::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use module_symbol::*;
use prelude::*;
use presheet::*;
use sheet::*;
use submodule::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntitySymbolDb)]
pub struct EntitySymbolJar(
    entity_tree_presheet,
    entity_tree_bundle,
    submodules,
    all_modules_within_crate,
    crate_prelude,
    EntitySymbolSheet,
    EntitySymbolBundle,
);
