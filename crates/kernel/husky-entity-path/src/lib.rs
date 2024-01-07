#![allow(incomplete_features)]
mod ancestry;
mod db;
mod error;
pub mod menu;
mod path;
pub mod region;
#[cfg(test)]
mod tests;
mod utils;

pub use self::ancestry::*;

pub use self::error::*;
pub use self::menu::*;
pub use self::path::*;

use either::*;
use husky_coword::Ident;
use husky_entity_kind::*;
use husky_vfs::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(
    ItemPathId,
    crate::path::major_item::ty::prelude_ty_path,
    crate::path::major_item::trai::prelude_trai_path,
    item_path_menu,
);
