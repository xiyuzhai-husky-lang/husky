mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use crate::*;
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TraitItemDefn {
    AssociatedFn(TraitAssociatedFnDefn),
    MethodFn(TraitMethodFnDefn),
    AssociatedType(TraitAssociatedTypeDefn),
    AssociatedVal(TraitAssociatedValDefn),
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
            TraitItemDecl::AssociatedFn(decl) => trai_associated_fn_defn(db, decl).into(),
            TraitItemDecl::MethodFn(decl) => trai_method_defn(db, decl).into(),
            TraitItemDecl::AssociatedType(_decl) => todo!(),
            TraitItemDecl::AssociatedVal(_decl) => todo!(),
        }
    }
}
