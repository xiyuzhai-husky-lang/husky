#![feature(stmt_expr_attributes)]
#![feature(trait_upcasting)]
mod associated_item;
mod bundle;
mod collector;
mod context;
mod db;
mod error;
mod expr;
mod impl_block;
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

pub use self::associated_item::*;
pub use self::bundle::*;
pub use self::db::EntityTreeDb;
pub use self::error::*;
pub use self::expr::*;
pub use self::impl_block::*;
pub use self::prelude::*;
pub use self::presheet::*;
pub use self::region_path::*;
pub use self::sheet::*;
pub use self::symbol::*;

use self::collector::*;
use self::context::*;
use self::subentity::*;
use self::submodule::*;
#[cfg(test)]
use self::tests::*;
use husky_accessibility::*;
use husky_ast::*;
use husky_entity_path::*;
use husky_entity_taxonomy::EntityKind;
use husky_vfs::*;
use husky_word::{Ident, WordDb};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use salsa::DebugWithDb;
use vec_like::AsVecMapEntry;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    SubmoduleSymbol,
    ModuleItemSymbol,
    UseSymbol,
    TypeImplBlock,
    TraitForTypeImplBlock,
    IllFormedImplBlock,
    AssociatedItem,
    TypeAssociatedItem,
    entity_tree_presheet,
    entity_tree_crate_bundle,
    submodules,
    module_subentity_path,
    all_modules_within_crate,
    crate_specific_prelude,
    ty_impl_blocks,
    ty_items,
    ty_impl_block_items,
    trai_for_ty_impl_block_items,
);
