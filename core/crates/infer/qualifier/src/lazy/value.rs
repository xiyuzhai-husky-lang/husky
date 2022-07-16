use print_utils::msg_once;

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
        member_ty: EntityRoutePtr,
        member_contract: LazyContract,
    ) -> InferResult<Self> {
        msg_once!("handle ref");
        Ok(Self::new(
            LazyExprQualifier::member_lazy_qualifier(
                this_qual,
                db.is_copyable(member_ty)?,
                member_contract,
            )?,
            member_ty,
        ))
    }

    pub(crate) fn parameter_use_lazy_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        parameter_liason: ParameterLiason,
        ty: EntityRoutePtr,
        contract: LazyContract,
    ) -> InferResult<Self> {
        Ok(LazyValueQualifiedTy::new(
            LazyVariableQualifier::parameter(parameter_liason, db.is_copyable(ty)?)
                .variable_use(contract)?,
            ty,
        ))
    }

    pub fn new(qual: LazyExprQualifier, ty: EntityRoutePtr) -> Self {
        msg_once!("handle ref");
        Self { qual, ty }
    }

    pub(crate) fn use_for_init(self, init_kind: InitKind) -> InferResult<LazyVariableQualifiedTy> {
        let qual = match init_kind {
            InitKind::Let | InitKind::Var => Err(derived!(
                "let or var is not allowed in lazy context".to_string()
            ))?,
            InitKind::Decl => match self.qual {
                LazyExprQualifier::Copyable => LazyVariableQualifier::Copyable,
                LazyExprQualifier::PureRef => LazyVariableQualifier::PureRef,
                LazyExprQualifier::EvalRef | LazyExprQualifier::Transient => {
                    LazyVariableQualifier::EvalRef
                }
            },
        };
        Ok(LazyVariableQualifiedTy { qual, ty: self.ty })
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
                LazyExprQualifier::PureRef => false,
                LazyExprQualifier::EvalRef => true,
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
                LazyContract::Pure | LazyContract::Pass => Binding::TempRef,
                _ => panic!(),
            },
            LazyExprQualifier::Transient => match contract {
                LazyContract::Pure => Binding::TempRef,
                LazyContract::Pass | LazyContract::Move => Binding::Move,
                LazyContract::EvalRef => panic!(),
            },
            LazyExprQualifier::Copyable => Binding::Copy,
            LazyExprQualifier::EvalRef => Binding::EvalRef,
        }
    }

    pub fn member_lazy_qualifier(
        this_qual: LazyExprQualifier,
        is_field_copyable: bool,
        member_contract: LazyContract,
    ) -> InferResult<Self> {
        Ok(if is_field_copyable {
            match member_contract {
                LazyContract::Pure => LazyExprQualifier::Copyable,
                LazyContract::EvalRef => LazyExprQualifier::EvalRef,
                _ => panic!(),
            }
        } else {
            // non-copyable
            match member_contract {
                LazyContract::Pure => LazyExprQualifier::PureRef,
                LazyContract::EvalRef => LazyExprQualifier::EvalRef,
                LazyContract::Pass | LazyContract::Move => this_qual,
            }
        })
    }

    pub fn parameter_use_lazy_qualifier(
        parameter_liason: ParameterLiason,
        is_copyable: bool,
        contract: LazyContract,
    ) -> InferResult<Self> {
        LazyVariableQualifier::parameter(parameter_liason, is_copyable).variable_use(contract)
    }
}
