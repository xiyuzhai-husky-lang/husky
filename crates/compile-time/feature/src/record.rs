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
        FeatureRepr::Block(ref block) => block_record_memb(db, block, field_ident),
    }
}

pub(crate) fn expr_record_field(
    db: &dyn FeatureQueryGroup,
    this: &Arc<FeatureExpr>,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    match this.kind {
        FeatureExprKind::Variable { ref value, .. } => expr_record_field(db, value, field_ident),
        FeatureExprKind::RecordOriginalFieldAccess { .. } => todo!(),
        FeatureExprKind::EntityFeature { ref block, .. } => {
            block_record_memb(db, block, field_ident)
        }
        FeatureExprKind::NewRecord {
            ref entity,
            ref opds,
            ..
        } => match entity.variant {
            EntityDefnVariant::Type {
                kind,
                ty_members: ref type_members,
                ref variants,
                ref trait_impls,
                ref members,
                ..
            } => match kind {
                TyKind::Record => {
                    if let Some((idx, type_member)) = type_members.iget_entry(field_ident) {
                        match type_member.variant {
                            EntityDefnVariant::TypeField {
                                ty,
                                ref fieldiant,
                                contract,
                            } => match fieldiant {
                                FieldDefnVariant::StructOriginal => panic!(),
                                FieldDefnVariant::RecordOriginal => opds[idx].clone().into(),
                                FieldDefnVariant::StructDerived { stmts } => {
                                    todo!()
                                }
                                FieldDefnVariant::RecordDerived { stmts } => {
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
                TyKind::Enum => todo!(),
                TyKind::Struct => todo!(),
                TyKind::Primitive => todo!(),
                TyKind::Vec => todo!(),
                TyKind::Array => todo!(),
                TyKind::Other => todo!(),
            },
            _ => panic!(),
        },
        FeatureExprKind::EnumKindLiteral { .. }
        | FeatureExprKind::PrimitiveBinaryOpr { .. }
        | FeatureExprKind::StructOriginalFieldAccess { .. }
        | FeatureExprKind::PrimitiveLiteral(_) => {
            p!(this.kind);
            panic!()
        }
        FeatureExprKind::This { ref repr } => db.record_field_repr(repr.clone(), field_ident),
        FeatureExprKind::GlobalInput => todo!(),
        FeatureExprKind::RoutineCall { .. } => todo!(),
        FeatureExprKind::PatternCall {} => todo!(),
        FeatureExprKind::RecordDerivedFieldAccess { .. } => todo!(),
        FeatureExprKind::ElementAccess { ref opds, .. } => todo!(),
    }
}

pub(crate) fn block_record_memb(
    db: &dyn FeatureQueryGroup,
    this: &Arc<FeatureBlock>,
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
