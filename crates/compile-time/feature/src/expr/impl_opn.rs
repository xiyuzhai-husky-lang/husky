use decl::{MembAccessKind, TyDecl};
use vm::LazyContract;

use super::*;

impl<'a> FeatureExprBuilder<'a> {
    pub(super) fn new_opn(
        &self,
        opn_kind: LazyOpnKind,
        compiled: (),
        opds: &[Arc<LazyExpr>],
        contract: LazyContract,
    ) -> (FeatureExprKind, FeaturePtr) {
        match opn_kind {
            LazyOpnKind::Binary { opr, this } => match this {
                EntityRoutePtr::Root(RootIdentifier::Void)
                | EntityRoutePtr::Root(RootIdentifier::I32)
                | EntityRoutePtr::Root(RootIdentifier::F32)
                | EntityRoutePtr::Root(RootIdentifier::B32)
                | EntityRoutePtr::Root(RootIdentifier::B64) => {
                    let lopd = self.new_expr(&opds[0]);
                    let ropd = self.new_expr(&opds[1]);
                    let feature = self.features.alloc(Feature::PrimitiveBinaryOpr {
                        opr,
                        lopd: lopd.feature,
                        ropd: ropd.feature,
                    });
                    (
                        FeatureExprKind::PrimitiveBinaryOpr { opr, lopd, ropd },
                        feature,
                    )
                }
                _ => todo!(),
            },
            LazyOpnKind::Prefix(_) => todo!(),
            LazyOpnKind::RoutineCall(routine) => {
                let uid = self.db.entity_uid(routine.route);
                let inputs: Vec<_> = opds.iter().map(|opd| self.new_expr(opd)).collect();
                let feature = self.features.alloc(Feature::FuncCall {
                    func: routine.route,
                    uid,
                    inputs: inputs.iter().map(|expr| expr.feature).collect(),
                });
                let entity_defn = self.db.opt_entity_defn(routine.route).unwrap().unwrap();
                let kind = match entity_defn.kind() {
                    EntityDefnVariant::Func {
                        input_placeholders,
                        stmts,
                        ..
                    } => FeatureExprKind::FuncCall {
                        func_ranged_scope: routine,
                        uid,
                        compiled: None,
                        callee_file: entity_defn.file,
                        input_placeholders: input_placeholders.clone(),
                        inputs,
                        instruction_sheet: self.db.entity_instruction_sheet(routine.route),
                        stmts: stmts.clone(),
                    },
                    EntityDefnVariant::Proc {
                        input_placeholders,
                        stmts,
                        ..
                    } => FeatureExprKind::ProcCall {
                        proc_ranged_scope: routine,
                        uid,
                        opt_compiled: None,
                        callee_file: entity_defn.file,
                        input_placeholders: input_placeholders.clone(),
                        inputs,
                        instruction_sheet: self.db.entity_instruction_sheet(routine.route),
                        stmts: stmts.clone(),
                    },
                    _ => panic!(),
                };
                (kind, feature)
            }
            LazyOpnKind::PatternCall => todo!(),
            LazyOpnKind::MembAccess {
                memb_ident,
                memb_access_kind,
            } => {
                let this = self.new_expr(&opds[0]);
                match memb_access_kind {
                    MembAccessKind::StructMembVar => {
                        let feature = self.features.alloc(Feature::StructMembVarAccess {
                            this: this.feature,
                            memb_ident,
                        });
                        msg_once!("compiled memb var access");
                        let this_ty_decl = self.db.ty_decl(this.ty).unwrap();
                        (
                            FeatureExprKind::StructMembVarAccess {
                                this,
                                memb_ident,
                                memb_idx: this_ty_decl.memb_idx(memb_ident),
                                contract,
                                opt_compiled: None,
                            },
                            feature,
                        )
                    }
                    MembAccessKind::StructMembFeature => todo!(),
                    MembAccessKind::RecordMemb => {
                        let repr = self.db.record_memb_repr(this.clone().into(), memb_ident);
                        let feature = repr.feature();
                        (
                            FeatureExprKind::RecordMembAccess {
                                this,
                                memb_ident,
                                repr,
                            },
                            feature,
                        )
                    }
                }
            }
            LazyOpnKind::MembCall { memb_ident, .. } => {
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd)).collect();
                let feature = self.features.alloc(Feature::MembCall {
                    memb_ident,
                    opds: opds.iter().map(|opd| opd.feature).collect(),
                });
                let ty_entity_defn = self.db.opt_entity_defn(opds[0].ty).unwrap().unwrap();
                let ty = match ty_entity_defn.kind() {
                    EntityDefnVariant::Ty(ty) => ty,
                    _ => panic!(),
                };
                match ty.kind {
                    TyDefnVariant::Enum { ref variants } => todo!(),
                    TyDefnVariant::Struct {
                        ref memb_vars,
                        ref memb_routines,
                    } => {
                        let memb_routine = memb_routines.get(memb_ident).unwrap();
                        let kind = match memb_routine.kind {
                            MembRoutineKind::Func { ref stmts } => FeatureExprKind::MembFuncCall {
                                memb_ident,
                                instruction_sheet: self
                                    .db
                                    .memb_routine_instruction_sheet(opds[0].ty, memb_ident),
                                stmts: stmts.clone(),
                                opds,
                                opt_compiled: None,
                            },
                            MembRoutineKind::Proc { ref stmts } => todo!(),
                        };
                        (kind, feature)
                    }
                    TyDefnVariant::Record { .. } => todo!(),
                }
            }
            LazyOpnKind::ElementAccess => todo!(),
            LazyOpnKind::StructCall(_) => todo!(),
            LazyOpnKind::ClassCall(ty) => {
                let uid = self.db.entity_uid(ty.route);
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd)).collect();
                let feature = self.features.alloc(Feature::ClassCall {
                    ty: ty.route,
                    uid,
                    opds: opds.iter().map(|opd| opd.feature).collect(),
                });
                let kind = FeatureExprKind::ClassCall {
                    ty,
                    entity: self.db.opt_entity_defn(ty.route).unwrap().unwrap(),
                    opds,
                };
                (kind, feature)
            }
        }
    }

    fn record_memb_var_value(
        &self,
        this: &FeatureExpr,
        memb_ident: CustomIdentifier,
    ) -> Arc<FeatureExpr> {
        match this.kind {
            FeatureExprKind::Variable { .. } => todo!(),
            FeatureExprKind::RecordMembAccess { .. } => todo!(),
            FeatureExprKind::MembPattCall { .. } => todo!(),
            FeatureExprKind::FeatureBlock { ref block, .. } => {
                self.derive_record_memb_var_value_from_block(block, memb_ident)
            }
            FeatureExprKind::ClassCall {
                ref entity,
                ref opds,
                ..
            } => match entity.kind() {
                EntityDefnVariant::Ty(ty) => match ty.kind {
                    TyDefnVariant::Record { ref memb_vars, .. } => {
                        p!(memb_ident, memb_vars);
                        let idx = memb_vars.position(memb_ident).unwrap();
                        opds[idx].clone()
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
            FeatureExprKind::This { ref repr } => todo!(),
            FeatureExprKind::GlobalInput => todo!(),
        }
    }

    // RecordMembExpr {
    //     feature: result.feature,
    //     kind: RecordMembExprKind::Expr(result.clone()),
    // },
    fn derive_record_memb_var_value_from_block(
        &self,
        block: &FeatureBlock,
        memb_ident: CustomIdentifier,
    ) -> Arc<FeatureExpr> {
        let stmt_features = block.stmt_features();
        if stmt_features.len() == 1 {
            match block.stmts.last().unwrap().kind {
                FeatureStmtKind::Return { ref result } => {
                    self.record_memb_var_value(result, memb_ident)
                }
                FeatureStmtKind::BranchGroup { kind, ref branches } => todo!(),
                _ => panic!(),
            }
        } else {
            todo!()
        }
    }
}
