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
#[cfg(test)]
use tests::*;
use vec_like::{AsVecMapEntry, VecMap};

#[salsa::jar(db = EntityTreeDb)]
pub struct EntitySymbolJar(
    entity_tree_presheet,
    entity_tree_bundle,
    submodules,
    all_modules_within_crate,
    EntitySymbolSheet,
    EntitySymbolBundle,
    crate_specific_prelude,
);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntityNode {
    Module {
        ident: Identifier,
        module_path: ModulePath,
    },
    ModuleItem {
        ident: Identifier,
        ast_idx: AstIdx,
        path: ModuleItemPath,
        variants: Option<VecMap<ModuleItemVariant>>,
    },
}

impl AsVecMapEntry for EntityNode {
    type K = Identifier;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            EntityNode::Module { ident, .. } | EntityNode::ModuleItem { ident, .. } => *ident,
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            EntityNode::Module { ident, .. } | EntityNode::ModuleItem { ident, .. } => ident,
        }
    }
}

impl salsa::DebugWithDb<dyn EntityTreeDb + '_> for EntityNode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityTreeDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Module { ident, module_path } => {
                f.debug_struct("Submodule").field("ident", ident).finish()
            }
            Self::ModuleItem {
                ident,
                ast_idx,
                path,
                variants,
            } => f
                .debug_struct("ModuleItem")
                .field(
                    "ident",
                    &ident.debug_with(db as &dyn WordDb, include_all_fields),
                )
                .field("ast_idx", ast_idx)
                .field("path", path)
                .field("variants", variants)
                .finish(),
        }
    }
}
impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntityNode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityTreeDb, include_all_fields)
    }
}
