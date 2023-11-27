mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb, jar = HirDefnJar)]
#[enum_class::from_variants]
pub enum TraitItemHirDefn {
    AssociatedFn(TraitAssociatedFnHirDefn),
    MethodFn(TraitMethodFnHirDefn),
    AssociatedType(TraitAssociatedTypeHirDefn),
    AssociatedVal(TraitAssociatedValHirDefn),
}

impl From<TraitItemHirDefn> for HirDefn {
    fn from(hir_defn: TraitItemHirDefn) -> Self {
        HirDefn::AssociatedItem(hir_defn.into())
    }
}

impl TraitItemHirDefn {
    pub fn hir_decl(self, _db: &::salsa::Db) -> TraitItemHirDecl {
        todo!()
    }

    pub fn path(self, _db: &::salsa::Db) -> AssociatedItemPath {
        todo!()
    }
    pub fn hir_expr_region(self, _db: &::salsa::Db) -> HirExprRegion {
        todo!()
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            TraitItemHirDefn::AssociatedFn(hir_defn) => hir_defn.dependencies(db),
            TraitItemHirDefn::MethodFn(hir_defn) => hir_defn.dependencies(db),
            TraitItemHirDefn::AssociatedType(hir_defn) => hir_defn.dependencies(db),
            TraitItemHirDefn::AssociatedVal(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            TraitItemHirDefn::AssociatedFn(hir_defn) => hir_defn.version_stamp(db),
            TraitItemHirDefn::MethodFn(hir_defn) => hir_defn.version_stamp(db),
            TraitItemHirDefn::AssociatedType(hir_defn) => hir_defn.version_stamp(db),
            TraitItemHirDefn::AssociatedVal(hir_defn) => hir_defn.version_stamp(db),
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
        TraitItemHirDecl::AssociatedFn(_hir_decl) => {
            todo!()
            // TraitAssociatedFnHirDefn::new(db, path, hir_decl)?.into()
        }
        TraitItemHirDecl::MethodFn(hir_decl) => {
            Some(TraitMethodFnHirDefn::new(db, path, hir_decl).into())
        }
        TraitItemHirDecl::AssociatedType(_hir_decl) => todo!(),
        TraitItemHirDecl::AssociatedVal(_hir_decl) => todo!(),
    }
}
