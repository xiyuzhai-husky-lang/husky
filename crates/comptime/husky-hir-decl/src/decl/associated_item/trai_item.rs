mod associated_fn;
mod associated_val;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;
pub use self::associated_val::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemHirDecl {
    AssociatedFn(TraitAssociatedFnHirDecl),
}

impl HasHirDecl for TraitItemPath {
    type HirDecl = TraitItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        todo!()
    }
}
