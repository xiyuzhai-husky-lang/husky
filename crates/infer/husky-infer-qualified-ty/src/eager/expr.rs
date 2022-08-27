use husky_entity_kind::FieldKind;
use husky_entity_route::{CanonicalQualifier, CanonicalTy, CanonicalTyKind};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerExprQualifiedTy {
    qual: EagerExprQualifier,
    canonical_ty: CanonicalTy,
}

impl std::ops::Deref for EagerExprQualifiedTy {
    type Target = CanonicalTy;

    fn deref(&self) -> &Self::Target {
        &self.canonical_ty
    }
}

impl std::fmt::Display for EagerExprQualifiedTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_option() {
            "Option ".fmt(f)?
        }
        self.qual.fmt(f)?;
        " ".fmt(f);
        self.intrinsic_ty().fmt(f)
    }
}

impl EagerExprQualifiedTy {
    pub fn binding(self, db: &dyn DeclQueryGroup, contract: EagerContract) -> Binding {
        match self.qual {
            EagerExprQualifier::PureRef | EagerExprQualifier::TempRef => match contract {
                EagerContract::Pure | EagerContract::TempRef | EagerContract::Pass => {
                    Binding::TempRef
                }
                _ => panic!(),
            },
            EagerExprQualifier::Transient => match contract {
                EagerContract::Pure => Binding::TempRef,
                EagerContract::Pass | EagerContract::Move => Binding::Move,
                EagerContract::EvalRef => panic!(),
                EagerContract::TempRef => todo!(),
                EagerContract::TempRefMut => todo!(),
            },
            EagerExprQualifier::Copyable => Binding::Copy,
            EagerExprQualifier::EvalRef => match contract {
                EagerContract::Pure => {
                    let is_intrinsic_ty_copyable = db.is_copyable(self.intrinsic_ty()).unwrap();
                    if is_intrinsic_ty_copyable {
                        Binding::DerefCopy
                    } else {
                        Binding::TempRef
                    }
                }
                EagerContract::TempRef => Binding::TempRef,
                EagerContract::EvalRef | EagerContract::Pass => Binding::EvalRef,
                EagerContract::Move => panic!(),
                EagerContract::TempRefMut => panic!(),
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

    #[inline(always)]
    pub fn qual(self) -> EagerExprQualifier {
        self.qual
    }

    pub(crate) fn ty_eager_qualified_ty() -> Self {
        Self {
            qual: EagerExprQualifier::EvalRef,
            canonical_ty: CanonicalTy::new(
                false,
                CanonicalQualifier::EvalRef,
                RootIdentifier::TypeType.into(),
            ),
        }
    }

    pub(crate) fn module_eager_qualified_ty() -> Self {
        Self {
            qual: EagerExprQualifier::EvalRef,
            canonical_ty: CanonicalTy::new(
                false,
                CanonicalQualifier::EvalRef,
                RootIdentifier::ModuleType.into(),
            ),
        }
    }

    pub(crate) fn trait_eager_qualified_ty() -> Self {
        Self {
            qual: EagerExprQualifier::EvalRef,
            canonical_ty: CanonicalTy::new(
                false,
                CanonicalQualifier::EvalRef,
                RootIdentifier::TraitType.into(),
            ),
        }
    }

    pub(crate) fn entity_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        let is_copyable = db.is_copyable(ty)?;
        Ok(Self {
            qual: if is_copyable {
                EagerExprQualifier::Copyable
            } else {
                EagerExprQualifier::EvalRef
            },
            canonical_ty: CanonicalTy::new(
                false,
                if is_copyable {
                    CanonicalQualifier::Intrinsic
                } else {
                    CanonicalQualifier::EvalRef
                },
                ty,
            ),
        })
    }

    pub(crate) fn feature_ty(ty: EntityRoutePtr) -> Self {
        Self {
            qual: EagerExprQualifier::EvalRef,
            canonical_ty: ty.canonicalize().with_eval_ref(),
        }
    }

    pub(crate) fn from_parameter_use(
        db: &dyn InferQualifiedTyQueryGroup,
        parameter_liason: ParameterModifier,
        ty: EntityRoutePtr,
        contract: EagerContract,
        range: TextRange,
    ) -> InferResult<Self> {
        Ok(EagerExprQualifiedTy::new(
            EagerExprQualifier::parameter_use_eager_qualifier(
                db.upcast(),
                ty,
                parameter_liason,
                contract,
                range,
            )?,
            ty,
        ))
    }

    pub fn new(qual0: EagerExprQualifier, raw_ty: EntityRoutePtr) -> Self {
        let canonical_ty = raw_ty.canonicalize();
        let qual = match qual0 {
            EagerExprQualifier::Copyable | EagerExprQualifier::Transient => {
                match canonical_ty.kind() {
                    CanonicalTyKind::Intrinsic | CanonicalTyKind::Optional => qual0,
                    CanonicalTyKind::EvalRef | CanonicalTyKind::OptionalEvalRef => {
                        EagerExprQualifier::EvalRef
                    }
                    CanonicalTyKind::TempRefMut => todo!(),
                }
            }
            EagerExprQualifier::PureRef => match canonical_ty.kind() {
                CanonicalTyKind::Intrinsic => EagerExprQualifier::PureRef,
                CanonicalTyKind::Optional => todo!(),
                CanonicalTyKind::EvalRef => todo!(),
                CanonicalTyKind::OptionalEvalRef => todo!(),
                CanonicalTyKind::TempRefMut => todo!(),
            },
            EagerExprQualifier::EvalRef => match canonical_ty.kind() {
                CanonicalTyKind::Intrinsic => EagerExprQualifier::EvalRef,
                CanonicalTyKind::Optional => todo!(),
                CanonicalTyKind::EvalRef => todo!(),
                CanonicalTyKind::OptionalEvalRef => todo!(),
                CanonicalTyKind::TempRefMut => todo!(),
            },
            EagerExprQualifier::TempRef => match canonical_ty.kind() {
                CanonicalTyKind::Intrinsic => EagerExprQualifier::TempRef,
                CanonicalTyKind::Optional => todo!(),
                CanonicalTyKind::EvalRef => todo!(),
                CanonicalTyKind::OptionalEvalRef => todo!(),
                CanonicalTyKind::TempRefMut => todo!(),
            },
            EagerExprQualifier::TempRefMut => match canonical_ty.kind() {
                CanonicalTyKind::Intrinsic => EagerExprQualifier::TempRefMut,
                CanonicalTyKind::Optional => todo!(),
                CanonicalTyKind::EvalRef => todo!(),
                CanonicalTyKind::OptionalEvalRef => todo!(),
                CanonicalTyKind::TempRefMut => todo!(),
            },
        };
        Self { qual, canonical_ty }
    }

    pub(crate) fn field_eager_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        this_qual: EagerExprQualifier,
        field_kind: FieldKind,
        field_ty: EntityRoutePtr,
        member_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_copyable: bool,
    ) -> InferResult<Self> {
        msg_once!("ad hoc; consider ref");
        Ok(Self::new(
            EagerExprQualifier::field(
                this_qual,
                field_kind,
                member_liason,
                member_contract,
                is_member_copyable,
            ),
            field_ty,
        ))
    }

    pub(crate) fn element_eager_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        this_qual: EagerExprQualifier,
        member_ty: EntityRoutePtr,
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
            member_ty,
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
        Ok(EagerVariableQualifiedTy::new(
            qual,
            self.canonical_ty.intrinsic_ty(),
        ))
    }

    pub fn is_implicitly_castable_to_output(
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
                EagerExprQualifier::PureRef | EagerExprQualifier::TempRef => false,
                EagerExprQualifier::Transient | EagerExprQualifier::Copyable => true,
                EagerExprQualifier::EvalRef => true,
                EagerExprQualifier::TempRefMut => todo!(),
            },
            OutputModifier::MemberAccess { .. } => todo!(),
        }
    }

    pub fn as_ty(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        if !db.is_explicitly_castable(self.intrinsic_ty(), ty)? {
            todo!()
        }
        Ok(Self::new(self.qual, ty))
    }

    pub fn unveil(self) -> Self {
        Self::new(self.qual, self.intrinsic_ty())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EagerExprQualifier {
    Copyable,
    PureRef,
    EvalRef,
    TempRef,
    TempRefMut,
    Transient,
}
impl std::fmt::Display for EagerExprQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            EagerExprQualifier::Copyable => "Copyable",
            EagerExprQualifier::PureRef => "PureRef",
            EagerExprQualifier::EvalRef => "EvalRef",
            EagerExprQualifier::TempRef => "TempRef",
            EagerExprQualifier::TempRefMut => "TempRefMut",
            EagerExprQualifier::Transient => "Transient",
        })
    }
}

impl HuskyDisplay for EagerExprQualifiedTy {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}{: <12?}{} {}{:?}{}",
                husky_print_utils::PINK,
                self.qual,
                husky_print_utils::RESET,
                husky_print_utils::GREEN,
                self.intrinsic_ty(),
                husky_print_utils::RESET,
            )
            .unwrap()
        } else {
            write!(result, "{}", self).unwrap()
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

    pub fn parameter_use_eager_qualifier(
        db: &dyn DeclQueryGroup,
        parameter_ty: EntityRoutePtr,
        parameter_liason: ParameterModifier,
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

    pub fn from_output(output_liason: OutputModifier, is_copyable: bool) -> Self {
        match output_liason {
            OutputModifier::Transfer => Self::transitive(is_copyable),
            OutputModifier::MemberAccess { .. } => todo!(),
        }
    }

    pub fn transitive(is_copyable: bool) -> Self {
        if is_copyable {
            EagerExprQualifier::Copyable
        } else {
            EagerExprQualifier::Transient
        }
    }

    pub fn field(
        this_qual: Self,
        field_kind: FieldKind,
        member_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_copyable: bool,
    ) -> Self {
        // ad hoc
        // merge this into member
        // no need for field_kind
        // just use member_liason
        match field_kind {
            FieldKind::StructOriginal
            | FieldKind::StructDefault
            | FieldKind::StructDerivedEager => Self::member(
                this_qual,
                member_liason,
                member_contract,
                is_member_copyable,
            ),
            FieldKind::StructDerivedLazy => EagerExprQualifier::EvalRef,
            FieldKind::RecordOriginal => todo!(),
            FieldKind::RecordDerived => todo!(),
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
