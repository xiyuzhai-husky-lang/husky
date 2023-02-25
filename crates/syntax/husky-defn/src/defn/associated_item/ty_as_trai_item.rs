mod assoc_ty;
mod assoc_val;
mod function;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method::*;

use crate::*;
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
pub enum TypeAsTraitItemDefn {
    Function(TypeAsTraitAssociatedFunctionDefn),
    Method(TypeAsTraitMethodDefn),
    ExternType(TypeAsTraitAssociatedTypeDefn),
    Value(TypeAsTraitAssociatedValueDefn),
}

impl From<TypeAsTraitAssociatedValueDefn> for TypeAsTraitItemDefn {
    fn from(v: TypeAsTraitAssociatedValueDefn) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAsTraitAssociatedTypeDefn> for TypeAsTraitItemDefn {
    fn from(v: TypeAsTraitAssociatedTypeDefn) -> Self {
        Self::ExternType(v)
    }
}

impl From<TypeAsTraitMethodDefn> for TypeAsTraitItemDefn {
    fn from(v: TypeAsTraitMethodDefn) -> Self {
        Self::Method(v)
    }
}

impl From<TypeAsTraitAssociatedFunctionDefn> for TypeAsTraitItemDefn {
    fn from(v: TypeAsTraitAssociatedFunctionDefn) -> Self {
        Self::Function(v)
    }
}

impl TypeAsTraitItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TypeAsTraitItemDecl {
        match self {
            TypeAsTraitItemDefn::Function(defn) => defn.decl(db).into(),
            TypeAsTraitItemDefn::Method(defn) => defn.decl(db).into(),
            TypeAsTraitItemDefn::ExternType(defn) => defn.decl(db).into(),
            TypeAsTraitItemDefn::Value(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> TypeAsTraitItemPath {
        todo!()
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            TypeAsTraitItemDefn::Function(defn) => defn.expr_region(db),
            TypeAsTraitItemDefn::Method(defn) => defn.expr_region(db),
            TypeAsTraitItemDefn::ExternType(defn) => defn.expr_region(db),
            TypeAsTraitItemDefn::Value(defn) => defn.expr_region(db),
        }
    }
}
