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
                ParameterLiason::Move => todo!(),
                ParameterLiason::TempRefMut => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::EvalRef => todo!(),
            }),
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }
}
