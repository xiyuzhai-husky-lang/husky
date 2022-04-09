use crate::*;
use entity_route::EntityRoutePtr;
use vm::InputContract;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyVariable {
    pub ident: CustomIdentifier,
    pub ty: EntityRoutePtr,
    pub is_reference: bool,
}

impl LazyVariable {
    pub(crate) fn from_input(input_placeholder: &InputPlaceholder) -> Self {
        LazyVariable {
            ident: input_placeholder.ident,
            ty: input_placeholder.ranged_ty.route,
            is_reference: match input_placeholder.contract {
                InputContract::Pure => false,
                InputContract::GlobalRef => todo!(),
                InputContract::Move => todo!(),
                InputContract::BorrowMut => todo!(),
                InputContract::MoveMut => todo!(),
                InputContract::Exec => todo!(),
            },
        }
    }
}
