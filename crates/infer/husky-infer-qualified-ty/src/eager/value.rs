use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerValueQualifiedTy {
    pub qual: EagerExprQualifier,
    pub ty: EntityRoutePtr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EagerExprQualifier {
    Copyable,
    PureRef,
    EvalRef,
    TempRef,
    TempRefMut,
    Transient,
}

impl std::fmt::Debug for EagerExprQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            EagerExprQualifier::Copyable => "Copyable",
            EagerExprQualifier::PureRef => "PureRef",
            EagerExprQualifier::EvalRef => "EvalRef",
            EagerExprQualifier::TempRef => "TempRef",
            EagerExprQualifier::TempRefMut => "RefMut",
            EagerExprQualifier::Transient => "Transient",
        })
    }
}

impl HuskyDisplay for EagerValueQualifiedTy {
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
impl EagerExprQualifier {
    pub fn mutable(&self) -> bool {
        match self {
            EagerExprQualifier::Copyable
            | EagerExprQualifier::PureRef
            | EagerExprQualifier::EvalRef
            | EagerExprQualifier::TempRef
            | EagerExprQualifier::Transient => false,
            EagerExprQualifier::TempRefMut => true,
        }
    }

    pub fn binding(self, contract: EagerContract) -> Binding {
        match self {
            EagerExprQualifier::PureRef | EagerExprQualifier::TempRef => Binding::TempRef,
            EagerExprQualifier::Transient => Binding::Move,
            EagerExprQualifier::Copyable => Binding::Copy,
            EagerExprQualifier::EvalRef => match contract {
                EagerContract::Pure | EagerContract::TempRef => Binding::TempRef,
                EagerContract::EvalRef => Binding::EvalRef,
                EagerContract::Pass => Binding::EvalRef,
                EagerContract::Move | EagerContract::TempRefMut => panic!(),
            },
            EagerExprQualifier::TempRefMut => match contract {
                EagerContract::Pure | EagerContract::TempRef | EagerContract::Pass => {
                    Binding::TempRef
                }
                EagerContract::TempRefMut => Binding::TempMut,
                _ => panic!(),
            },
        }
    }

    pub fn parameter_use_eager_qualifier(
        db: &dyn DeclQueryGroup,
        parameter_ty: EntityRoutePtr,
        parameter_liason: ParameterLiason,
        contract: EagerContract,
        range: TextRange,
    ) -> InferResult<Self> {
        EagerVariableQualifier::parameter_eager_variable_qualifier(
            db,
            parameter_ty,
            parameter_liason,
        )?
        .variable_use_eager_expr_qualifier(contract, range)
    }

    pub fn from_output(output_liason: OutputLiason, is_copyable: bool) -> Self {
        match output_liason {
            OutputLiason::Transfer => Self::transitive(is_copyable),
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    pub fn transitive(is_copyable: bool) -> Self {
        if is_copyable {
            EagerExprQualifier::Copyable
        } else {
            EagerExprQualifier::Transient
        }
    }

    pub fn member(
        this_qual: Self,
        member_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_copyable: bool,
    ) -> Self {
        if is_member_copyable {
            match member_contract {
                EagerContract::Pure => EagerExprQualifier::Copyable,
                EagerContract::EvalRef => EagerExprQualifier::EvalRef,
                EagerContract::TempRefMut => EagerExprQualifier::TempRefMut,
                EagerContract::Pass | EagerContract::TempRef | EagerContract::Move => panic!(),
            }
            // match this_qual {
            //     EagerExprQualifier::Copyable
            //     | EagerExprQualifier::PureRef
            //     | EagerExprQualifier::EvalRef
            //     | EagerExprQualifier::TempRef
            //     | EagerExprQualifier::Transient => EagerExprQualifier::Copyable,
            //     EagerExprQualifier::TempRefMut => EagerExprQualifier::TempRefMut,
            // }
        } else {
            match this_qual {
                EagerExprQualifier::Copyable => panic!(),
                EagerExprQualifier::PureRef
                | EagerExprQualifier::EvalRef
                | EagerExprQualifier::TempRef
                | EagerExprQualifier::Transient
                | EagerExprQualifier::TempRefMut => this_qual,
            }
        }
    }
}

impl EagerValueQualifiedTy {
    pub(crate) fn ty_eager_qualified_ty() -> Self {
        Self {
            qual: EagerExprQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }

    pub(crate) fn module_eager_qualified_ty() -> Self {
        Self {
            qual: EagerExprQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::ModuleType),
        }
    }

    pub(crate) fn trait_eager_qualified_ty() -> Self {
        Self {
            qual: EagerExprQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TraitType),
        }
    }

    pub(crate) fn entity_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        Ok(Self {
            qual: if db.is_copyable(ty)? {
                EagerExprQualifier::Copyable
            } else {
                EagerExprQualifier::EvalRef
            },
            ty,
        })
    }

    pub(crate) fn feature_ty(ty: EntityRoutePtr) -> Self {
        Self {
            qual: EagerExprQualifier::EvalRef,
            ty: ty.deref_route(),
        }
    }

    pub(crate) fn from_parameter_use(
        db: &dyn InferQualifiedTyQueryGroup,
        parameter_liason: ParameterLiason,
        ty: EntityRoutePtr,
        contract: EagerContract,
        range: TextRange,
    ) -> InferResult<Self> {
        Ok(EagerValueQualifiedTy {
            qual: EagerExprQualifier::parameter_use_eager_qualifier(
                db.upcast(),
                ty,
                parameter_liason,
                contract,
                range,
            )?,
            ty: ty.deref_route(),
        })
    }

    pub(crate) fn new(qual: EagerExprQualifier, ty: EntityRoutePtr) -> Self {
        match ty.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => {
                let qual = if ty.temporal_arguments.len() == 0 {
                    match qual {
                        EagerExprQualifier::Copyable => EagerExprQualifier::EvalRef,
                        EagerExprQualifier::PureRef => todo!(),
                        EagerExprQualifier::EvalRef => todo!(),
                        EagerExprQualifier::TempRef => todo!(),
                        EagerExprQualifier::TempRefMut => todo!(),
                        EagerExprQualifier::Transient => todo!(),
                    }
                } else {
                    todo!()
                };
                Self {
                    qual,
                    ty: ty.deref_route(),
                }
            }
            _ => Self { qual, ty },
        }
    }

    pub(crate) fn member_eager_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        this_qual: EagerExprQualifier,
        field_ty: EntityRoutePtr,
        member_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_copyable: bool,
    ) -> InferResult<Self> {
        msg_once!("ad hoc; consider ref");
        Ok(Self::new(
            EagerExprQualifier::member(
                this_qual,
                member_liason,
                member_contract,
                is_member_copyable,
            ),
            field_ty,
        ))
    }

    pub(crate) fn init_variable_qualified_ty(
        self,
        init_kind: InitKind,
    ) -> InferResult<EagerVariableQualifiedTy> {
        let qual = match init_kind {
            InitKind::Let => match self.qual {
                EagerExprQualifier::Copyable => EagerVariableQualifier::Copyable,
                EagerExprQualifier::PureRef => EagerVariableQualifier::PureRef,
                EagerExprQualifier::TempRef => EagerVariableQualifier::TempRef,
                EagerExprQualifier::Transient => EagerVariableQualifier::Owned,
                EagerExprQualifier::EvalRef => EagerVariableQualifier::EvalRef,
                EagerExprQualifier::TempRefMut => EagerVariableQualifier::TempRef,
            },
            InitKind::Var => match self.qual {
                EagerExprQualifier::Copyable
                | EagerExprQualifier::EvalRef
                | EagerExprQualifier::TempRefMut => EagerVariableQualifier::CopyableMut,
                EagerExprQualifier::PureRef => EagerVariableQualifier::PureRef,
                EagerExprQualifier::TempRef => EagerVariableQualifier::TempRef,
                EagerExprQualifier::Transient => EagerVariableQualifier::OwnedMut,
            },
            InitKind::Decl => match self.qual {
                EagerExprQualifier::Copyable => EagerVariableQualifier::Copyable,
                EagerExprQualifier::PureRef => EagerVariableQualifier::PureRef,
                EagerExprQualifier::EvalRef => EagerVariableQualifier::EvalRef,
                EagerExprQualifier::TempRef => EagerVariableQualifier::TempRef,
                EagerExprQualifier::Transient => EagerVariableQualifier::Owned,
                EagerExprQualifier::TempRefMut => panic!(),
            },
        };
        Ok(EagerVariableQualifiedTy { qual, ty: self.ty })
    }

    pub fn is_implicitly_castable_to_output(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        output_liason: OutputLiason,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicitly_castable(self.ty, output_ty) {
            return false;
        }
        match output_liason {
            OutputLiason::Transfer => match self.qual {
                EagerExprQualifier::PureRef | EagerExprQualifier::TempRef => false,
                EagerExprQualifier::Transient | EagerExprQualifier::Copyable => true,
                EagerExprQualifier::EvalRef => true,
                EagerExprQualifier::TempRefMut => todo!(),
            },
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    pub fn as_ty(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        if !db.is_explicitly_castable(self.ty, ty)? {
            todo!()
        }
        Ok(Self {
            qual: self.qual,
            ty,
        })
    }
}
