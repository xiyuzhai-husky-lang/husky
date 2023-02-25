mod assoc_ty;
mod assoc_val;
mod function;
mod memo;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use memo::*;
pub use method::*;

use crate::*;
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
pub enum TypeItemDefn {
    Function(TypeAssociatedFunctionDefn),
    Method(TypeMethodDefn),
    ExternType(TypeAssociatedTypeDefn),
    Value(TypeAssociatedValueDefn),
    Memo(TypeMemoDefn),
}

impl From<TypeMemoDefn> for TypeItemDefn {
    fn from(v: TypeMemoDefn) -> Self {
        Self::Memo(v)
    }
}

impl From<TypeAssociatedValueDefn> for TypeItemDefn {
    fn from(v: TypeAssociatedValueDefn) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAssociatedTypeDefn> for TypeItemDefn {
    fn from(v: TypeAssociatedTypeDefn) -> Self {
        Self::ExternType(v)
    }
}

impl From<TypeMethodDefn> for TypeItemDefn {
    fn from(v: TypeMethodDefn) -> Self {
        Self::Method(v)
    }
}

impl From<TypeAssociatedFunctionDefn> for TypeItemDefn {
    fn from(v: TypeAssociatedFunctionDefn) -> Self {
        Self::Function(v)
    }
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

    pub fn path(self, db: &dyn DefnDb) -> AssociatedItemPath {
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
