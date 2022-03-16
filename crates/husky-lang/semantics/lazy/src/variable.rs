use scope::{InputPlaceholder, ScopePtr};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyVariable {
    pub ident: CustomIdentifier,
    pub ty: ScopePtr,
    pub is_reference: bool,
}

impl LazyVariable {
    pub(crate) fn from_input(input_placeholder: &InputPlaceholder) -> Self {
        LazyVariable {
            ident: input_placeholder.ident,
            ty: input_placeholder.ranged_ty.scope,
            is_reference: match input_placeholder.contract {
                vm::Contract::PureInput => false,
                vm::Contract::Share => todo!(),
                vm::Contract::Take => todo!(),
                vm::Contract::BorrowMut => todo!(),
                vm::Contract::TakeMut => todo!(),
            },
        }
    }
}
