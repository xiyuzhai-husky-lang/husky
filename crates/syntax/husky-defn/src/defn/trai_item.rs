mod assoc_ty;
mod assoc_val;
mod function;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TraitItemDefn {
    Function(TraitAssociatedFunctionDefn),
    Method(TraitMethodDefn),
    AlienType(TraitAssociatedTypeDefn),
    Value(TraitAssociatedValueDefn),
}
