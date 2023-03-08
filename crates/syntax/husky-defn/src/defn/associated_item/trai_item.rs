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
#[enum_class::from_variants]
pub enum TraitItemDefn {
    Function(TraitAssociatedFunctionDefn),
    Method(TraitMethodDefn),
    ExternType(TraitAssociatedTypeDefn),
    Value(TraitAssociatedValueDefn),
}

impl TraitItemDefn {
    pub fn decl(self, _db: &dyn DefnDb) -> TraitItemDecl {
        todo!()
    }

    pub fn path(self, _db: &dyn DefnDb) -> AssociatedItemPath {
        todo!()
    }
    pub fn expr_region(self, _db: &dyn DefnDb) -> ExprRegion {
        todo!()
    }
}
