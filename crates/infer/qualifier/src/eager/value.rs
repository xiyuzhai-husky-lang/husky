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

impl TestDisplay for EagerValueQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}{: <12?}{} {}{:?}{}",
                print_utils::PINK,
                self.qual,
                print_utils::RESET,
                print_utils::GREEN,
                self.ty,
                print_utils::RESET,
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
            EagerExprQualifier::PureRef | EagerExprQualifier::TempRef => match contract {
                EagerContract::Pure | EagerContract::Pass => Binding::TempRef,
                EagerContract::Move => panic!(),
                EagerContract::TempRefMut => todo!(),
                EagerContract::EvalRef => panic!(),
                EagerContract::TempRef => todo!(),
            },
            EagerExprQualifier::Transient => todo!(),
            EagerExprQualifier::Copyable => Binding::Copy,
            EagerExprQualifier::EvalRef => match contract {
                EagerContract::Pure => Binding::TempRef,
                EagerContract::EvalRef => Binding::EvalRef,
                EagerContract::Move => todo!(),
                EagerContract::TempRefMut => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerExprQualifier::TempRefMut => todo!(),
        }
    }

    pub fn method_opt_output_binding(
        self,
        output_liason: OutputLiason,
        output_contract: EagerContract,
        is_output_ty_copyable: bool,
    ) -> Option<Binding> {
        match output_liason {
            OutputLiason::Transfer => None,
            OutputLiason::MemberAccess { member_liason } => {
                Some(self.member_binding(member_liason, output_contract, is_output_ty_copyable))
            }
        }
    }

    pub fn member_binding(
        self,
        member_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_ty_copyable: bool,
    ) -> Binding {
        if is_member_ty_copyable {
            match member_contract {
                EagerContract::Pure => Binding::Copy,
                EagerContract::Move => todo!(),
                EagerContract::TempRefMut => match member_liason {
                    MemberLiason::Immutable => todo!(),
                    MemberLiason::Mutable => Binding::TempRefMut,
                    MemberLiason::Derived => todo!(),
                },
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            }
        } else {
            // non-copyable
            match self {
                EagerExprQualifier::Copyable => todo!(),
                EagerExprQualifier::PureRef => match member_contract {
                    EagerContract::Pure | EagerContract::Pass => Binding::TempRef,
                    EagerContract::Move => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::TempRefMut => todo!(),
                    EagerContract::EvalRef => todo!(),
                },
                EagerExprQualifier::EvalRef => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::EvalRef => Binding::EvalRef,
                    EagerContract::Move => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::TempRefMut => todo!(),
                    EagerContract::Pass => todo!(),
                },
                EagerExprQualifier::TempRef => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::TempRefMut => todo!(),
                    EagerContract::EvalRef => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::Pass => Binding::TempRef,
                },
                EagerExprQualifier::TempRefMut => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::TempRefMut => Binding::TempRefMut,
                    EagerContract::EvalRef => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::Pass => todo!(),
                },
                EagerExprQualifier::Transient => todo!(),
            }
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
        .variable_use_eager_value_qualifier(contract, range)
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
        match ty.kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Ref,
            } => todo!(),
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
                EagerExprQualifier::EvalRef => todo!(),
                EagerExprQualifier::TempRefMut => todo!(),
            },
            InitKind::Var => match self.qual {
                EagerExprQualifier::Copyable => EagerVariableQualifier::CopyableMut,
                EagerExprQualifier::PureRef => todo!(),
                EagerExprQualifier::TempRef => todo!(),
                EagerExprQualifier::Transient => EagerVariableQualifier::OwnedMut,
                EagerExprQualifier::EvalRef => EagerVariableQualifier::CopyableMut,
                EagerExprQualifier::TempRefMut => todo!(),
            },
            InitKind::Decl => match self.qual {
                EagerExprQualifier::Copyable => EagerVariableQualifier::Copyable,
                EagerExprQualifier::PureRef => EagerVariableQualifier::PureRef,
                EagerExprQualifier::EvalRef => EagerVariableQualifier::EvalRef,
                EagerExprQualifier::TempRef => todo!(),
                EagerExprQualifier::Transient => EagerVariableQualifier::Owned,
                EagerExprQualifier::TempRefMut => todo!(),
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
                EagerExprQualifier::EvalRef => todo!(),
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
