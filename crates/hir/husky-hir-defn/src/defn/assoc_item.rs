mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;
use husky_hir_decl::decl::AssocItemHirDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum AssocItemHirDefn {
    TypeItem(TypeItemHirDefn),
    TraitItem(TraitItemHirDefn),
    TraitForTypeItem(TraitForTypeItemHirDefn),
}

impl AssocItemHirDefn {
    pub fn path(self, db: &::salsa::Db) -> AssocItemPath {
        match self {
            AssocItemHirDefn::TypeItem(hir_defn) => hir_defn.path(db).into(),
            AssocItemHirDefn::TraitItem(_) => todo!(),
            AssocItemHirDefn::TraitForTypeItem(hir_defn) => hir_defn.path(db).into(),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> AssocItemHirDecl {
        match self {
            AssocItemHirDefn::TypeItem(hir_defn) => hir_defn.hir_decl(db).into(),
            AssocItemHirDefn::TraitItem(_) => todo!(),
            AssocItemHirDefn::TraitForTypeItem(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            AssocItemHirDefn::TypeItem(hir_defn) => hir_defn.hir_expr_region(db),
            AssocItemHirDefn::TraitItem(_) => todo!(),
            AssocItemHirDefn::TraitForTypeItem(hir_defn) => hir_defn.hir_expr_region(db),
        }
    }

    pub fn hir_expr_body_and_region(self, db: &::salsa::Db) -> Option<(HirExprIdx, HirExprRegion)> {
        match self {
            AssocItemHirDefn::TypeItem(hir_defn) => hir_defn.hir_expr_body_and_region(db),
            AssocItemHirDefn::TraitItem(_) => todo!(),
            AssocItemHirDefn::TraitForTypeItem(hir_defn) => hir_defn.hir_expr_body_and_region(db),
        }
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            AssocItemHirDefn::TypeItem(hir_defn) => hir_defn.dependencies(db),
            AssocItemHirDefn::TraitItem(hir_defn) => hir_defn.dependencies(db),
            AssocItemHirDefn::TraitForTypeItem(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            AssocItemHirDefn::TypeItem(hir_defn) => hir_defn.version_stamp(db),
            AssocItemHirDefn::TraitItem(hir_defn) => hir_defn.version_stamp(db),
            AssocItemHirDefn::TraitForTypeItem(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for AssocItemPath {
    type HirDefn = AssocItemHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        Some(match self {
            AssocItemPath::TypeItem(hir_decl) => hir_decl.hir_defn(db)?.into(),
            AssocItemPath::TraitItem(hir_decl) => hir_decl.hir_defn(db)?.into(),
            AssocItemPath::TraitForTypeItem(hir_decl) => hir_decl.hir_defn(db)?.into(),
        })
    }
}
