mod trai_item;
mod ty_as_trai_item;
mod ty_item;

pub use trai_item::*;
pub use ty_as_trai_item::*;
pub use ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum AssociatedItemDefn {
    TypeItem(TypeItemDefn),
    TraitItem(TraitItemDefn),
    TypeAsTraitItem(TypeAsTraitItemDefn),
}

impl AssociatedItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> AssociatedItemDecl {
        match self {
            AssociatedItemDefn::TypeItem(defn) => defn.decl(db).into(),
            AssociatedItemDefn::TraitItem(_) => todo!(),
            AssociatedItemDefn::TypeAsTraitItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            AssociatedItemDefn::TypeItem(defn) => defn.expr_region(db),
            AssociatedItemDefn::TraitItem(_) => todo!(),
            AssociatedItemDefn::TypeAsTraitItem(defn) => Some(defn.expr_region(db)),
        }
    }

    pub fn path(self, _db: &dyn DefnDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemDefn::TypeItem(_) => todo!(),
            AssociatedItemDefn::TraitItem(_) => todo!(),
            AssociatedItemDefn::TypeAsTraitItem(_) => todo!(),
        }
    }
}
