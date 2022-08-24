use husky_ast::MatchLiason;
use husky_entity_route::{CanonicalEntityRoutePtrKind, EntityRoutePtr};
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
        parameter_liason: ParameterModifier,
        parameter_ty: EntityRoutePtr,
        output: OutputModifier,
        range: TextRange,
    ) -> InferResult<LazyContract> {
        match output {
            OutputModifier::Transfer => Ok(match parameter_liason {
                ParameterModifier::None => match parameter_ty.canonicalize().kind() {
                    CanonicalEntityRoutePtrKind::Intrinsic => LazyContract::Pure,
                    CanonicalEntityRoutePtrKind::Optional => LazyContract::Pure,
                    CanonicalEntityRoutePtrKind::EvalRef => LazyContract::EvalRef,
                    CanonicalEntityRoutePtrKind::OptionalEvalRef => todo!(),
                    CanonicalEntityRoutePtrKind::TempRefMut => todo!(),
                },
                ParameterModifier::Move | ParameterModifier::MoveMut => LazyContract::Move,
                ParameterModifier::TempRefMut => panic!(),
                ParameterModifier::MemberAccess => todo!(),
                ParameterModifier::EvalRef => LazyContract::EvalRef,
                ParameterModifier::TempRef => todo!(),
            }),
            OutputModifier::MemberAccess { .. } => todo!(),
        }
    }

    pub fn field_access_lazy_contract(
        field_liason: MemberLiason,
        member_contract: LazyContract,
        is_member_copyable: bool,
        range: TextRange,
    ) -> InferResult<LazyContract> {
        // infer this contract
        Ok(member_contract)
    }

    pub fn from_match(match_liason: MatchLiason) -> Self {
        match match_liason {
            MatchLiason::Pure => LazyContract::Pure,
        }
    }
}
