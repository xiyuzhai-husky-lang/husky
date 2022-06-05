use crate::*;
use entity_route::EntityRoutePtr;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyVariable {
    pub ident: CustomIdentifier,
    pub ty: EntityRoutePtr,
    pub is_reference: bool,
}

impl LazyVariable {
    pub(crate) fn from_parameter(parameter: &Parameter) -> Self {
        LazyVariable {
            ident: parameter.ranged_ident.ident,
            ty: parameter.ranged_ty.route,
            is_reference: match parameter.liason {
                ParameterLiason::Pure => false,
                ParameterLiason::EvalRef => todo!(),
                ParameterLiason::Move => todo!(),
                ParameterLiason::TempRefMut => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::TempRef => todo!(),
            },
        }
    }
}
