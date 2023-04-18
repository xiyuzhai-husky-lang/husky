mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use associated_fn::*;
pub use associated_ty::*;
pub use associated_val::*;
pub use method_fn::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDefn {
    Function(TraitForTypeAssociatedFnDefn),
    Method(TraitForTypeMethodFnDefn),
    ExternType(TraitForTypeAssociatedTypeDefn),
    Value(TraitForTypeAssociatedValDefn),
}

impl TraitForTypeItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TraitForTypeItemDecl {
        match self {
            TraitForTypeItemDefn::Function(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::Method(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::ExternType(defn) => defn.decl(db).into(),
            TraitForTypeItemDefn::Value(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, _db: &dyn DefnDb) -> TraitForTypeItemPath {
        todo!()
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            TraitForTypeItemDefn::Function(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::Method(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::ExternType(defn) => defn.expr_region(db),
            TraitForTypeItemDefn::Value(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for TraitForTypeItemDecl {
    type Defn = TraitForTypeItemDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            TraitForTypeItemDecl::AssociatedFunction(_) => todo!(),
            TraitForTypeItemDecl::Method(decl) => trai_for_ty_method_defn(db, decl).into(),
            TraitForTypeItemDecl::AssociatedType(decl) => {
                trai_for_ty_associated_ty_defn(db, decl).into()
            }
            TraitForTypeItemDecl::AssociatedValue(decl) => {
                trai_for_ty_associated_val_defn(db, decl).into()
            }
        }
    }
}
