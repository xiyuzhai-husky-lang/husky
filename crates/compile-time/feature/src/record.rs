use semantics_entity::{EntityKind, TyDefnKind};
use sync_utils::ARwLock;

use crate::*;
use std::{collections::HashMap, sync::Arc};

//     fn gen_block_memb_var(
//         &mut self,
//         db: &dyn FeatureQueryGroup,
//         block: &FeatureBlock,
//         memb_ident: CustomIdentifier,
//     ) -> FeatureRepr {
//         let stmt_features = block.stmt_features();
//         if stmt_features.len() == 1 {
//             match block.stmts.last().unwrap().kind {
//                 FeatureStmtKind::Return { ref result } => self.memb_var(db, result, memb_ident),
//                 FeatureStmtKind::BranchGroup { kind, ref branches } => todo!(),
//                 _ => panic!(),
//             }
//         } else {
//             todo!()
//         }
//     }

//     fn gen_block_memb_feature(
//         &mut self,
//         db: &dyn FeatureQueryGroup,
//         block: &FeatureBlock,
//         memb_ident: CustomIdentifier,
//     ) -> FeatureRepr {
//         let stmt_features = block.stmt_features();
//         if stmt_features.len() == 1 {
//             match block.stmts.last().unwrap().kind {
//                 FeatureStmtKind::Return { ref result } => self.memb_feature(db, result, memb_ident),
//                 FeatureStmtKind::BranchGroup { kind, ref branches } => todo!(),
//                 _ => panic!(),
//             }
//         } else {
//             todo!()
//         }
//     }
// }

pub(crate) fn record_memb_repr(
    db: &dyn FeatureQueryGroup,
    this: FeatureRepr,
    memb_ident: CustomIdentifier,
) -> FeatureRepr {
    match this {
        FeatureRepr::Expr(ref expr) => expr_record_memb(db, expr, memb_ident),
        FeatureRepr::Block(ref block) => block_record_memb(db, block, memb_ident),
    }
}

pub(crate) fn expr_record_memb(
    db: &dyn FeatureQueryGroup,
    this: &Arc<FeatureExpr>,
    memb_ident: CustomIdentifier,
) -> FeatureRepr {
    match this.kind {
        FeatureExprKind::Variable { .. } => todo!(),
        FeatureExprKind::RecordMembAccess { .. } => todo!(),
        FeatureExprKind::MembPattCall { .. } => todo!(),
        FeatureExprKind::ScopedFeature { ref block, .. } => {
            block_record_memb(db, block, memb_ident)
        }
        FeatureExprKind::ClassCall {
            ref entity,
            ref opds,
            ..
        } => match entity.kind() {
            EntityKind::Ty(ty) => match ty.kind {
                TyDefnKind::Record {
                    ref memb_vars,
                    ref memb_features,
                } => {
                    if let Some(idx) = memb_vars.position(memb_ident) {
                        opds[idx].clone().into()
                    } else if let Some(memb_feature) = memb_features.get(memb_ident) {
                        FeatureBlock::new(
                            db,
                            Some(this.clone().into()),
                            &memb_feature.stmts,
                            &[],
                            db.features(),
                        )
                        .into()
                    } else {
                        todo!()
                    }
                }
                _ => panic!(),
            },
            _ => panic!(),
        },
        FeatureExprKind::FuncCall { .. }
        | FeatureExprKind::EnumLiteral { .. }
        | FeatureExprKind::PrimitiveBinaryOpr { .. }
        | FeatureExprKind::ProcCall { .. }
        | FeatureExprKind::MembFuncCall { .. }
        | FeatureExprKind::MembProcCall { .. }
        | FeatureExprKind::StructMembVarAccess { .. }
        | FeatureExprKind::PrimitiveLiteral(_) => {
            panic!()
        }
        FeatureExprKind::This { ref repr } => db.record_memb_repr(repr.clone(), memb_ident),
    }
}

pub(crate) fn block_record_memb(
    db: &dyn FeatureQueryGroup,
    this: &Arc<FeatureBlock>,
    memb_ident: CustomIdentifier,
) -> FeatureRepr {
    let stmt_features = this.stmt_features();
    if stmt_features.len() == 1 {
        match this.stmts.last().unwrap().kind {
            FeatureStmtKind::Return { ref result } => {
                db.record_memb_repr(result.clone().into(), memb_ident)
            }
            FeatureStmtKind::BranchGroup { kind, ref branches } => todo!(),
            _ => panic!(),
        }
    } else {
        todo!()
    }
}
