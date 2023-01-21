mod trai_item;
mod ty_as_trai_item;
mod ty_item;

pub use trai_item::*;
pub use ty_as_trai_item::*;
pub use ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AssociatedItemDefn {
    TypeItem(TypeItemDefn),
    TraitItem(TraitItemDefn),
    TypeAsTraitItem(TypeAsTraitItemDefn),
}

impl From<TypeAsTraitItemDefn> for AssociatedItemDefn {
    fn from(v: TypeAsTraitItemDefn) -> Self {
        Self::TypeAsTraitItem(v)
    }
}

impl From<TraitItemDefn> for AssociatedItemDefn {
    fn from(v: TraitItemDefn) -> Self {
        Self::TraitItem(v)
    }
}

impl From<TypeItemDefn> for AssociatedItemDefn {
    fn from(v: TypeItemDefn) -> Self {
        Self::TypeItem(v)
    }
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

    pub fn path(self, db: &dyn DefnDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemDefn::TypeItem(_) => todo!(),
            AssociatedItemDefn::TraitItem(_) => todo!(),
            AssociatedItemDefn::TypeAsTraitItem(_) => todo!(),
        }
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for AssociatedItemDefn {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DefnJar>>::as_jar_db(db);
        match self {
            AssociatedItemDefn::TypeItem(decl) => f
                .debug_tuple("TypeItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            AssociatedItemDefn::TraitItem(decl) => f
                .debug_tuple("TraitItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            AssociatedItemDefn::TypeAsTraitItem(decl) => f
                .debug_tuple("TypeAsTraitItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
