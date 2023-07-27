mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum AssociatedItemHirDefn {
    TypeItem(TypeItemHirDefn),
    TraitItem(TraitItemHirDefn),
    TraitForTypeItem(TraitForTypeItemHirDefn),
}

impl AssociatedItemHirDefn {
    pub fn hir_decl(self, db: &dyn HirDefnDb) -> AssociatedItemHirDecl {
        match self {
            AssociatedItemHirDefn::TypeItem(hir_defn) => hir_defn.hir_decl(db).into(),
            AssociatedItemHirDefn::TraitItem(_) => todo!(),
            AssociatedItemHirDefn::TraitForTypeItem(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            AssociatedItemHirDefn::TypeItem(hir_defn) => hir_defn.hir_expr_region(db),
            AssociatedItemHirDefn::TraitItem(_) => todo!(),
            AssociatedItemHirDefn::TraitForTypeItem(hir_defn) => Some(hir_defn.hir_expr_region(db)),
        }
    }

    pub fn path(self, _db: &dyn HirDefnDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemHirDefn::TypeItem(_) => todo!(),
            AssociatedItemHirDefn::TraitItem(_) => todo!(),
            AssociatedItemHirDefn::TraitForTypeItem(_) => todo!(),
        }
    }
}

impl HasHirDefn for AssociatedItemPath {
    type HirDefn = AssociatedItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        match self {
            AssociatedItemPath::TypeItem(hir_decl) => hir_decl.hir_defn(db).into(),
            AssociatedItemPath::TraitItem(hir_decl) => hir_decl.hir_defn(db).into(),
            AssociatedItemPath::TraitForTypeItem(hir_decl) => hir_decl.hir_defn(db).into(),
        }
    }
}
