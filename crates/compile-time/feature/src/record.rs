use entity_kind::TypeKind;
use semantics_entity::EntityDefnVariant;
use sync_utils::ARwLock;

use crate::*;
use std::{collections::HashMap, sync::Arc};

pub(crate) fn record_field_repr(
    db: &dyn FeatureQueryGroup,
    this: FeatureRepr,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    match this {
        FeatureRepr::Expr(ref expr) => expr_record_memb(db, expr, field_ident),
        FeatureRepr::Block(ref block) => block_record_memb(db, block, field_ident),
    }
}

pub(crate) fn expr_record_memb(
    db: &dyn FeatureQueryGroup,
    this: &Arc<FeatureExpr>,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    match this.kind {
        FeatureExprKind::Variable { ref value, .. } => expr_record_memb(db, value, field_ident),
        FeatureExprKind::RecordFieldAccess { .. } => todo!(),
        FeatureExprKind::FeatureBlock { ref block, .. } => {
            block_record_memb(db, block, field_ident)
        }
        FeatureExprKind::NewRecord {
            ref entity,
            ref opds,
            ..
        } => match entity.kind() {
            EntityDefnVariant::Type {
                kind,
                ref type_members,
                ref variants,
                ref trait_impls,
                ref members,
            } => match kind {
                TypeKind::Record => {
                    todo!("make a difference between derived and original")
                    // if let Some(idx) = ty_defn.fields.position(field_ident) {
                    //     opds[idx].clone().into()
                    // } else if let Some(field_feature) = fields.get(field_ident) {
                    //     FeatureBlock::new(
                    //         db,
                    //         Some(this.clone().into()),
                    //         &field_feature.stmts,
                    //         &[],
                    //         db.features(),
                    //     )
                    //     .into()
                    // } else {
                    //     todo!()
                    // }
                }
                _ => panic!(),
                TypeKind::Enum => todo!(),
                TypeKind::Struct => todo!(),
                TypeKind::Primitive => todo!(),
                TypeKind::Vec => todo!(),
                TypeKind::Array => todo!(),
                TypeKind::Other => todo!(),
            },
            _ => panic!(),
        },
        FeatureExprKind::EnumLiteral { .. }
        | FeatureExprKind::PrimitiveBinaryOpr { .. }
        | FeatureExprKind::StructFieldAccess { .. }
        | FeatureExprKind::PrimitiveLiteral(_) => {
            p!(this.kind);
            panic!()
        }
        FeatureExprKind::This { ref repr } => db.record_field_repr(repr.clone(), field_ident),
        FeatureExprKind::GlobalInput => todo!(),
        FeatureExprKind::RoutineCall { .. } => todo!(),
        FeatureExprKind::PatternCall {} => todo!(),
    }
}

pub(crate) fn block_record_memb(
    db: &dyn FeatureQueryGroup,
    this: &Arc<FeatureBlock>,
    field_ident: CustomIdentifier,
) -> FeatureRepr {
    let stmt_features = this.stmt_features();
    if stmt_features.len() == 1 {
        match this.stmts.last().unwrap().kind {
            FeatureStmtKind::Return { ref result } => {
                db.record_field_repr(result.clone().into(), field_ident)
            }
            FeatureStmtKind::BranchGroup { kind, ref branches } => todo!(),
            _ => panic!(),
        }
    } else {
        todo!()
    }
}
