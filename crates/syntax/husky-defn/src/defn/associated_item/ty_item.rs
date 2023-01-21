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
pub enum TypeItemDefn {
    Function(TypeAssociatedFunctionDefn),
    Method(TypeMethodDefn),
    AlienType(TypeAssociatedTypeDefn),
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
        Self::AlienType(v)
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
            TypeItemDefn::AlienType(defn) => defn.decl(db).into(),
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
            TypeItemDefn::AlienType(defn) => defn.expr_region(db).into(),
            TypeItemDefn::Value(defn) => defn.expr_region(db).into(),
            TypeItemDefn::Memo(defn) => defn.expr_region(db).into(),
        }
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for TypeItemDefn {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DefnJar>>::as_jar_db(db);
        match self {
            TypeItemDefn::Function(decl) => f
                .debug_tuple("Function")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemDefn::Method(decl) => f
                .debug_tuple("Method")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemDefn::AlienType(_) => todo!(),
            TypeItemDefn::Value(_) => todo!(),
            TypeItemDefn::Memo(decl) => f
                .debug_tuple("Memo")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
