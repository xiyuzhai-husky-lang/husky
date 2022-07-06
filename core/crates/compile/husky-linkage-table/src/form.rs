use crate::*;
use husky_instantiate::Instantiable;
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageForm {
    VecConstructor {
        element_ty: EntityRoutePtr,
    },
    TypeCall {
        ty: EntityRoutePtr,
    },
    Routine {
        routine: EntityRoutePtr,
    },
    ElementAccess {
        opd_tys: SmallVec<[EntityRoutePtr; 2]>,
    },
    StructFieldAccess {
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
    },
}

impl Instantiable for LinkageForm {
    type Target = Self;

    fn instantiate(&self, instantiator: &husky_instantiate::InstantiationContext) -> Self::Target {
        match self {
            LinkageForm::VecConstructor { element_ty } => todo!(),
            LinkageForm::TypeCall { ty } => todo!(),
            LinkageForm::Routine { routine } => todo!(),
            LinkageForm::ElementAccess { opd_tys } => todo!(),
            LinkageForm::StructFieldAccess {
                this_ty,
                field_ident,
            } => todo!(),
        }
    }
}
