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
pub enum TypeAsTraitItemDefn {
    Function(TypeAsTraitAssociatedFunctionDefn),
    Method(TypeAsTraitMethodDefn),
    AlienType(TypeAsTraitAssociatedTypeDefn),
    Value(TypeAsTraitAssociatedValueDefn),
}

impl From<TypeAsTraitAssociatedValueDefn> for TypeAsTraitItemDefn {
    fn from(v: TypeAsTraitAssociatedValueDefn) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAsTraitAssociatedTypeDefn> for TypeAsTraitItemDefn {
    fn from(v: TypeAsTraitAssociatedTypeDefn) -> Self {
        Self::AlienType(v)
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
        todo!()
    }

    pub fn path(self, db: &dyn DefnDb) -> TypeAsTraitItemPath {
        todo!()
    }
    pub fn expr_sheet(self, db: &dyn DefnDb) -> ExprSheet {
        todo!()
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for TypeAsTraitItemDefn {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DefnJar>>::as_jar_db(db);
        match self {
            TypeAsTraitItemDefn::Function(decl) => f
                .debug_tuple("Function")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeAsTraitItemDefn::Method(decl) => f
                .debug_tuple("Method")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeAsTraitItemDefn::AlienType(_) => todo!(),
            TypeAsTraitItemDefn::Value(_) => todo!(),
        }
    }
}
