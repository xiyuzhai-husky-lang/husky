#![feature(trait_upcasting)]
mod bundle;
mod collector;
mod context;
mod db;
mod error;
mod implementation;
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
use context::*;
use error::EntitySymbolError;
use husky_ast::*;
use husky_entity_kind::EntityKind;
use husky_entity_path::*;
use husky_package_path::*;
use husky_vfs::*;
use husky_word::{Identifier, WordDb};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use presheet::*;
use sheet::*;
use submodule::*;
#[cfg(test)]
use tests::*;
use vec_like::VecMap;

#[salsa::jar(db = EntitySymbolDb)]
pub struct EntitySymbolJar(
    entity_tree_presheet,
    entity_tree_bundle,
    submodules,
    all_modules_within_crate,
    EntitySymbolSheet,
    EntitySymbolBundle,
);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ModuleSymbol {
    Submodule {
        ident: Identifier,
    },
    ModuleItem {
        ident: Identifier,
        ast_idx: AstIdx,
        path: ModuleItemPath,
        variants: Option<VecMap<Identifier, ModuleItemVariant>>,
    },
}

impl salsa::DebugWithDb<dyn EntitySymbolDb + '_> for ModuleSymbol {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntitySymbolDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Submodule { ident } => f.debug_struct("Submodule").field("ident", ident).finish(),
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
impl<Db: EntitySymbolDb> salsa::DebugWithDb<Db> for ModuleSymbol {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntitySymbolDb, include_all_fields)
    }
}
