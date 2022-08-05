use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerVariableQualifiedTy {
    pub qual: EagerVariableQualifier,
    pub ty: EntityRoutePtr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EagerVariableQualifier {
    Copyable,
    CopyableMut,
    Owned,
    OwnedMut,
    PureRef,
    EvalRef,
    TempRef,
    TempRefMut,
}

impl HuskyDisplay for EagerVariableQualifiedTy {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}{: <12?}{} {}{:?}{}",
                husky_print_utils::PINK,
                self.qual,
                husky_print_utils::RESET,
                husky_print_utils::GREEN,
                self.ty,
                husky_print_utils::RESET,
            )
            .unwrap()
        } else {
            write!(result, "{: <12?} {:?}", self.qual, self.ty,).unwrap()
        }
    }
}

impl std::fmt::Debug for EagerVariableQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            EagerVariableQualifier::Copyable => "Copyable",
            EagerVariableQualifier::CopyableMut => "CopyableMut",
            EagerVariableQualifier::Owned => "Owned",
            EagerVariableQualifier::OwnedMut => "OwnedMut",
            EagerVariableQualifier::PureRef => "PureRef",
            EagerVariableQualifier::EvalRef => "EvalRef",
            EagerVariableQualifier::TempRef => "TempRef",
            EagerVariableQualifier::TempRefMut => "RefMut",
        })
    }
}

impl EagerVariableQualifier {
    pub fn mutable(&self) -> bool {
        match self {
            EagerVariableQualifier::Copyable
            | EagerVariableQualifier::PureRef
            | EagerVariableQualifier::EvalRef
            | EagerVariableQualifier::TempRef
            | EagerVariableQualifier::Owned => false,
            EagerVariableQualifier::CopyableMut
            | EagerVariableQualifier::OwnedMut
            | EagerVariableQualifier::TempRefMut => true,
        }
    }

    pub fn parameter_eager_variable_qualifier(
        db: &dyn DeclQueryGroup,
        parameter_ty: EntityRoutePtr,
        parameter_liason: ParameterLiason,
    ) -> InferResult<Self> {
        Ok(match parameter_ty.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => {
                if parameter_ty.temporal_arguments.len() == 0 {
                    EagerVariableQualifier::EvalRef
                } else {
                    EagerVariableQualifier::TempRef
                }
            }
            _ => {
                let is_copyable = db.is_copyable(parameter_ty)?;
                match parameter_liason {
                    ParameterLiason::Pure => {
                        if is_copyable {
                            EagerVariableQualifier::Copyable
                        } else {
                            EagerVariableQualifier::PureRef
                        }
                    }
                    ParameterLiason::EvalRef => EagerVariableQualifier::EvalRef,
                    ParameterLiason::Move => EagerVariableQualifier::Owned,
                    ParameterLiason::MoveMut => EagerVariableQualifier::OwnedMut,
                    ParameterLiason::TempRefMut => EagerVariableQualifier::TempRefMut,
                    ParameterLiason::TempRef => EagerVariableQualifier::TempRef,
                    ParameterLiason::MemberAccess => todo!(),
                }
            }
        })
    }

    pub fn variable_use_eager_expr_qualifier(
        self,
        contract: EagerContract,
        range: TextRange,
    ) -> InferResult<EagerExprQualifier> {
        Ok(match contract {
            EagerContract::Pure => match self {
                EagerVariableQualifier::Copyable | EagerVariableQualifier::CopyableMut => {
                    EagerExprQualifier::Copyable
                }
                _ => EagerExprQualifier::PureRef,
            },
            EagerContract::Move => match self {
                EagerVariableQualifier::Copyable | EagerVariableQualifier::CopyableMut => panic!(),
                EagerVariableQualifier::Owned | EagerVariableQualifier::OwnedMut => {
                    EagerExprQualifier::Transient
                }
                _ => throw!(format!("can't move from ref"), range),
            },
            EagerContract::Pass => match self {
                EagerVariableQualifier::Copyable | EagerVariableQualifier::CopyableMut => {
                    EagerExprQualifier::Copyable
                }
                EagerVariableQualifier::EvalRef => EagerExprQualifier::EvalRef,
                EagerVariableQualifier::PureRef => EagerExprQualifier::PureRef,
                _ => EagerExprQualifier::TempRef,
            },
            EagerContract::EvalRef => match self {
                EagerVariableQualifier::EvalRef => EagerExprQualifier::EvalRef,
                _ => throw!(format!("expect eval ref"), range),
            },
            EagerContract::TempRef => todo!(),
            EagerContract::TempRefMut => match self {
                EagerVariableQualifier::CopyableMut | EagerVariableQualifier::OwnedMut => {
                    EagerExprQualifier::TempRefMut
                }
                _ => throw!(format!("can't bind this to a temp mut ref"), range),
            },
        })
    }

    pub fn binding(self, contract: EagerContract) -> Binding {
        match self {
            EagerVariableQualifier::PureRef | EagerVariableQualifier::TempRef => Binding::TempRef,
            EagerVariableQualifier::Copyable => Binding::Copy,
            EagerVariableQualifier::EvalRef => match contract {
                EagerContract::Pure | EagerContract::TempRef => Binding::TempRef,
                EagerContract::EvalRef | EagerContract::Pass => Binding::EvalRef,
                _ => panic!(),
            },
            EagerVariableQualifier::TempRefMut => todo!(),
            EagerVariableQualifier::CopyableMut => match contract {
                EagerContract::Pure | EagerContract::Pass => Binding::Copy,
                EagerContract::TempRefMut => Binding::TempMut,
                _ => panic!(),
            },
            EagerVariableQualifier::Owned => match contract {
                EagerContract::Pure | EagerContract::TempRef | EagerContract::Pass => {
                    Binding::TempRef
                }
                EagerContract::Move => Binding::Move,
                EagerContract::TempRefMut => Binding::TempMut,
                _ => panic!(),
            },
            EagerVariableQualifier::OwnedMut => match contract {
                EagerContract::Move => Binding::Move,
                EagerContract::TempRefMut => Binding::TempMut,
                EagerContract::Pure | EagerContract::TempRef | EagerContract::Pass => {
                    Binding::TempRef
                }
                _ => panic!(),
            },
        }
    }
}

impl EagerVariableQualifiedTy {
    pub(crate) fn ty_qualified_ty() -> Self {
        Self {
            qual: EagerVariableQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }

    pub(crate) fn from_parameter(
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
        parameter_liason: ParameterLiason,
    ) -> InferResult<Self> {
        Ok(EagerVariableQualifiedTy {
            qual: EagerVariableQualifier::parameter_eager_variable_qualifier(
                db.upcast(),
                ty,
                parameter_liason,
            )?,
            ty: ty.deref_route(),
        })
    }

    pub(crate) fn new(qual: EagerVariableQualifier, ty: EntityRoutePtr) -> Self {
        match ty.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => todo!(),
            _ => Self { qual, ty },
        }
    }
}
