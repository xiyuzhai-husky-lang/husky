use husky_ast::MatchLiason;
use husky_entity_route::{CanonicalQualifier, CanonicalTyKind, EntityRoutePtr};
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
    pub(crate) fn pure_or_pass(
        db: &dyn InferContractSalsaQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        Ok(if db.is_copyable(ty)? {
            LazyContract::Pure
        } else {
            LazyContract::Pass
        })
    }

    pub(crate) fn parameter_lazy_contract(
        db: &dyn InferContractSalsaQueryGroup,
        parameter_liason: ParameterModifier,
        parameter_ty: EntityRoutePtr,
        output: OutputModifier,
        range: TextRange,
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
                        CanonicalQualifier::Intrinsic => todo!(),
                        CanonicalQualifier::EvalRef => todo!(),
                        CanonicalQualifier::TempRef => todo!(),
                        CanonicalQualifier::TempRefMut => todo!(),
                    }
                    // match canonical_parameter_ty.kind() {
                    //     CanonicalTyKind::Intrinsic => match db.is_copyable(parameter_ty)? {
                    //         true => LazyContract::Pure,
                    //         false => LazyContract::Move,
                    //     },
                    //     CanonicalTyKind::Optional => todo!(),
                    //     CanonicalTyKind::EvalRef => todo!(),
                    //     CanonicalTyKind::OptionalEvalRef => LazyContract::EvalRef,
                    //     CanonicalTyKind::TempRefMut => todo!(),
                    // }
                }
                ParameterModifier::TempRefMut => panic!(),
                ParameterModifier::MemberAccess => todo!(),
                ParameterModifier::EvalRef => LazyContract::EvalRef,
                ParameterModifier::TempRef => todo!(),
            }),
            OutputModifier::MemberAccess { .. } => todo!(),
        }
    }

    pub fn field_self_lazy_contract(
        member_modifier: MemberModifier,
        member_contract: LazyContract,
        is_member_copyable: bool,
        range: TextRange,
    ) -> InferResult<LazyContract> {
        Ok(match member_modifier {
            MemberModifier::Immutable | MemberModifier::Mutable => member_contract,
            MemberModifier::Property => LazyContract::EvalRef,
        })
    }

    pub fn from_match(match_liason: MatchLiason) -> Self {
        match match_liason {
            MatchLiason::Pure => LazyContract::Pure,
        }
    }
}
