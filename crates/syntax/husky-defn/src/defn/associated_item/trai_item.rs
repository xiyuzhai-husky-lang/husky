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
pub enum TraitItemDefn {
    Function(TraitAssociatedFunctionDefn),
    Method(TraitMethodDefn),
    AlienType(TraitAssociatedTypeDefn),
    Value(TraitAssociatedValueDefn),
}

impl From<TraitMethodDefn> for TraitItemDefn {
    fn from(v: TraitMethodDefn) -> Self {
        Self::Method(v)
    }
}

impl From<TraitAssociatedFunctionDefn> for TraitItemDefn {
    fn from(v: TraitAssociatedFunctionDefn) -> Self {
        Self::Function(v)
    }
}

impl TraitItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TraitItemDecl {
        todo!()
    }

    pub fn path(self, db: &dyn DefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        todo!()
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for TraitItemDefn {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        todo!()
    }
}
