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
#[salsa::derive_debug_with_db(db = HirDefnDb)]
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
}

impl HasHirDefn for TraitItemPath {
    type HirDefn = TraitItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        trai_item_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_item_hir_defn(db: &dyn HirDefnDb, path: TraitItemPath) -> TraitItemHirDefn {
    let hir_decl = path.hir_decl(db);
    match hir_decl {
        TraitItemHirDecl::AssociatedFn(hir_decl) => {
            todo!()
            // TraitAssociatedFnHirDefn::new(db, path, hir_decl)?.into()
        }
        TraitItemHirDecl::MethodFn(hir_decl) => {
            TraitMethodFnHirDefn::new(db, path, hir_decl).into()
        }
        TraitItemHirDecl::AssociatedType(hir_decl) => todo!(),
        TraitItemHirDecl::AssociatedVal(hir_decl) => todo!(),
    }
}
