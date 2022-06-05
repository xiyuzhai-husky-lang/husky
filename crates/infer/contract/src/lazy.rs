use ast::MatchLiason;
use entity_route::EntityRoutePtr;
use infer_error::throw;
use text::TextRange;

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
        parameter_liason: ParameterLiason,
        output: OutputLiason,
        range: TextRange,
    ) -> InferResult<LazyContract> {
        match output {
            OutputLiason::Transfer => Ok(match parameter_liason {
                ParameterLiason::Pure => LazyContract::Pure,
                ParameterLiason::Move | ParameterLiason::MoveMut => LazyContract::Move,
                ParameterLiason::TempRefMut => panic!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::EvalRef => LazyContract::EvalRef,
                ParameterLiason::TempRef => todo!(),
            }),
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    pub fn field_access_lazy_contract(
        field_liason: MemberLiason,
        member_contract: LazyContract,
        is_member_copyable: bool,
        range: TextRange,
    ) -> InferResult<LazyContract> {
        // infer this contract
        Ok(if is_member_copyable {
            match member_contract {
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => LazyContract::Pure,
                LazyContract::Pass | LazyContract::Move => panic!(),
            }
        } else {
            match field_liason {
                MemberLiason::Immutable | MemberLiason::Mutable => member_contract,
                MemberLiason::Derived => todo!(),
            }
        })
    }

    pub fn from_match(match_liason: MatchLiason) -> Self {
        match match_liason {
            MatchLiason::Pure => LazyContract::Pure,
        }
    }
}
