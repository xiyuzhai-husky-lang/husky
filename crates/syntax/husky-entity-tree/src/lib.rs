#![feature(stmt_expr_attributes)]
#![feature(trait_upcasting)]
mod associated_item;
mod bundle;
mod collector;
mod context;
mod db;
mod error;
mod expr;
mod im;
mod prelude;
mod presheet;
mod region_path;
mod sheet;
mod subentity;
mod submodule;
mod symbol;
#[cfg(test)]
mod tests;
mod utils;

pub use associated_item::*;
pub use associated_item::*;
pub use bundle::*;
pub use db::EntityTreeDb;
pub use error::EntityTreeError;
pub use error::*;
pub use expr::*;
pub use im::*;
pub use prelude::*;
pub use presheet::*;
pub use region_path::*;
pub use sheet::*;
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
use salsa::DebugWithDb;
use subentity::*;
#[cfg(test)]
use tests::*;
use vec_like::{AsVecMapEntry, VecMap};

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    SubmoduleSymbol,
    ModuleItemSymbol,
    UseSymbol,
    Impl,
    AssociatedItem,
    entity_tree_presheet,
    entity_tree_crate_bundle,
    submodules,
    module_subentity_path,
    all_modules_within_crate,
    crate_specific_prelude,
    ty_impls,
    ty_associated_items,
    impl_associated_items,
);
