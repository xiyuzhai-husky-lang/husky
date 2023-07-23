mod associated_item;
mod decr;
mod impl_block;
mod module_item;
mod ty_variant;

pub use self::associated_item::*;
pub use self::decr::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::ty_variant::*;

use crate::{db::*, *};

pub trait HasHirDecl {
    type HirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl;
}

pub enum HirDecl {}
