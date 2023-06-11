mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeItemDefn {
    AssociatedFn(TypeAssociatedFnDefn),
    MethodFn(TypeMethodFnDefn),
    AssociatedType(TypeAssociatedTypeDefn),
    AssociatedVal(TypeAssociatedValDefn),
    MemoizedField(TypeMemoizedFieldDefn),
}

impl TypeItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TypeItemDecl {
        match self {
            TypeItemDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TypeItemDefn::MethodFn(defn) => defn.decl(db).into(),
            TypeItemDefn::AssociatedType(defn) => defn.decl(db).into(),
            TypeItemDefn::AssociatedVal(defn) => defn.decl(db).into(),
            TypeItemDefn::MemoizedField(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, _db: &dyn DefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            TypeItemDefn::AssociatedFn(defn) => defn.expr_region(db).into(),
            TypeItemDefn::MethodFn(defn) => defn.expr_region(db).into(),
            TypeItemDefn::AssociatedType(defn) => defn.expr_region(db).into(),
            TypeItemDefn::AssociatedVal(defn) => defn.expr_region(db).into(),
            TypeItemDefn::MemoizedField(defn) => defn.expr_region(db).into(),
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
            TypeItemDecl::AssociatedVal(_) => todo!(),
            TypeItemDecl::MemoizedField(decl) => ty_memo_defn(db, decl).into(),
        }
    }
}
