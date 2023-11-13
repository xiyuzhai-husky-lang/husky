#![feature(trait_upcasting)]
#![allow(incomplete_features)]
mod ancestry;
mod db;
mod error;
mod menu;
mod path;
#[cfg(test)]
mod tests;
mod utils;

pub use self::ancestry::*;
pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::path::*;

use either::*;
use husky_coword::Ident;
use husky_entity_kind::*;
use husky_vfs::*;
use salsa::DbWithJar;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(
    TypePath,
    prelude_ty_path,
    TraitPath,
    FugitivePath,
    TypeItemPath,
    TraitItemPath,
    TraitForTypeItemPath,
    TypeVariantPath,
    TypeImplBlockPath,
    TraitForTypeImplBlockPath,
    IllFormedImplBlockPath,
    AttrPath,
    item_path_menu,
);
