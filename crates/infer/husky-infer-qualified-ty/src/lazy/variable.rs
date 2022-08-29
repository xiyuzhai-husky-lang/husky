use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyVariableQualifiedTy {
    pub qual: LazyVariableQualifier,
    pub ty: EntityRoutePtr,
}

impl HuskyDisplay for LazyVariableQualifiedTy {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        write!(result, "{: <12?} {:?}", self.qual, self.ty).unwrap()
    }
}

impl LazyVariableQualifiedTy {
    pub fn new(qual: LazyVariableQualifier, ty: EntityRoutePtr) -> Self {
        msg_once!("handle ref");
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
                LazyContract::Pure | LazyContract::Pass => Binding::TempRef,
                _ => panic!(),
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
                LazyContract::Move => panic!(),
            },
            LazyVariableQualifier::PureRef => match contract {
                LazyContract::Pass | LazyContract::Pure => LazyExprQualifier::PureRef,
                LazyContract::EvalRef | LazyContract::Move => panic!(),
            },
            LazyVariableQualifier::EvalRef => match contract {
                LazyContract::Pass => LazyExprQualifier::EvalRef,
                LazyContract::EvalRef => LazyExprQualifier::EvalRef,
                LazyContract::Pure => LazyExprQualifier::PureRef,
                LazyContract::Move => panic!(),
            },
        })
    }

    pub fn parameter(parameter_liason: ParameterModifier, is_copyable: bool) -> Self {
        match parameter_liason {
            ParameterModifier::None => {
                if is_copyable {
                    LazyVariableQualifier::Copyable
                } else {
                    LazyVariableQualifier::PureRef
                }
            }
            ParameterModifier::EvalRef => LazyVariableQualifier::EvalRef,
            ParameterModifier::Owned
            | ParameterModifier::MemberAccess
            | ParameterModifier::TempRef
            | ParameterModifier::TempRefMut
            | ParameterModifier::OwnedMut => {
                panic!("not allowed in lazy paradigm")
            }
        }
    }
}
