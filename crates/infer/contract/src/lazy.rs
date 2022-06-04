use ast::MatchLiason;
use text::TextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyContract {
    Init,
    Return,
    UseMemberForInit,
    UseMemberForReturn,
    EvalRef,
    Pure,
    Move,
}

impl LazyContract {
    pub(crate) fn from_parameter(
        parameter_liason: ParameterLiason,
        output: OutputLiason,
        range: TextRange,
    ) -> InferResult<LazyContract> {
        match output {
            OutputLiason::Transfer => Ok(match parameter_liason {
                ParameterLiason::Pure => LazyContract::Pure,
                ParameterLiason::Move | ParameterLiason::MoveMut => LazyContract::Move,
                ParameterLiason::TempMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::EvalRef => todo!(),
            }),
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    pub fn from_field_access(
        field_liason: MemberLiason,
        field_contract: LazyContract,
        is_member_copyable: bool,
        range: TextRange,
    ) -> InferResult<LazyContract> {
        // infer this contract
        Ok(if is_member_copyable {
            match field_contract {
                LazyContract::Init => todo!(),
                LazyContract::Return => todo!(),
                LazyContract::UseMemberForInit => todo!(),
                LazyContract::UseMemberForReturn => todo!(),
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => LazyContract::Pure,
                LazyContract::Move => todo!(),
            }
        } else {
            match field_liason {
                MemberLiason::Immutable | MemberLiason::Mutable => match field_contract {
                    LazyContract::Move => LazyContract::Move,
                    LazyContract::Pure => LazyContract::Pure,
                    LazyContract::EvalRef => todo!(),
                    LazyContract::Init => todo!(),
                    LazyContract::Return => todo!(),
                    LazyContract::UseMemberForInit => todo!(),
                    LazyContract::UseMemberForReturn => todo!(),
                },
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
