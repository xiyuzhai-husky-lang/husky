use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyVariableQualifiedTy {
    pub qual: LazyVariableQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for LazyVariableQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        write!(result, "{: <12?} {:?}", self.qual, self.ty).unwrap()
    }
}

impl LazyVariableQualifiedTy {
    pub(crate) fn parameter_lazy_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        parameter_liason: ParameterLiason,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        Ok(LazyVariableQualifiedTy::new(
            LazyVariableQualifier::parameter(parameter_liason, db.is_copyable(ty)?),
            ty,
        ))
    }

    pub fn new(qual: LazyVariableQualifier, ty: EntityRoutePtr) -> Self {
        emsg_once!("handle ref");
        Self { qual, ty }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LazyVariableQualifier {
    Copyable,
    PureRef,
    EvalRef,
}

impl LazyVariableQualifier {
    pub fn feature(is_copyable: bool) -> LazyVariableQualifier {
        if is_copyable {
            LazyVariableQualifier::Copyable
        } else {
            LazyVariableQualifier::EvalRef
        }
    }

    pub fn binding(self, contract: LazyContract) -> Binding {
        match self {
            LazyVariableQualifier::PureRef => match contract {
                LazyContract::Pure => Binding::TempRef,
                LazyContract::EvalRef => todo!(),
                LazyContract::Move => todo!(),
                LazyContract::Pass => Binding::TempRef,
            },
            LazyVariableQualifier::Copyable => Binding::Copy,
            LazyVariableQualifier::EvalRef => Binding::EvalRef,
        }
    }

    pub fn variable_use(self, contract: LazyContract) -> InferResult<LazyExprQualifier> {
        Ok(match self {
            LazyVariableQualifier::Copyable => match contract {
                LazyContract::Pass => LazyExprQualifier::Copyable,
                LazyContract::EvalRef => LazyExprQualifier::EvalRef,
                LazyContract::Pure => LazyExprQualifier::Copyable,
                LazyContract::Move => todo!(),
            },
            LazyVariableQualifier::PureRef => match contract {
                LazyContract::Pass => todo!(),
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => LazyExprQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
            LazyVariableQualifier::EvalRef => match contract {
                LazyContract::Pass => LazyExprQualifier::EvalRef,
                LazyContract::EvalRef => LazyExprQualifier::EvalRef,
                LazyContract::Pure => LazyExprQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
        })
    }

    pub fn parameter(parameter_liason: ParameterLiason, is_copyable: bool) -> Self {
        match parameter_liason {
            ParameterLiason::Pure => {
                if is_copyable {
                    LazyVariableQualifier::Copyable
                } else {
                    LazyVariableQualifier::PureRef
                }
            }
            ParameterLiason::EvalRef => LazyVariableQualifier::EvalRef,
            ParameterLiason::Move => todo!(),
            ParameterLiason::TempRefMut => todo!(),
            ParameterLiason::MoveMut => todo!(),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::TempRef => todo!(),
        }
    }
}
