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
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TraitItemHirDefn {
    AssociatedFn(TraitAssociatedFnHirDefn),
    MethodFn(TraitMethodFnHirDefn),
    AssociatedType(TraitAssociatedTypeHirDefn),
    AssociatedVal(TraitAssociatedValHirDefn),
}

impl TraitItemHirDefn {
    pub fn hir_decl(self, _db: &dyn HirDefnDb) -> TraitItemHirDecl {
        todo!()
    }

    pub fn path(self, _db: &dyn HirDefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn hir_expr_region(self, _db: &dyn HirDefnDb) -> HirExprRegion {
        todo!()
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        match self {
            TraitItemHirDefn::AssociatedFn(_) => todo!(),
            TraitItemHirDefn::MethodFn(_) => todo!(),
            TraitItemHirDefn::AssociatedType(_) => todo!(),
            TraitItemHirDefn::AssociatedVal(_) => todo!(),
        }
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        match self {
            TraitItemHirDefn::AssociatedFn(_) => todo!(),
            TraitItemHirDefn::MethodFn(_) => todo!(),
            TraitItemHirDefn::AssociatedType(_) => todo!(),
            TraitItemHirDefn::AssociatedVal(_) => todo!(),
        }
    }
}

impl HasHirDefn for TraitItemPath {
    type HirDefn = TraitItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        trai_item_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_item_hir_defn(
    db: &dyn HirDefnDb,
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
