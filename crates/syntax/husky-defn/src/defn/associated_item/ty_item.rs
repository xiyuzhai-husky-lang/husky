mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use associated_fn::*;
pub use associated_ty::*;
pub use associated_val::*;
pub use memoized_field::*;
pub use method_fn::*;

use crate::*;
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeItemDefn {
    Function(TypeAssociatedFnDefn),
    Method(TypeMethodFnDefn),
    ExternType(TypeAssociatedTypeDefn),
    Value(TypeAssociatedValDefn),
    Memo(TypeMemoDefn),
}

impl TypeItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TypeItemDecl {
        match self {
            TypeItemDefn::Function(defn) => defn.decl(db).into(),
            TypeItemDefn::Method(defn) => defn.decl(db).into(),
            TypeItemDefn::ExternType(defn) => defn.decl(db).into(),
            TypeItemDefn::Value(defn) => defn.decl(db).into(),
            TypeItemDefn::Memo(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, _db: &dyn DefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            TypeItemDefn::Function(defn) => defn.expr_region(db).into(),
            TypeItemDefn::Method(defn) => defn.expr_region(db).into(),
            TypeItemDefn::ExternType(defn) => defn.expr_region(db).into(),
            TypeItemDefn::Value(defn) => defn.expr_region(db).into(),
            TypeItemDefn::Memo(defn) => defn.expr_region(db).into(),
        }
    }
}

impl HasDefn for TypeItemDecl {
    type Defn = TypeItemDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            TypeItemDecl::AssociatedFn(decl) => ty_associated_fn_defn(db, decl).into(),
            TypeItemDecl::MethodFn(decl) => ty_method_fn_defn(db, decl).into(),
            TypeItemDecl::AssociatedType(_) => todo!(),
            TypeItemDecl::AssociatedValue(_) => todo!(),
            TypeItemDecl::Memo(decl) => ty_memo_defn(db, decl).into(),
        }
    }
}
