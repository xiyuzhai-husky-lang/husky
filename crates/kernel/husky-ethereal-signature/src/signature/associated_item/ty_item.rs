mod associated_fn;
mod associated_val;
mod memoized_field;
mod method;

pub use self::associated_fn::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method::*;

use super::*;

pub enum TypeItemEtherealSignatureTemplates {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnEtherealSignatureTemplate>),
    Method(TypeMethodEtherealSignatureTemplates),
}
