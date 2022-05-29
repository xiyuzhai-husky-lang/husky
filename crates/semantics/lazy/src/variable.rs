use crate::*;
use entity_route::EntityRoutePtr;
use vm::InputLiason;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyVariable {
    pub ident: CustomIdentifier,
    pub ty: EntityRoutePtr,
    pub is_reference: bool,
}

impl LazyVariable {
    pub(crate) fn from_input(input_placeholder: &InputParameter) -> Self {
        LazyVariable {
            ident: input_placeholder.ranged_ident.ident,
            ty: input_placeholder.ranged_ty.route,
            is_reference: match input_placeholder.liason {
                InputLiason::Pure => false,
                InputLiason::GlobalRef => todo!(),
                InputLiason::Move => todo!(),
                InputLiason::BorrowMut => todo!(),
                InputLiason::MoveMut => todo!(),
                InputLiason::MemberAccess => todo!(),
            },
        }
    }
}
