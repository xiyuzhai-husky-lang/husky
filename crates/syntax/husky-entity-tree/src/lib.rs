#![feature(stmt_expr_attributes)]
#![feature(trait_upcasting)]
mod bundle;
mod collector;
mod context;
mod db;
mod error;
mod expr;
mod node;
mod prelude;
mod presheet;
mod region_path;
mod sheet;
mod subentity;
mod submodule;
mod symbol;
mod table;
#[cfg(test)]
mod tests;
mod utils;

pub use self::bundle::*;
pub use self::db::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::node::*;
pub use self::prelude::*;
pub use self::presheet::*;
pub use self::region_path::*;
pub use self::sheet::*;
pub use self::symbol::*;
pub use self::table::*;

use self::collector::*;
use self::context::*;
use self::subentity::*;
use self::submodule::*;
#[cfg(test)]
use self::tests::*;
use husky_ast::*;
use husky_entity_path::*;
use husky_entity_taxonomy::EntityKind;
use husky_scope::*;
use husky_scope_expr::*;
use husky_vfs::*;
use husky_word::{Ident, WordDb};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use salsa::DebugWithDb;
use vec_like::AsVecMapEntry;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    SubmoduleNodePath,
    SubmoduleNode,
    submodule_node,
    ModuleItemNode,
    UseSymbol,
    // module items
    TraitNodePath,
    trai_node,
    TypeNodePath,
    ty_node,
    FugitiveNodePath,
    fugitive_node,
    // ty variant
    TypeVariantNodePath,
    // associated items
    TypeItemNodePath,
    TypeItemNode,
    TraitItemNodePath,
    TraitItemNode,
    TraitForTypeItemNodePath,
    TraitForTypeItemNode,
    TypeVariantNode,
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
    // variants
    TypeVariant,
    ty_path_variants,
    // impl blocks
    // - type impl block
    TypeImplBlockNode,
    // - trait for type impl block
    TraitForTypeImplBlockNode,
    // - ill formed impl block
    IllFormedImplBlockNode,
);
