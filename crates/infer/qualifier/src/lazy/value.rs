use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyValueQualifiedTy {
    pub qual: LazyExprQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for LazyValueQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        write!(result, "{: <12?} {:?}", self.qual, self.ty).unwrap()
    }
}

impl LazyValueQualifiedTy {
    pub(crate) fn ty_lazy_qualified_ty() -> Self {
        Self {
            qual: LazyExprQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }

    pub(crate) fn module_lazy_qualified_ty() -> Self {
        Self {
            qual: LazyExprQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::ModuleType),
        }
    }

    pub(crate) fn trait_lazy_qualified_ty() -> Self {
        Self {
            qual: LazyExprQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TraitType),
        }
    }

    pub(crate) fn member_lazy_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        this_qual: LazyExprQualifier,
        field_ty: EntityRoutePtr,
        field_liason: MemberLiason,
    ) -> InferResult<Self> {
        Ok(Self::new(
            LazyExprQualifier::member_lazy_qualifier(
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
            LazyExprQualifier::parameter(parameter_liason, db.is_copyable(ty)?)
                .variable_use(contract)?,
            ty,
        ))
    }

    pub fn new(qual: LazyExprQualifier, ty: EntityRoutePtr) -> Self {
        emsg_once!("handle ref");
        Self { qual, ty }
    }

    pub(crate) fn use_for_init(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let | InitKind::Var => Err(derived!(
                "let or var is not allowed in lazy context".to_string()
            ))?,
            InitKind::Decl => match self.qual {
                LazyExprQualifier::Copyable => LazyExprQualifier::Copyable,
                LazyExprQualifier::PureRef => LazyExprQualifier::PureRef,
                LazyExprQualifier::EvalRef | LazyExprQualifier::Transient => {
                    LazyExprQualifier::EvalRef
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
                LazyExprQualifier::Copyable => true,
                LazyExprQualifier::PureRef => todo!(),
                LazyExprQualifier::EvalRef => todo!(),
                LazyExprQualifier::Transient => true,
            },
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LazyExprQualifier {
    Copyable,
    PureRef,
    EvalRef,
    Transient,
}

impl LazyExprQualifier {
    pub fn feature(is_copyable: bool) -> LazyExprQualifier {
        if is_copyable {
            LazyExprQualifier::Copyable
        } else {
            LazyExprQualifier::EvalRef
        }
    }

    pub fn binding(self, contract: LazyContract) -> Binding {
        match self {
            LazyExprQualifier::PureRef => match contract {
                LazyContract::Pure => Binding::TempRef,
                LazyContract::EvalRef => todo!(),
                LazyContract::Move => todo!(),
                LazyContract::Pass => Binding::TempRef,
            },
            LazyExprQualifier::Transient => todo!(),
            LazyExprQualifier::Copyable => Binding::Copy,
            LazyExprQualifier::EvalRef => Binding::EvalRef,
        }
    }

    pub fn variable_use(self, contract: LazyContract) -> InferResult<Self> {
        Ok(match self {
            LazyExprQualifier::Copyable => match contract {
                LazyContract::Pass => LazyExprQualifier::Copyable,
                LazyContract::EvalRef => LazyExprQualifier::EvalRef,
                LazyContract::Pure => LazyExprQualifier::Copyable,
                LazyContract::Move => todo!(),
            },
            LazyExprQualifier::PureRef => match contract {
                LazyContract::Pass => todo!(),
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => LazyExprQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
            LazyExprQualifier::EvalRef => match contract {
                LazyContract::Pass => LazyExprQualifier::EvalRef,
                LazyContract::EvalRef => LazyExprQualifier::EvalRef,
                LazyContract::Pure => LazyExprQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
            LazyExprQualifier::Transient => todo!(),
        })
    }

    pub fn member_lazy_qualifier(
        this_qual: LazyExprQualifier,
        field_liason: MemberLiason,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        Ok(if is_field_copyable {
            LazyExprQualifier::Copyable
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
                    LazyExprQualifier::Copyable
                } else {
                    LazyExprQualifier::PureRef
                }
            }
            ParameterLiason::EvalRef => LazyExprQualifier::EvalRef,
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
                    LazyExprQualifier::Copyable => todo!(),
                    LazyExprQualifier::PureRef => Binding::TempRef,
                    LazyExprQualifier::EvalRef => Binding::EvalRef,
                    LazyExprQualifier::Transient => Binding::Move,
                },
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => todo!(),
                LazyContract::Move => todo!(),
            }
        }
    }
}
