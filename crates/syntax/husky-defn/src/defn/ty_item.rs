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

impl TypeItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TypeItemDecl {
        todo!()
    }

    pub fn path(self, db: &dyn DefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn expr_sheet(self, db: &dyn DefnDb) -> AssociatedItemDefnExprSheet {
        todo!()
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for TypeItemDefn {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
