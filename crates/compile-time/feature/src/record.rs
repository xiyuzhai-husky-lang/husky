use entity_syntax::TyKind;
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
        FeatureExprKind::RecordMembAccess { .. } => todo!(),
        FeatureExprKind::MembPattCall { .. } => todo!(),
        FeatureExprKind::FeatureBlock { ref block, .. } => {
            block_record_memb(db, block, field_ident)
        }
        FeatureExprKind::ClassCall {
            ref entity,
            ref opds,
            ..
        } => match entity.kind() {
            EntityDefnVariant::Ty(ty_defn) => match ty_defn.kind {
                TyKind::Record => {
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
                TyKind::Enum => todo!(),
                TyKind::Struct => todo!(),
                TyKind::Primitive => todo!(),
                TyKind::Vec => todo!(),
                TyKind::Array => todo!(),
                TyKind::Other => todo!(),
            },
            _ => panic!(),
        },
        FeatureExprKind::FuncCall { .. }
        | FeatureExprKind::EnumLiteral { .. }
        | FeatureExprKind::PrimitiveBinaryOpr { .. }
        | FeatureExprKind::ProcCall { .. }
        | FeatureExprKind::MethodCall { .. }
        | FeatureExprKind::MembProcCall { .. }
        | FeatureExprKind::StructMembVarAccess { .. }
        | FeatureExprKind::PrimitiveLiteral(_) => {
            panic!()
        }
        FeatureExprKind::This { ref repr } => db.record_field_repr(repr.clone(), field_ident),
        FeatureExprKind::GlobalInput => todo!(),
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
