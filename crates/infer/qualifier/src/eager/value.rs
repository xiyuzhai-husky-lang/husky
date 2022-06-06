use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerValueQualifiedTy {
    pub qual: EagerValueQualifier,
    pub ty: EntityRoutePtr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EagerValueQualifier {
    Copyable,
    PureRef,
    EvalRef,
    TempRef,
    TempRefMut,
    Transient,
}

impl std::fmt::Debug for EagerValueQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            EagerValueQualifier::Copyable => "Copyable",
            EagerValueQualifier::PureRef => "PureRef",
            EagerValueQualifier::EvalRef => "EvalRef",
            EagerValueQualifier::TempRef => "TempRef",
            EagerValueQualifier::TempRefMut => "RefMut",
            EagerValueQualifier::Transient => "Transient",
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
impl EagerValueQualifier {
    pub fn mutable(&self) -> bool {
        match self {
            EagerValueQualifier::Copyable
            | EagerValueQualifier::PureRef
            | EagerValueQualifier::EvalRef
            | EagerValueQualifier::TempRef
            | EagerValueQualifier::Transient => false,
            EagerValueQualifier::TempRefMut => true,
        }
    }

    pub fn binding(self, contract: EagerContract) -> Binding {
        match self {
            EagerValueQualifier::PureRef | EagerValueQualifier::TempRef => match contract {
                EagerContract::Pure | EagerContract::Pass => Binding::TempRef,
                EagerContract::Move => panic!(),
                EagerContract::TempRefMut => todo!(),
                EagerContract::EvalRef => panic!(),
                EagerContract::TempRef => todo!(),
            },
            EagerValueQualifier::Transient => todo!(),
            EagerValueQualifier::Copyable => Binding::Copy,
            EagerValueQualifier::EvalRef => match contract {
                EagerContract::Pure => Binding::TempRef,
                EagerContract::EvalRef => Binding::EvalRef,
                EagerContract::Move => todo!(),
                EagerContract::TempRefMut => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerValueQualifier::TempRefMut => todo!(),
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
                EagerValueQualifier::Copyable => todo!(),
                EagerValueQualifier::PureRef => match member_contract {
                    EagerContract::Pure | EagerContract::Pass => Binding::TempRef,
                    EagerContract::Move => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::TempRefMut => todo!(),
                    EagerContract::EvalRef => todo!(),
                },
                EagerValueQualifier::EvalRef => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::EvalRef => Binding::EvalRef,
                    EagerContract::Move => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::TempRefMut => todo!(),
                    EagerContract::Pass => todo!(),
                },
                EagerValueQualifier::TempRef => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::TempRefMut => todo!(),
                    EagerContract::EvalRef => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::Pass => Binding::TempRef,
                },
                EagerValueQualifier::TempRefMut => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::TempRefMut => Binding::TempRefMut,
                    EagerContract::EvalRef => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::Pass => todo!(),
                },
                EagerValueQualifier::Transient => todo!(),
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
            EagerValueQualifier::Copyable
        } else {
            EagerValueQualifier::Transient
        }
    }

    pub fn member(this_qual: Self, field_liason: MemberLiason, is_member_copyable: bool) -> Self {
        if is_member_copyable {
            match this_qual {
                EagerValueQualifier::Copyable
                | EagerValueQualifier::PureRef
                | EagerValueQualifier::EvalRef
                | EagerValueQualifier::TempRef
                | EagerValueQualifier::Transient => EagerValueQualifier::Copyable,
                EagerValueQualifier::TempRefMut => EagerValueQualifier::TempRefMut,
            }
        } else {
            match this_qual {
                EagerValueQualifier::Copyable => panic!(),
                EagerValueQualifier::PureRef
                | EagerValueQualifier::EvalRef
                | EagerValueQualifier::TempRef
                | EagerValueQualifier::Transient
                | EagerValueQualifier::TempRefMut => this_qual,
            }
        }
    }
}

impl EagerValueQualifiedTy {
    pub(crate) fn ty_qualified_ty() -> Self {
        Self {
            qual: EagerValueQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
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
            qual: EagerValueQualifier::parameter_use_eager_qualifier(
                db.upcast(),
                ty,
                parameter_liason,
                contract,
                range,
            )?,
            ty: ty.deref_route(),
        })
    }

    pub(crate) fn new(qual: EagerValueQualifier, ty: EntityRoutePtr) -> Self {
        match ty.kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Ref,
            } => todo!(),
            _ => Self { qual, ty },
        }
    }

    pub(crate) fn member_eager_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        this_qual: EagerValueQualifier,
        field_ty: EntityRoutePtr,
        field_liason: MemberLiason,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        msg_once!("ad hoc; consider ref");
        Ok(Self::new(
            EagerValueQualifier::member(this_qual, field_liason, is_field_copyable),
            field_ty,
        ))
    }

    pub(crate) fn init_variable_qualified_ty(
        self,
        init_kind: InitKind,
    ) -> InferResult<EagerVariableQualifiedTy> {
        let qual = match init_kind {
            InitKind::Let => match self.qual {
                EagerValueQualifier::Copyable => EagerVariableQualifier::Copyable,
                EagerValueQualifier::PureRef => EagerVariableQualifier::PureRef,
                EagerValueQualifier::TempRef => EagerVariableQualifier::TempRef,
                EagerValueQualifier::Transient => EagerVariableQualifier::Owned,
                EagerValueQualifier::EvalRef => todo!(),
                EagerValueQualifier::TempRefMut => todo!(),
            },
            InitKind::Var => match self.qual {
                EagerValueQualifier::Copyable => EagerVariableQualifier::CopyableMut,
                EagerValueQualifier::PureRef => todo!(),
                EagerValueQualifier::TempRef => todo!(),
                EagerValueQualifier::Transient => EagerVariableQualifier::OwnedMut,
                EagerValueQualifier::EvalRef => EagerVariableQualifier::CopyableMut,
                EagerValueQualifier::TempRefMut => todo!(),
            },
            InitKind::Decl => match self.qual {
                EagerValueQualifier::Copyable => EagerVariableQualifier::Copyable,
                EagerValueQualifier::PureRef => EagerVariableQualifier::PureRef,
                EagerValueQualifier::EvalRef => EagerVariableQualifier::EvalRef,
                EagerValueQualifier::TempRef => todo!(),
                EagerValueQualifier::Transient => EagerVariableQualifier::Owned,
                EagerValueQualifier::TempRefMut => todo!(),
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
                EagerValueQualifier::PureRef | EagerValueQualifier::TempRef => false,
                EagerValueQualifier::Transient | EagerValueQualifier::Copyable => true,
                EagerValueQualifier::EvalRef => todo!(),
                EagerValueQualifier::TempRefMut => todo!(),
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
