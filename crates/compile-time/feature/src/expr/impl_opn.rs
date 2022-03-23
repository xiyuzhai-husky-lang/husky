use semantics_error::*;
use vm::{Compiled, LazyContract};

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
                ScopePtr::Builtin(BuiltinIdentifier::Void)
                | ScopePtr::Builtin(BuiltinIdentifier::I32)
                | ScopePtr::Builtin(BuiltinIdentifier::F32)
                | ScopePtr::Builtin(BuiltinIdentifier::B32)
                | ScopePtr::Builtin(BuiltinIdentifier::B64) => {
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
                let uid = self.db.entity_vc().uid(routine.scope);
                let inputs: Vec<_> = opds.iter().map(|opd| self.new_expr(opd)).collect();
                let feature = self.features.alloc(Feature::FuncCall {
                    func: routine.scope,
                    uid,
                    inputs: inputs.iter().map(|expr| expr.feature).collect(),
                });
                let entity = self.db.entity(routine.scope).unwrap();
                let kind = match entity.kind() {
                    EntityKind::Func {
                        input_placeholders,
                        stmts,
                        ..
                    } => FeatureExprKind::FuncCall {
                        func_ranged_scope: routine,
                        uid,
                        compiled: None,
                        callee_file: entity.file,
                        input_placeholders: input_placeholders.clone(),
                        inputs,
                        instruction_sheet: self.db.scope_instruction_sheet(routine.scope).unwrap(),
                        stmts: stmts.clone(),
                    },
                    EntityKind::Proc {
                        input_placeholders,
                        stmts,
                        ..
                    } => FeatureExprKind::ProcCall {
                        proc_ranged_scope: routine,
                        uid,
                        compiled: None,
                        callee_file: entity.file,
                        input_placeholders: input_placeholders.clone(),
                        inputs,
                        instruction_sheet: self.db.scope_instruction_sheet(routine.scope).unwrap(),
                        stmts: stmts.clone(),
                    },
                    _ => panic!(),
                };
                (kind, feature)
            }
            LazyOpnKind::PattCall => todo!(),
            LazyOpnKind::MembVarAccess(memb_var_ident) => {
                let this = self.new_expr(&opds[0]);
                let feature = self.features.alloc(Feature::MembVarAccess {
                    this: this.feature,
                    memb_ident: memb_var_ident,
                });
                msg_once!("compiled memb var access");
                (
                    FeatureExprKind::MembVarAccess {
                        this,
                        memb_var_ident,
                        contract,
                        opt_compiled: None,
                    },
                    feature,
                )
            }
            LazyOpnKind::MembCall { memb_ident, .. } => {
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd)).collect();
                let feature = self.features.alloc(Feature::MembCall {
                    memb_ident,
                    opds: opds.iter().map(|opd| opd.feature).collect(),
                });
                let ty_entity = self.db.entity(opds[0].ty).unwrap();
                let ty = match ty_entity.kind() {
                    EntityKind::Ty(ty) => ty,
                    _ => panic!(),
                };
                match ty.kind {
                    TyKind::Enum { ref variants } => todo!(),
                    TyKind::Struct {
                        ref memb_vars,
                        ref memb_routines,
                    } => {
                        let memb_routine = memb_routines.get(memb_ident).unwrap();
                        let kind = match memb_routine.kind {
                            MembRoutineKind::Func { ref stmts } => FeatureExprKind::MembFuncCall {
                                memb_ident,
                                instruction_sheet: self
                                    .db
                                    .memb_routine_instruction_sheet(opds[0].ty, memb_ident)
                                    .unwrap(),
                                stmts: stmts.clone(),
                                opds,
                                compiled: None,
                            },
                            MembRoutineKind::Proc { ref stmts } => todo!(),
                        };
                        (kind, feature)
                    }
                }
            }
            LazyOpnKind::ElementAccess => todo!(),
            LazyOpnKind::TypeCall(_) => todo!(),
        }
    }
}
