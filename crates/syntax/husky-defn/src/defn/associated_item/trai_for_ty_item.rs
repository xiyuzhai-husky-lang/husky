mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDefn {
    AssociatedFn(TraitForTypeAssociatedFnDefn),
    MethodFn(TraitForTypeMethodFnDefn),
    AssociatedType(TraitForTypeAssociatedTypeDefn),
    AssociatedVal(TraitForTypeAssociatedValDefn),
}

impl TraitForTypeItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TraitForTypeItemDecl {
        match self {
            TraitForTypeItemDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::MethodFn(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::AssociatedType(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::AssociatedVal(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, _db: &dyn DefnDb) -> TraitForTypeItemPath {
        todo!()
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            TraitForTypeItemDefn::AssociatedFn(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::MethodFn(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::AssociatedType(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::AssociatedVal(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for TraitForTypeItemDecl {
    type Defn = TraitForTypeItemDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            TraitForTypeItemDecl::AssociatedFn(_) => todo!(),
            TraitForTypeItemDecl::MethodFn(decl) => trai_for_ty_method_defn(db, decl).into(),
            TraitForTypeItemDecl::AssociatedType(decl) => {
                trai_for_ty_associated_ty_defn(db, decl).into()
            }
            TraitForTypeItemDecl::AssociatedVal(decl) => {
                trai_for_ty_associated_val_defn(db, decl).into()
            }
        }
    }
}
