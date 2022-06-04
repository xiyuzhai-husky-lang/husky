use crate::*;
use ast::MatchLiason;
use entity_route::{EntityRouteKind, EntityRoutePtr};
use infer_error::throw;
use text::TextRange;
use word::RootIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerContract {
    Pure,
    EvalRef,
    Move,
    UseForLetInit,
    UseForVarInit,
    UseForAssignRvalue,
    UseMemberForLetInit,
    UseMemberForVarInit,
    Return,
    TempRefMut,
    Exec,
}

impl EagerContract {
    pub(crate) fn from_this_argument(
        parameter_liason: ParameterLiason,
        output_liason: OutputLiason,
        output_contract: EagerContract,
        is_output_ty_copyable: bool,
        range: TextRange,
    ) -> InferResult<EagerContract> {
        match output_liason {
            OutputLiason::Transfer => {
                match output_contract {
                    EagerContract::Pure
                    | EagerContract::Move
                    | EagerContract::Return
                    | EagerContract::UseForLetInit
                    | EagerContract::UseForVarInit
                    | EagerContract::Exec
                    | EagerContract::UseForAssignRvalue => (),
                    EagerContract::TempRefMut => match output_liason {
                        OutputLiason::Transfer => {
                            throw!(format!("can't mutate transferred output"), range)
                        }
                        OutputLiason::MemberAccess { .. } => todo!(),
                    },
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::EvalRef => todo!(),
                }
                Ok(match parameter_liason {
                    ParameterLiason::Pure => EagerContract::Pure,
                    ParameterLiason::Move | ParameterLiason::MoveMut => EagerContract::Move,
                    ParameterLiason::TempMut => EagerContract::TempRefMut,
                    ParameterLiason::MemberAccess => panic!(),
                    ParameterLiason::EvalRef => EagerContract::EvalRef,
                })
            }
            OutputLiason::MemberAccess { .. } => Ok(match output_contract {
                EagerContract::Pure => EagerContract::Pure,
                EagerContract::Move => EagerContract::Move,
                EagerContract::UseForLetInit => {
                    if is_output_ty_copyable {
                        EagerContract::Pure
                    } else {
                        EagerContract::UseMemberForLetInit
                    }
                }
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => EagerContract::UseMemberForLetInit,
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::TempRefMut => output_contract,
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::EvalRef => EagerContract::EvalRef,
            }),
        }
    }

    pub(crate) fn from_parameter_argument(
        parameter_ty: EntityRoutePtr,
        parameter_liason: ParameterLiason,
        output_liason: OutputLiason,
        output_contract: EagerContract,
        is_output_ty_copyable: bool,
        range: TextRange,
    ) -> InferResult<EagerContract> {
        match parameter_ty.kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Ref,
            } => Ok(EagerContract::EvalRef),
            _ => match output_liason {
                OutputLiason::Transfer => {
                    match output_contract {
                        EagerContract::Pure
                        | EagerContract::Move
                        | EagerContract::Return
                        | EagerContract::UseForLetInit
                        | EagerContract::UseForVarInit
                        | EagerContract::Exec
                        | EagerContract::UseForAssignRvalue => (),
                        EagerContract::TempRefMut => match output_liason {
                            OutputLiason::Transfer => {
                                throw!(format!("can't mutate transferred output"), range)
                            }
                            OutputLiason::MemberAccess { .. } => todo!(),
                        },
                        EagerContract::UseMemberForLetInit => todo!(),
                        EagerContract::UseMemberForVarInit => todo!(),
                        EagerContract::EvalRef => todo!(),
                    }
                    Ok(match parameter_liason {
                        ParameterLiason::Pure => EagerContract::Pure,
                        ParameterLiason::Move | ParameterLiason::MoveMut => EagerContract::Move,
                        ParameterLiason::TempMut => EagerContract::TempRefMut,
                        ParameterLiason::MemberAccess => panic!(),
                        ParameterLiason::EvalRef => EagerContract::EvalRef,
                    })
                }
                OutputLiason::MemberAccess { .. } => Ok(EagerContract::Pure),
            },
        }
    }

    pub fn from_field_access(
        field_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_copyable: bool,
        range: TextRange,
    ) -> InferResult<EagerContract> {
        // infer this contract
        if is_member_copyable {
            Ok(match member_contract {
                EagerContract::Pure => EagerContract::Pure,
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => EagerContract::Pure,
                EagerContract::UseForVarInit => EagerContract::Pure,
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => EagerContract::Pure,
                EagerContract::TempRefMut => match field_liason {
                    MemberLiason::Immutable => {
                        throw!(
                            format!("can't turn an immutable member into ref mut"),
                            range
                        )
                    }
                    MemberLiason::Mutable => EagerContract::TempRefMut,
                    MemberLiason::Derived => todo!(),
                },
                EagerContract::Exec => todo!(),
                EagerContract::EvalRef => todo!(),
            })
        } else {
            match field_liason {
                MemberLiason::Immutable => match member_contract {
                    EagerContract::Pure => Ok(EagerContract::Pure),

                    EagerContract::Move => Ok(EagerContract::Move),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForLetInit | EagerContract::UseMemberForLetInit => {
                        Ok(EagerContract::UseMemberForLetInit)
                    }
                    EagerContract::UseForVarInit | EagerContract::UseMemberForVarInit => {
                        Ok(EagerContract::UseMemberForVarInit)
                    }
                    EagerContract::Return => todo!(),
                    EagerContract::TempRefMut => throw!(
                        format!("can't bind mutable reference to an immutable field"),
                        range
                    ),
                    EagerContract::UseForAssignRvalue => {
                        throw!(format!("can't assign to an immutable field"), range)
                    }
                    EagerContract::EvalRef => Ok(EagerContract::EvalRef),
                },
                MemberLiason::Mutable => match member_contract {
                    EagerContract::Pure => Ok(EagerContract::Pure),
                    EagerContract::Move => Ok(EagerContract::Move),
                    EagerContract::TempRefMut => Ok(EagerContract::TempRefMut),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForLetInit | EagerContract::UseMemberForLetInit => {
                        Ok(EagerContract::UseMemberForLetInit)
                    }
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::EvalRef => Ok(EagerContract::EvalRef),
                },
                MemberLiason::Derived => panic!(),
            }
        }
    }

    pub fn from_match(match_liason: MatchLiason) -> Self {
        match match_liason {
            MatchLiason::Pure => EagerContract::Pure,
        }
    }
}
