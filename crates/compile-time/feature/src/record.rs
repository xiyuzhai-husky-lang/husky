use entity_kind::TyKind;
use semantics_entity::{EntityDefnVariant, FieldDefnVariant};

use crate::*;
use std::sync::Arc;

pub(crate) fn record_field_repr(
    db: &dyn FeatureQueryGroup,
    this: FeatureRepr,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    match this {
        FeatureRepr::Expr(ref expr) => expr_record_field(db, expr, field_ident),
        FeatureRepr::LazyBlock(ref block) => block_record_field(db, block, field_ident),
        FeatureRepr::FuncBlock(_) => todo!(),
        FeatureRepr::ProcBlock(_) => todo!(),
    }
}

pub(crate) fn expr_record_field(
    db: &dyn FeatureQueryGroup,
    this: &Arc<FeatureExpr>,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    match this.variant {
        FeatureExprVariant::Variable { ref value, .. } => expr_record_field(db, value, field_ident),
        FeatureExprVariant::RecordOriginalFieldAccess { .. } => todo!(),
        FeatureExprVariant::EntityFeature { ref repr, .. } => {
            record_field_repr(db, repr.clone(), field_ident)
        }
        FeatureExprVariant::NewRecord {
            ref entity,
            ref opds,
            ..
        } => match entity.variant {
            EntityDefnVariant::Ty {
                kind,
                ty_members: ref type_members,
                ref variants,
                ref trait_impls,
                ref members,
                ..
            } => {
                if let Some((idx, type_member)) = type_members.iget_entry(field_ident) {
                    match type_member.variant {
                        EntityDefnVariant::TyField {
                            ty,
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
        FeatureExprVariant::EnumKindLiteral { .. }
        | FeatureExprVariant::PrimitiveBinaryOpr { .. }
        | FeatureExprVariant::StructOriginalFieldAccess { .. }
        | FeatureExprVariant::PrimitiveLiteral(_) => {
            p!(this.variant);
            panic!()
        }
        FeatureExprVariant::ThisValue { ref repr } => {
            db.record_field_repr(repr.clone(), field_ident)
        }
        FeatureExprVariant::EvalInput => todo!(),
        FeatureExprVariant::RoutineCall { .. } => todo!(),
        FeatureExprVariant::PatternCall {} => todo!(),
        FeatureExprVariant::RecordDerivedFieldAccess { .. } => todo!(),
        FeatureExprVariant::ElementAccess { ref opds, .. } => todo!(),
        FeatureExprVariant::StructDerivedLazyFieldAccess {
            ref this,
            field_ident,
            ref repr,
        } => todo!(),
    }
}

pub(crate) fn block_record_field(
    db: &dyn FeatureQueryGroup,
    this: &Arc<FeatureLazyBlock>,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    let stmt_features = this.stmt_features();
    if stmt_features.len() == 1 {
        match this.stmts.last().unwrap().variant {
            FeatureStmtVariant::Return { ref result } => {
                db.record_field_repr(result.clone().into(), field_ident)
            }
            FeatureStmtVariant::ConditionFlow { ref branches } => todo!(),
            _ => panic!(),
        }
    } else {
        todo!()
    }
}
