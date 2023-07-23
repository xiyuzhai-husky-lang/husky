mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemHirDefn {
    AssociatedFn(TraitForTypeAssociatedFnHirDefn),
    MethodFn(TraitForTypeMethodFnHirDefn),
    AssociatedType(TraitForTypeAssociatedTypeHirDefn),
    AssociatedVal(TraitForTypeAssociatedValHirDefn),
}

impl TraitForTypeItemHirDefn {
    pub fn decl(self, db: &dyn HirDefnDb) -> TraitForTypeItemHirDecl {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TraitForTypeItemHirDefn::MethodFn(defn) => defn.decl(db).into(),
            TraitForTypeItemHirDefn::AssociatedType(defn) => defn.decl(db).into(),
            TraitForTypeItemHirDefn::AssociatedVal(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, _db: &dyn HirDefnDb) -> TraitForTypeItemPath {
        todo!()
    }

    pub fn expr_region(self, db: &dyn HirDefnDb) -> HirExprRegion {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(defn) => defn.expr_region(db),
            TraitForTypeItemHirDefn::MethodFn(defn) => defn.expr_region(db),
            TraitForTypeItemHirDefn::AssociatedType(defn) => defn.expr_region(db),
            TraitForTypeItemHirDefn::AssociatedVal(defn) => defn.expr_region(db),
        }
    }
}

impl HasHirDefn for TraitForTypeItemPath {
    type HirDefn = TraitForTypeItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        trai_for_ty_item_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_for_ty_item_hir_defn(
    db: &dyn HirDefnDb,
    path: TraitForTypeItemPath,
) -> TraitForTypeItemHirDefn {
    match path.hir_decl(db) {
        TraitForTypeItemHirDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemHirDecl::MethodFn(decl) => {
            TraitForTypeMethodFnHirDefn::new(db, path, decl).into()
        }
        TraitForTypeItemHirDecl::AssociatedType(decl) => {
            TraitForTypeAssociatedTypeHirDefn::new(db, path, decl).into()
        }
        TraitForTypeItemHirDecl::AssociatedVal(decl) => {
            TraitForTypeAssociatedValHirDefn::new(db, path, decl).into()
        }
    }
}
