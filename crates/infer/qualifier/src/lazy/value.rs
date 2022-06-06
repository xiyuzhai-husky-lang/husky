use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyValueQualifiedTy {
    pub qual: LazyValueQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for LazyValueQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        write!(result, "{: <12?} {:?}", self.qual, self.ty).unwrap()
    }
}

impl LazyValueQualifiedTy {
    pub(crate) fn ty_ty() -> Self {
        Self {
            qual: LazyValueQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }

    pub(crate) fn trait_ty() -> Self {
        Self {
            qual: LazyValueQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TraitType),
        }
    }

    pub(crate) fn member_lazy_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        this_qual: LazyValueQualifier,
        field_ty: EntityRoutePtr,
        field_liason: MemberLiason,
    ) -> InferResult<Self> {
        Ok(Self::new(
            LazyValueQualifier::member_lazy_qualifier(
                this_qual,
                field_liason,
                db.is_copyable(field_ty)?,
            )?,
            field_ty,
        ))
    }

    pub(crate) fn parameter_use_lazy_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        parameter_liason: ParameterLiason,
        ty: EntityRoutePtr,
        contract: LazyContract,
    ) -> InferResult<Self> {
        Ok(LazyValueQualifiedTy::new(
            LazyValueQualifier::parameter(parameter_liason, db.is_copyable(ty)?)
                .variable_use(contract)?,
            ty,
        ))
    }

    pub fn new(qual: LazyValueQualifier, ty: EntityRoutePtr) -> Self {
        emsg_once!("handle ref");
        Self { qual, ty }
    }

    pub(crate) fn use_for_init(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let | InitKind::Var => Err(derived!(
                "let or var is not allowed in lazy context".to_string()
            ))?,
            InitKind::Decl => match self.qual {
                LazyValueQualifier::Copyable => LazyValueQualifier::Copyable,
                LazyValueQualifier::PureRef => LazyValueQualifier::PureRef,
                LazyValueQualifier::EvalRef | LazyValueQualifier::Transient => {
                    LazyValueQualifier::EvalRef
                }
            },
        };
        Ok(Self { qual, ty: self.ty })
    }

    pub(crate) fn is_implicitly_convertible_to_output(
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
                LazyValueQualifier::Copyable => true,
                LazyValueQualifier::PureRef => todo!(),
                LazyValueQualifier::EvalRef => todo!(),
                LazyValueQualifier::Transient => true,
            },
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LazyValueQualifier {
    Copyable,
    PureRef,
    EvalRef,
    Transient,
}

impl LazyValueQualifier {
    pub fn feature(is_copyable: bool) -> LazyValueQualifier {
        if is_copyable {
            LazyValueQualifier::Copyable
        } else {
            LazyValueQualifier::EvalRef
        }
    }

    pub fn binding(self, contract: LazyContract) -> Binding {
        match self {
            LazyValueQualifier::PureRef => match contract {
                LazyContract::Pure => Binding::TempRef,
                LazyContract::EvalRef => todo!(),
                LazyContract::Move => todo!(),
                LazyContract::Pass => Binding::TempRef,
            },
            LazyValueQualifier::Transient => todo!(),
            LazyValueQualifier::Copyable => Binding::Copy,
            LazyValueQualifier::EvalRef => Binding::EvalRef,
        }
    }

    pub fn variable_use(self, contract: LazyContract) -> InferResult<Self> {
        Ok(match self {
            LazyValueQualifier::Copyable => match contract {
                LazyContract::Pass => LazyValueQualifier::Copyable,
                LazyContract::EvalRef => LazyValueQualifier::EvalRef,
                LazyContract::Pure => LazyValueQualifier::Copyable,
                LazyContract::Move => todo!(),
            },
            LazyValueQualifier::PureRef => match contract {
                LazyContract::Pass => todo!(),
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => LazyValueQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
            LazyValueQualifier::EvalRef => match contract {
                LazyContract::Pass => LazyValueQualifier::EvalRef,
                LazyContract::EvalRef => LazyValueQualifier::EvalRef,
                LazyContract::Pure => LazyValueQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
            LazyValueQualifier::Transient => todo!(),
        })
    }

    pub fn member_lazy_qualifier(
        this_qual: LazyValueQualifier,
        field_liason: MemberLiason,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        Ok(if is_field_copyable {
            LazyValueQualifier::Copyable
        } else {
            // non-copyable
            this_qual
        })
    }

    pub fn parameter_use_lazy_qualifier(
        input_liason: ParameterLiason,
        is_copyable: bool,
        contract: LazyContract,
    ) -> InferResult<Self> {
        Self::parameter(input_liason, is_copyable).variable_use(contract)
    }

    pub fn parameter(parameter_liason: ParameterLiason, is_copyable: bool) -> Self {
        match parameter_liason {
            ParameterLiason::Pure => {
                if is_copyable {
                    LazyValueQualifier::Copyable
                } else {
                    LazyValueQualifier::PureRef
                }
            }
            ParameterLiason::EvalRef => LazyValueQualifier::EvalRef,
            ParameterLiason::Move => todo!(),
            ParameterLiason::TempRefMut => todo!(),
            ParameterLiason::MoveMut => todo!(),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::TempRef => todo!(),
        }
    }

    pub fn method_opt_output_binding(
        self,
        output_liason: OutputLiason,
        output_contract: LazyContract,
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
        member_contract: LazyContract,
        is_member_ty_copyable: bool,
    ) -> Binding {
        if is_member_ty_copyable {
            match member_contract {
                LazyContract::Pass => Binding::Copy,
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => Binding::Copy,
                LazyContract::Move => todo!(),
            }
        } else {
            // non-copyable
            match member_contract {
                LazyContract::Pass => match self {
                    LazyValueQualifier::Copyable => todo!(),
                    LazyValueQualifier::PureRef => Binding::TempRef,
                    LazyValueQualifier::EvalRef => Binding::EvalRef,
                    LazyValueQualifier::Transient => Binding::Move,
                },
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => todo!(),
                LazyContract::Move => todo!(),
            }
        }
    }
}
