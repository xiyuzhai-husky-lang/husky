mod assoc_ritchie;
mod assoc_ty;
mod assoc_val;
mod method_ritchie;

pub use self::assoc_ritchie::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::method_ritchie::*;

use super::*;
use husky_entity_path::AssocItemPath;
use husky_hir_decl::decl::TraitItemHirDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemHirDefn {
    AssocRitchie(TraitAssocRitchieHirDefn),
    MethodFn(TraitMethodFnHirDefn),
    AssocType(TraitAssocTypeHirDefn),
    AssocVal(TraitAssocValHirDefn),
}

impl From<TraitItemHirDefn> for HirDefn {
    fn from(hir_defn: TraitItemHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TraitItemHirDefn {
    pub fn hir_decl(self, _db: &::salsa::Db) -> TraitItemHirDecl {
        todo!()
    }

    pub fn path(self, _db: &::salsa::Db) -> AssocItemPath {
        todo!()
    }
    pub fn hir_expr_region(self, _db: &::salsa::Db) -> HirExprRegion {
        todo!()
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            TraitItemHirDefn::AssocRitchie(hir_defn) => hir_defn.dependencies(db),
            TraitItemHirDefn::MethodFn(hir_defn) => hir_defn.dependencies(db),
            TraitItemHirDefn::AssocType(hir_defn) => hir_defn.dependencies(db),
            TraitItemHirDefn::AssocVal(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            TraitItemHirDefn::AssocRitchie(hir_defn) => hir_defn.version_stamp(db),
            TraitItemHirDefn::MethodFn(hir_defn) => hir_defn.version_stamp(db),
            TraitItemHirDefn::AssocType(hir_defn) => hir_defn.version_stamp(db),
            TraitItemHirDefn::AssocVal(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for TraitItemPath {
    type HirDefn = TraitItemHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        trai_item_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_item_hir_defn(
    db: &::salsa::Db,
    path: TraitItemPath,
) -> Option<TraitItemHirDefn> {
    let hir_decl = path.hir_decl(db)?;
    match hir_decl {
        TraitItemHirDecl::AssocRitchie(_hir_decl) => {
            todo!()
            // TraitAssocRitchieHirDefn::new(db, path, hir_decl)?.into()
        }
        TraitItemHirDecl::MethodFn(hir_decl) => {
            Some(TraitMethodFnHirDefn::new(db, path, hir_decl).into())
        }
        TraitItemHirDecl::AssocType(_hir_decl) => todo!(),
        TraitItemHirDecl::AssocVal(_hir_decl) => todo!(),
    }
}
