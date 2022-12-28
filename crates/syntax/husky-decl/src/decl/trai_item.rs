mod assoc_ty;
mod assoc_val;
mod function;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method::*;

pub enum TraitItemDecl {
    Function(TraitAssociatedFunctionDecl),
    Method(TraitMethodDecl),
    AlienType(TraitAssociatedTypeDecl),
    Value(TraitAssociatedValueDecl),
}
