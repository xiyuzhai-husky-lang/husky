pub mod assoc_ritchie;
pub mod assoc_ty;
pub mod assoc_val;
pub mod method_ritchie;

use self::assoc_ritchie::*;
use self::assoc_ty::*;
use self::assoc_val::*;
use self::method_ritchie::*;
use super::*;
use husky_entity_path::path::assoc_item::trai_for_ty_item::TraitForTypeItemPath;
use husky_hir_decl::decl::TraitForTypeItemHirDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemHirDefn {
    AssocRitchie(TraitForTypeAssocRitchieHirDefn),
    MethodFn(TraitForTypeMethodRitchieHirDefn),
    AssocType(TraitForTypeAssocTypeHirDefn),
    AssocVal(TraitForTypeAssocValHirDefn),
}

impl From<TraitForTypeItemHirDefn> for HirDefn {
    fn from(hir_defn: TraitForTypeItemHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TraitForTypeItemHirDefn {
    pub fn path(self, db: &::salsa::Db) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::AssocType(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::AssocVal(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> TraitForTypeItemHirDecl {
        match self {
            TraitForTypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::AssocType(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::AssocVal(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            TraitForTypeItemHirDefn::AssocRitchie(slf) => {
                slf.hir_eager_expr_region(db).map(Into::into)
            }
            TraitForTypeItemHirDefn::MethodFn(slf) => slf.hir_eager_expr_region(db).map(Into::into),
            TraitForTypeItemHirDefn::AssocType(_slf) => None,
            TraitForTypeItemHirDefn::AssocVal(slf) => slf.hir_expr_region(db),
        }
    }

    pub fn hir_expr_body_and_region(self, db: &::salsa::Db) -> Option<(HirExprIdx, HirExprRegion)> {
        match self {
            TraitForTypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn
                .eager_body_with_hir_eager_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn
                .eager_body_with_hir_eager_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            TraitForTypeItemHirDefn::AssocType(_hir_defn) => todo!(),
            TraitForTypeItemHirDefn::AssocVal(_hir_defn) => todo!(),
            // TraitForTypeItemHirDefn::MemoizedField(hir_defn) => hir_defn
            //     .eager_body_with_hir_eager_expr_region(db)
            //     .map(|(body, region)| (body.into(), region.into())),
        }
    }

    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        match self {
            TraitForTypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn.deps(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.deps(db),
            TraitForTypeItemHirDefn::AssocType(hir_defn) => hir_defn.deps(db),
            TraitForTypeItemHirDefn::AssocVal(hir_defn) => hir_defn.deps(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            TraitForTypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::AssocType(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::AssocVal(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for TraitForTypeItemPath {
    type HirDefn = TraitForTypeItemHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        trai_for_ty_item_hir_defn(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn trai_for_ty_item_hir_defn(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> Option<TraitForTypeItemHirDefn> {
    match path.hir_decl(db)? {
        TraitForTypeItemHirDecl::AssocRitchie(hir_decl) => {
            Some(TraitForTypeAssocRitchieHirDefn::new(db, path, hir_decl).into())
        }
        TraitForTypeItemHirDecl::MethodFn(hir_decl) => {
            Some(TraitForTypeMethodRitchieHirDefn::new(db, path, hir_decl).into())
        }
        TraitForTypeItemHirDecl::AssocType(hir_decl) => {
            Some(TraitForTypeAssocTypeHirDefn::new(db, path, hir_decl).into())
        }
        TraitForTypeItemHirDecl::AssocVal(hir_decl) => {
            Some(TraitForTypeAssocValHirDefn::new(db, path, hir_decl).into())
        }
    }
}
