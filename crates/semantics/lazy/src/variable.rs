use crate::*;
use entity_route::EntityRoutePtr;
use vm::ParameterLiason;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyVariable {
    pub ident: CustomIdentifier,
    pub ty: EntityRoutePtr,
    pub is_reference: bool,
}

impl LazyVariable {
    pub(crate) fn from_input(input_placeholder: &Parameter) -> Self {
        LazyVariable {
            ident: input_placeholder.ranged_ident.ident,
            ty: input_placeholder.ranged_ty.route,
            is_reference: match input_placeholder.liason {
                ParameterLiason::Pure => false,
                ParameterLiason::EvalRef => todo!(),
                ParameterLiason::Move => todo!(),
                ParameterLiason::TempRefMut => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
            },
        }
    }
}
