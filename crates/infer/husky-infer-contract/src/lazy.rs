use husky_ast::MatchLiason;
use husky_entity_route::{CanonicalQualifier, CanonicalTy, CanonicalTyKind, EntityRoutePtr};
use husky_text::TextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyContract {
    Pass,
    EvalRef,
    Pure,
    Move,
}

impl LazyContract {
    pub(crate) fn init_contract(
        db: &dyn InferContractSalsaQueryGroup,
        ty: CanonicalTy,
    ) -> InferResult<Self> {
        match ty.qual() {
            CanonicalQualifier::Intrinsic => Ok(if db.is_copyable(ty.intrinsic_ty())? {
                LazyContract::Pure
            } else {
                LazyContract::Pass
            }),
            CanonicalQualifier::EvalRef => Ok(LazyContract::EvalRef),
            CanonicalQualifier::TempRef => todo!(),
            CanonicalQualifier::TempRefMut => todo!(),
        }
    }

    pub(crate) fn parameter_lazy_contract(
        db: &dyn InferContractSalsaQueryGroup,
        parameter_liason: ParameterModifier,
        parameter_ty: EntityRoutePtr,
        output: OutputModifier,
    ) -> InferResult<LazyContract> {
        match output {
            OutputModifier::Transfer => Ok(match parameter_liason {
                ParameterModifier::None => match parameter_ty.canonicalize().kind() {
                    CanonicalTyKind::Intrinsic => LazyContract::Pure,
                    CanonicalTyKind::Optional => LazyContract::Pure,
                    CanonicalTyKind::EvalRef => LazyContract::EvalRef,
                    CanonicalTyKind::OptionalEvalRef => todo!(),
                    CanonicalTyKind::TempRefMut => todo!(),
                },
                ParameterModifier::Owned | ParameterModifier::OwnedMut => {
                    let canonical_parameter_ty = parameter_ty.canonicalize();
                    match canonical_parameter_ty.qual() {
                        CanonicalQualifier::Intrinsic => match db.is_copyable(parameter_ty)? {
                            true => LazyContract::Pure,
                            false => LazyContract::Move,
                        },
                        CanonicalQualifier::EvalRef => todo!(),
                        CanonicalQualifier::TempRef => todo!(),
                        CanonicalQualifier::TempRefMut => todo!(),
                    }
                }
                ParameterModifier::TempRefMut => panic!(),
                ParameterModifier::MemberAccess => todo!(),
                ParameterModifier::EvalRef => LazyContract::EvalRef,
                ParameterModifier::TempRef => todo!(),
            }),
            OutputModifier::MemberAccess { .. } => todo!(),
        }
    }

    pub fn member_self_lazy_contract(
        member_modifier: MemberModifier,
        member_contract: LazyContract,
        member_ty: EntityRoutePtr,
    ) -> InferResult<LazyContract> {
        Ok(match member_modifier {
            MemberModifier::Immutable | MemberModifier::Mutable => {
                let canonical_member_ty = member_ty.canonicalize();
                match canonical_member_ty.qual() {
                    CanonicalQualifier::Intrinsic => member_contract,
                    CanonicalQualifier::EvalRef => LazyContract::Pure,
                    CanonicalQualifier::TempRef => todo!(),
                    CanonicalQualifier::TempRefMut => todo!(),
                }
            }
            MemberModifier::Property => LazyContract::EvalRef,
        })
    }

    pub fn from_match(match_liason: MatchLiason) -> Self {
        match match_liason {
            MatchLiason::Pure => LazyContract::Pure,
        }
    }
}
