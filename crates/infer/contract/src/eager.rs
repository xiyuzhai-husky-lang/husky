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
    MoveMut,
    Exec,
}

impl EagerContract {
    pub(crate) fn from_this(
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
                    EagerContract::MoveMut => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::EvalRef => todo!(),
                }
                Ok(match parameter_liason {
                    ParameterLiason::Pure => EagerContract::Pure,
                    ParameterLiason::Move | ParameterLiason::MoveMut => EagerContract::Move,
                    ParameterLiason::TempRefMut => EagerContract::TempRefMut,
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
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::EvalRef => todo!(),
            }),
        }
    }

    pub(crate) fn from_parameter(
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
                        EagerContract::MoveMut => todo!(),
                        EagerContract::UseMemberForLetInit => todo!(),
                        EagerContract::UseMemberForVarInit => todo!(),
                        EagerContract::EvalRef => todo!(),
                    }
                    Ok(match parameter_liason {
                        ParameterLiason::Pure => EagerContract::Pure,
                        ParameterLiason::Move | ParameterLiason::MoveMut => EagerContract::Move,
                        ParameterLiason::TempRefMut => EagerContract::TempRefMut,
                        ParameterLiason::MemberAccess => panic!(),
                        ParameterLiason::EvalRef => EagerContract::EvalRef,
                    })
                }
                OutputLiason::MemberAccess { .. } => Ok(EagerContract::Pure),
            },
        }
    }

    pub fn this_contract_from_field_access(
        field_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_copyable: bool,
        range: TextRange,
    ) -> InferResult<EagerContract> {
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
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::EvalRef => todo!(),
            })
        } else {
            match field_liason {
                MemberLiason::Immutable => match member_contract {
                    EagerContract::Pure => Ok(EagerContract::Pure),

                    EagerContract::Move | EagerContract::MoveMut => Ok(EagerContract::Move),
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
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForLetInit | EagerContract::UseMemberForLetInit => {
                        Ok(EagerContract::UseMemberForLetInit)
                    }
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::EvalRef => todo!(),
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

    // pub fn lazy(self) -> LazyContract {
    //     match self {
    //         MatchContract::Pure => LazyContract::Pure,
    //     }
    // }

    // pub fn this_lazy_contract(
    //     self,
    //     field_contract: LazyContract,
    //     is_member_copyable: bool,
    // ) -> VMCompileResult<LazyContract> {
    //     Ok(if is_member_copyable {
    //         match field_contract {
    //             LazyContract::Init => todo!(),
    //             LazyContract::Return => todo!(),
    //             LazyContract::UseMemberForInit => todo!(),
    //             LazyContract::UseMemberForReturn => todo!(),
    //             LazyContract::EvalRef => todo!(),
    //             LazyContract::Pure => LazyContract::Pure,
    //             LazyContract::Move => todo!(),
    //         }
    //     } else {
    //         match self {
    //             MemberLiason::Immutable | MemberLiason::Mutable => match field_contract {
    //                 LazyContract::Move => LazyContract::Move,
    //                 LazyContract::Pure => LazyContract::Pure,
    //                 LazyContract::EvalRef => todo!(),
    //                 LazyContract::Init => todo!(),
    //                 LazyContract::Return => todo!(),
    //                 LazyContract::UseMemberForInit => todo!(),
    //                 LazyContract::UseMemberForReturn => todo!(),
    //             },
    //             MemberLiason::Derived => todo!(),
    //         }
    //     })
    // }
}
