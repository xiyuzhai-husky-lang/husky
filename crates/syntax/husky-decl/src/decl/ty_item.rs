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

pub enum TypeItemDecl {
    Function(TypeAssociatedFunctionDecl),
    Method(TypeMethodDecl),
    AliasType(TypeAssociatedTypeDecl),
    Value(TypeAssociatedValueDecl),
    Memo(TypeMemoDecl),
}
