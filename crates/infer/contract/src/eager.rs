use crate::*;
use ast::MatchLiason;
use entity_route::{EntityRouteKind, EntityRoutePtr};
use infer_decl::DeclQueryGroup;
use infer_error::throw;
use text::TextRange;
use word::RootIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerContract {
    Pure,
    Move,
    Pass,
    EvalRef,
    TempRef,
    TempRefMut,
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
                    EagerContract::Pure | EagerContract::Move | EagerContract::Pass => (),
                    EagerContract::TempRefMut => match output_liason {
                        OutputLiason::Transfer => {
                            throw!(format!("can't mutate transferred output"), range)
                        }
                        OutputLiason::MemberAccess { .. } => todo!(),
                    },
                    EagerContract::EvalRef => todo!(),
                    EagerContract::TempRef => todo!(),
                }
                Ok(match parameter_liason {
                    ParameterLiason::Pure => EagerContract::Pure,
                    ParameterLiason::Move | ParameterLiason::MoveMut => EagerContract::Move,
                    ParameterLiason::TempMut => EagerContract::TempRefMut,
                    ParameterLiason::MemberAccess => panic!(),
                    ParameterLiason::EvalRef => EagerContract::EvalRef,
                })
            }
            OutputLiason::MemberAccess { .. } => Ok(output_contract),
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
                        | EagerContract::Pure
                        | EagerContract::Pass => (),
                        EagerContract::TempRefMut => match output_liason {
                            OutputLiason::Transfer => {
                                throw!(format!("can't mutate transferred output"), range)
                            }
                            OutputLiason::MemberAccess { .. } => todo!(),
                        },
                        EagerContract::EvalRef => todo!(),
                        EagerContract::TempRef => todo!(),
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
                EagerContract::Pure => todo!(),
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            })
        } else {
            match field_liason {
                MemberLiason::Immutable => match member_contract {
                    EagerContract::TempRefMut => throw!(
                        format!("can't bind mutable reference to an immutable field"),
                        range
                    ),
                    _ => Ok(member_contract),
                },
                MemberLiason::Mutable => match member_contract {
                    EagerContract::Pure => Ok(EagerContract::Pure),
                    EagerContract::Move => Ok(EagerContract::Move),
                    EagerContract::TempRefMut => Ok(EagerContract::TempRefMut),
                    EagerContract::Pure => todo!(),
                    EagerContract::EvalRef => Ok(EagerContract::EvalRef),
                    EagerContract::TempRef => todo!(),
                    EagerContract::Pass => todo!(),
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

    pub fn init_contract(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> InferResult<Self> {
        Ok(if db.is_copyable(ty)? {
            EagerContract::Pure
        } else {
            EagerContract::Pass
        })
    }

    pub fn ret_contract(
        db: &dyn DeclQueryGroup,
        output_ty: EntityRoutePtr,
        return_ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        Ok(if output_ty.kind == return_ty.kind {
            if db.is_copyable(output_ty)? {
                EagerContract::Pure
            } else {
                EagerContract::Move
            }
        } else {
            p!(output_ty, return_ty);
            todo!()
        })
    }
}
