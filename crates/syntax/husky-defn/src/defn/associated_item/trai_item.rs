mod associated_fn;
mod associated_ty;
mod associated_value;
mod method_fn;

pub use associated_fn::*;
pub use associated_ty::*;
pub use associated_value::*;
pub use method_fn::*;

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

impl HasDefn for TraitItemDecl {
    type Defn = TraitItemDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            TraitItemDecl::AssociatedFunction(decl) => {
                trai_associated_function_defn(db, decl).into()
            }
            TraitItemDecl::Method(decl) => trai_method_defn(db, decl).into(),
            TraitItemDecl::AssociatedType(_decl) => todo!(),
            TraitItemDecl::Value(_decl) => todo!(),
        }
    }
}
