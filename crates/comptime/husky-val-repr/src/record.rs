use husky_item_semantics::{EntityDefnVariant, FieldDefnVariant};

use crate::*;
use std::sync::Arc;

pub(crate) fn record_field_repr(db: &dyn ValReprDb, this: ValRepr, field_ident: Ident) -> ValRepr {
    match this {
        ValRepr::Value { .. } => todo!(),
        ValRepr::LazyExpr(ref expr) => expr_record_field(db, expr, field_ident),
        ValRepr::LazyBody(ref block) => block_record_field(db, block, field_ident),
        ValRepr::FuncBody(_) => todo!(),
        ValRepr::ProcBody(_) => todo!(),
        ValRepr::TargetInput { .. } => todo!(),
    }
}

pub(crate) fn expr_record_field(db: &dyn ValReprDb, this: &ValExpr, field_ident: Ident) -> ValRepr {
    match this.variant {
        FeatureLazyExprVariant::Variable { ref value, .. } => {
            expr_record_field(db, value, field_ident)
        }
        FeatureLazyExprVariant::RecordOriginalField { .. } => todo!(),
        FeatureLazyExprVariant::EntityFeature { ref repr, .. } => {
            record_field_repr(db, repr.clone(), field_ident)
        }
        FeatureLazyExprVariant::NewRecord {
            ref item, ref opds, ..
        } => match item.variant {
            EntityDefnVariant::EtherealTerm { ref ty_members, .. } => {
                if let Some((idx, type_member)) = ty_members.iget_entry(field_ident) {
                    match type_member.variant {
                        EntityDefnVariant::TyField {
                            ref field_variant, ..
                        } => match field_variant {
                            FieldDefnVariant::StructOriginal => panic!(),
                            FieldDefnVariant::RecordOriginal => opds[idx].clone().into(),
                            FieldDefnVariant::StructDerivedLazy { .. } => {
                                todo!()
                            }
                            FieldDefnVariant::RecordDerived { .. } => {
                                todo!()
                            }
                            FieldDefnVariant::StructDefault { .. } => todo!(),
                            FieldDefnVariant::StructDerivedEager { .. } => {
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
        FeatureLazyExprVariant::Index { .. } => todo!(),
        FeatureLazyExprVariant::StructDerivedLazyField { .. } => todo!(),
        _ => todo!(),
    }
}

pub(crate) fn block_record_field(
    db: &dyn ValReprDb,
    this: &ValBlock,
    field_ident: Ident,
) -> ValRepr {
    let stmt_features = this.stmt_features();
    if stmt_features.len() == 1 {
        match this.stmts.last().unwrap().variant {
            ValStmtData::Return { ref result } => {
                db.record_field_repr(result.clone().into(), field_ident)
            }
            ValStmtData::ConditionFlow { .. } => todo!(),
            _ => panic!(),
        }
    } else {
        todo!()
    }
}
