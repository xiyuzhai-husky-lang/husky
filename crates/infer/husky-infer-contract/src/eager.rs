use crate::*;
use husky_ast::{MatchLiason, RawReturnContext, RawReturnContextKind};
use husky_entity_route::{CanonicalTyKind, EntityRoutePtr, EntityRouteVariant};
use husky_infer_error::throw;
use husky_text::TextRange;
use husky_word::RootIdentifier;
use infer_decl::DeclQueryGroup;

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
    pub(crate) fn parameter_eager_contract(
        db: &dyn InferContractSalsaQueryGroup,
        parameter_liason: ParameterModifier,
        parameter_ty: EntityRoutePtr,
        output_liason: OutputModifier,
        range: TextRange,
    ) -> InferResult<EagerContract> {
        Ok(match parameter_ty.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => EagerContract::EvalRef,
            _ => match output_liason {
                OutputModifier::Transfer => match parameter_liason {
                    ParameterModifier::None => match parameter_ty.canonicalize().kind() {
                        CanonicalTyKind::Intrinsic => EagerContract::Pure,
                        CanonicalTyKind::Optional => EagerContract::Pure,
                        CanonicalTyKind::EvalRef => EagerContract::EvalRef,
                        CanonicalTyKind::OptionalEvalRef => todo!(),
                        CanonicalTyKind::TempRefMut => todo!(),
                    },
                    ParameterModifier::Owned | ParameterModifier::OwnedMut => {
                        let canonical_parameter_ty = parameter_ty.canonicalize();
                        match canonical_parameter_ty.kind() {
                            CanonicalTyKind::Intrinsic => match db.is_copyable(parameter_ty)? {
                                true => EagerContract::Pure,
                                false => EagerContract::Move,
                            },
                            CanonicalTyKind::Optional => todo!(),
                            CanonicalTyKind::EvalRef => todo!(),
                            CanonicalTyKind::OptionalEvalRef => EagerContract::EvalRef,
                            CanonicalTyKind::TempRefMut => todo!(),
                        }
                    }
                    ParameterModifier::TempRefMut => EagerContract::TempRefMut,
                    ParameterModifier::MemberAccess => panic!(),
                    ParameterModifier::EvalRef => EagerContract::EvalRef,
                    ParameterModifier::TempRef => todo!(),
                },
                OutputModifier::MemberAccess { .. } => EagerContract::Pure,
            },
        })
    }

    pub(crate) fn method_call_this_eager_contract(
        parameter_liason: ParameterModifier,
        output_liason: OutputModifier,
        output_contract: EagerContract,
    ) -> EagerContract {
        match output_liason {
            OutputModifier::Transfer => match parameter_liason {
                ParameterModifier::None => EagerContract::Pure,
                ParameterModifier::Owned | ParameterModifier::OwnedMut => EagerContract::Move,
                ParameterModifier::TempRefMut => EagerContract::TempRefMut,
                ParameterModifier::MemberAccess => panic!(),
                ParameterModifier::EvalRef => EagerContract::EvalRef,
                ParameterModifier::TempRef => todo!(),
            },
            OutputModifier::MemberAccess { .. } => output_contract,
        }
    }

    pub fn field_self_eager_contract(
        member_modifier: MemberModifier,
        member_contract: EagerContract,
        is_member_copyable: bool,
        range: TextRange,
    ) -> InferResult<EagerContract> {
        if !member_modifier.allow_mutable() && member_contract.require_mutable() {
            todo!()
        }
        Ok(match member_modifier {
            MemberModifier::Immutable | MemberModifier::Mutable => member_contract,
            MemberModifier::Property => EagerContract::EvalRef,
        })
    }

    pub fn require_mutable(self) -> bool {
        match self {
            EagerContract::TempRefMut => true,
            EagerContract::Pure
            | EagerContract::Move
            | EagerContract::Pass
            | EagerContract::EvalRef
            | EagerContract::TempRef => false,
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
        expr_ty: EntityRoutePtr,
        return_context: RawReturnContext,
    ) -> InferResult<Self> {
        match return_context.kind {
            RawReturnContextKind::Normal => Ok(if output_ty.variant == expr_ty.variant {
                if db.is_copyable(output_ty)? {
                    EagerContract::Pure
                } else {
                    EagerContract::Move
                }
            } else if output_ty.variant
                == (EntityRouteVariant::Root {
                    ident: RootIdentifier::Option,
                })
            {
                if output_ty.entity_route_argument(0) == expr_ty {
                    if db.is_copyable(expr_ty)? {
                        EagerContract::Pure
                    } else {
                        EagerContract::Move
                    }
                } else {
                    todo!()
                }
            } else {
                todo!()
            }),
            RawReturnContextKind::Feature | RawReturnContextKind::LazyField => {
                if output_ty.is_eval_ref() {
                    todo!("warn: output ty should be dereferenced")
                } else if output_ty == expr_ty {
                    if db.is_copyable(expr_ty)? {
                        Ok(EagerContract::Pure)
                    } else {
                        Ok(EagerContract::Pass)
                    }
                } else if output_ty.variant
                    == (EntityRouteVariant::Root {
                        ident: RootIdentifier::Option,
                    })
                {
                    if output_ty.entity_route_argument(0) == expr_ty {
                        Ok(if db.is_copyable(expr_ty)? {
                            EagerContract::Pure
                        } else {
                            EagerContract::Move
                        })
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
        }
    }
}
