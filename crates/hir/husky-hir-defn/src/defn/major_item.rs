pub mod form;
pub mod trai;
pub mod ty;

use self::form::*;
use self::trai::*;
use self::ty::*;
use super::*;
use husky_entity_path::path::major_item::MajorItemPath;
use husky_hir_decl::decl::MajorItemHirDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorItemHirDefn {
    Type(TypeHirDefn),
    Trait(TraitHirDefn),
    Form(MajorFormHirDefn),
}

impl MajorItemHirDefn {
    pub fn path(self, db: &::salsa::Db) -> MajorItemPath {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.path(db).into(),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.path(db).into(),
            MajorItemHirDefn::Form(hir_defn) => hir_defn.path(db).into(),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> MajorItemHirDecl {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.hir_decl(db).into(),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.hir_decl(db).into(),
            MajorItemHirDefn::Form(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            MajorItemHirDefn::Type(_) | MajorItemHirDefn::Trait(_) => None,
            MajorItemHirDefn::Form(hir_defn) => hir_defn.hir_expr_region(db),
        }
    }

    pub fn hir_expr_body_and_region(self, db: &::salsa::Db) -> Option<(HirExprIdx, HirExprRegion)> {
        match self {
            MajorItemHirDefn::Type(_) | MajorItemHirDefn::Trait(_) => None,
            MajorItemHirDefn::Form(hir_defn) => hir_defn.hir_expr_body_and_region(db),
        }
    }

    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.deps(db),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.deps(db),
            MajorItemHirDefn::Form(hir_defn) => hir_defn.deps(db),
        }
    }

    pub fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.version_stamp(db),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.version_stamp(db),
            MajorItemHirDefn::Form(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for MajorItemPath {
    type HirDefn = MajorItemHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        Some(match self {
            MajorItemPath::Type(path) => path.hir_defn(db)?.into(),
            MajorItemPath::Form(path) => path.hir_defn(db)?.into(),
            MajorItemPath::Trait(path) => path.hir_defn(db)?.into(),
        })
    }
}
