use husky_entity_kind::TyKind;
use husky_entity_semantics::{EntityDefnVariant, FieldDefnVariant};

use crate::*;
use std::sync::Arc;

pub(crate) fn record_field_repr<'eval>(
    db: &dyn FeatureGenQueryGroup,
    this: FeatureRepr,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    match this {
        FeatureRepr::Value { .. } => todo!(),
        FeatureRepr::LazyExpr(ref expr) => expr_record_field(db, expr, field_ident),
        FeatureRepr::LazyBlock(ref block) => block_record_field(db, block, field_ident),
        FeatureRepr::FuncBlock(_) => todo!(),
        FeatureRepr::ProcBlock(_) => todo!(),
        FeatureRepr::TargetInput { .. } => todo!(),
    }
}

pub(crate) fn expr_record_field<'eval>(
    db: &dyn FeatureGenQueryGroup,
    this: &Arc<FeatureLazyExpr>,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    match this.variant {
        FeatureLazyExprVariant::Variable { ref value, .. } => {
            expr_record_field(db, value, field_ident)
        }
        FeatureLazyExprVariant::RecordOriginalField { .. } => todo!(),
        FeatureLazyExprVariant::EntityFeature { ref repr, .. } => {
            record_field_repr(db, repr.clone(), field_ident)
        }
        FeatureLazyExprVariant::NewRecord {
            ref entity,
            ref opds,
            ..
        } => match entity.variant {
            EntityDefnVariant::Ty {
                ty_kind: kind,
                ty_members: ref type_members,
                ref variants,
                ref trait_impls,
                ref members,
                ..
            } => {
                if let Some((idx, type_member)) = type_members.iget_entry(field_ident) {
                    match type_member.variant {
                        EntityDefnVariant::TyField {
                            field_ty: ty,
                            ref field_variant,
                            liason: contract,
                            ..
                        } => match field_variant {
                            FieldDefnVariant::StructOriginal => panic!(),
                            FieldDefnVariant::RecordOriginal => opds[idx].clone().into(),
                            FieldDefnVariant::StructDerivedLazy { defn_repr: block } => {
                                todo!()
                            }
                            FieldDefnVariant::RecordDerived { ref defn_repr } => {
                                todo!()
                            }
                            FieldDefnVariant::StructDefault { default } => todo!(),
                            FieldDefnVariant::StructDerivedEager { derivation: value } => {
                                todo!()
                            }
                        },
                        _ => panic!(),
                    }
                } else {
                    todo!()
                }
            }
            _ => panic!(),
        },
        FeatureLazyExprVariant::PrimitiveBinaryOpr { .. }
        | FeatureLazyExprVariant::StructOriginalField { .. }
        | FeatureLazyExprVariant::Literal(_) => {
            p!(this.variant);
            panic!()
        }
        FeatureLazyExprVariant::ThisValue { ref repr } => {
            db.record_field_repr(repr.clone(), field_ident)
        }
        FeatureLazyExprVariant::EvalInput => todo!(),
        FeatureLazyExprVariant::RoutineCall { .. } => todo!(),
        FeatureLazyExprVariant::RecordDerivedField { .. } => todo!(),
        FeatureLazyExprVariant::ElementAccess { ref opds, .. } => todo!(),
        FeatureLazyExprVariant::StructDerivedLazyField {
            ref this,
            field_ident,
            field_uid,
            ref repr,
        } => todo!(),
        _ => todo!(),
    }
}

pub(crate) fn block_record_field(
    db: &dyn FeatureGenQueryGroup,
    this: &Arc<FeatureLazyBlock>,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    let stmt_features = this.stmt_features();
    if stmt_features.len() == 1 {
        match this.stmts.last().unwrap().variant {
            FeatureLazyStmtVariant::Return { ref result } => {
                db.record_field_repr(result.clone().into(), field_ident)
            }
            FeatureLazyStmtVariant::ConditionFlow { ref branches } => todo!(),
            _ => panic!(),
        }
    } else {
        todo!()
    }
}
