use husky_entity_route::{CanonicalQualifier, CanonicalTy};
use husky_print_utils::msg_once;
use infer_decl::DeclQueryGroup;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyExprQualifiedTy {
    qual: LazyExprQualifier,
    canonical_ty: CanonicalTy,
}

impl std::ops::Deref for LazyExprQualifiedTy {
    type Target = CanonicalTy;

    fn deref(&self) -> &Self::Target {
        &self.canonical_ty
    }
}

impl HuskyDisplay for LazyExprQualifiedTy {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        write!(result, "{}", self).unwrap()
    }
}

impl std::fmt::Display for LazyExprQualifiedTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.option_level() {
            "?".fmt(f)?
        }
        self.qual.fmt(f)?;
        " ".fmt(f)?;
        self.intrinsic_ty().fmt(f)
    }
}

impl std::fmt::Display for LazyExprQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LazyExprQualifier::Copyable => "Copyable",
            LazyExprQualifier::PureRef => "PureRef",
            LazyExprQualifier::EvalRef => "EvalRef",
            LazyExprQualifier::Transient => "Transient",
        }
        .fmt(f)
    }
}

impl LazyExprQualifiedTy {
    pub fn binding(self, db: &dyn DeclQueryGroup, contract: LazyContract) -> Binding {
        match self.qual {
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
            LazyExprQualifier::EvalRef => match contract {
                LazyContract::Pure => {
                    let is_intrinsic_ty_copyable = db.is_copyable(self.intrinsic_ty()).unwrap();
                    if is_intrinsic_ty_copyable {
                        Binding::DerefCopy
                    } else {
                        Binding::TempRef
                    }
                }
                LazyContract::EvalRef | LazyContract::Pass => Binding::EvalRef,
                LazyContract::Move => panic!(),
            },
        }
    }

    #[inline(always)]
    pub fn qual(self) -> LazyExprQualifier {
        self.qual
    }

    pub(crate) fn entity_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        assert!(ty.is_intrinsic());
        Ok(Self {
            qual: if db.is_copyable(ty)? {
                LazyExprQualifier::Copyable
            } else {
                LazyExprQualifier::EvalRef
            },
            canonical_ty: CanonicalTy::new(0, CanonicalQualifier::EvalRef, ty),
        })
    }

    pub(crate) fn ty_lazy_qualified_ty() -> Self {
        Self {
            qual: LazyExprQualifier::EvalRef,
            canonical_ty: CanonicalTy::new(
                0,
                CanonicalQualifier::EvalRef,
                EntityRoutePtr::Root(RootBuiltinIdentifier::TypeType),
            ),
        }
    }

    pub(crate) fn module_lazy_qualified_ty() -> Self {
        Self {
            qual: LazyExprQualifier::EvalRef,
            canonical_ty: CanonicalTy::new(
                0,
                CanonicalQualifier::EvalRef,
                EntityRoutePtr::Root(RootBuiltinIdentifier::Module),
            ),
        }
    }

    pub(crate) fn trait_lazy_qualified_ty() -> Self {
        Self {
            qual: LazyExprQualifier::EvalRef,
            canonical_ty: CanonicalTy::new(
                0,
                CanonicalQualifier::EvalRef,
                EntityRoutePtr::Root(RootBuiltinIdentifier::Trait),
            ),
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
        parameter_liason: ParameterModifier,
        ty: EntityRoutePtr,
        contract: LazyContract,
    ) -> InferResult<Self> {
        Ok(LazyExprQualifiedTy::new(
            LazyVariableQualifier::parameter(parameter_liason, db.is_copyable(ty)?)
                .variable_use(contract)?,
            ty,
        ))
    }

    pub fn new(qual: LazyExprQualifier, raw_ty: EntityRoutePtr) -> Self {
        let canonical_ty = raw_ty.canonicalize();
        let qual = match qual {
            LazyExprQualifier::Copyable | LazyExprQualifier::Transient => {
                match canonical_ty.qual() {
                    CanonicalQualifier::Intrinsic => qual,
                    CanonicalQualifier::EvalRef => LazyExprQualifier::EvalRef,
                    CanonicalQualifier::TempRef => todo!(),
                    CanonicalQualifier::TempRefMut => todo!(),
                }
            }
            LazyExprQualifier::PureRef => match canonical_ty.qual() {
                CanonicalQualifier::Intrinsic => LazyExprQualifier::PureRef,
                CanonicalQualifier::EvalRef => todo!(),
                CanonicalQualifier::TempRef => todo!(),
                CanonicalQualifier::TempRefMut => todo!(),
            },
            LazyExprQualifier::EvalRef => match canonical_ty.qual() {
                CanonicalQualifier::Intrinsic => LazyExprQualifier::EvalRef,
                CanonicalQualifier::EvalRef => LazyExprQualifier::EvalRef,
                CanonicalQualifier::TempRef => todo!(),
                CanonicalQualifier::TempRefMut => todo!(),
            },
        };
        Self { qual, canonical_ty }
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
        Ok(LazyVariableQualifiedTy {
            qual,
            ty: self.intrinsic_ty(),
        })
    }

    pub(crate) fn is_implicitly_convertible_to_output(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        output_liason: OutputModifier,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicitly_castable(self.intrinsic_ty(), output_ty) {
            return false;
        }
        match output_liason {
            OutputModifier::Transfer => match self.qual {
                LazyExprQualifier::Copyable => true,
                LazyExprQualifier::PureRef => false,
                LazyExprQualifier::EvalRef => true,
                LazyExprQualifier::Transient => true,
            },
            OutputModifier::MemberAccess { .. } => todo!(),
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
        parameter_liason: ParameterModifier,
        is_copyable: bool,
        contract: LazyContract,
    ) -> InferResult<Self> {
        LazyVariableQualifier::parameter(parameter_liason, is_copyable).variable_use(contract)
    }
}
