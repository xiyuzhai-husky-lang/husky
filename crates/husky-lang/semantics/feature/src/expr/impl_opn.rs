use vm::Compiled;

use super::*;

impl<'a> FeatureExprBuilder<'a> {
    pub(super) fn new_opn(
        &self,
        opn_kind: LazyOpnKind,
        compiled: Option<Compiled>,
        opds: &[Arc<LazyExpr>],
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
                        ranged_scope: routine,
                        uid,
                        compiled: None,
                        callee_file: entity.file,
                        input_placeholders: input_placeholders.clone(),
                        inputs,
                        instruction_sheet: self.db.instruction_sheet(routine.scope).unwrap(),
                        stmts: stmts.clone(),
                    },
                    EntityKind::Proc {
                        input_placeholders,
                        stmts,
                        ..
                    } => FeatureExprKind::ProcCall {
                        ranged_scope: routine,
                        uid,
                        compiled: None,
                        callee_file: entity.file,
                        input_placeholders: input_placeholders.clone(),
                        inputs,
                        instruction_sheet: self.db.instruction_sheet(routine.scope).unwrap(),
                        stmts: stmts.clone(),
                    },
                    _ => panic!(),
                };
                (kind, feature)
            }
            LazyOpnKind::PatternCall => todo!(),
            LazyOpnKind::MembVarAccess(memb_var_ident) => {
                let this = self.new_expr(&opds[0]);
                let feature = self.features.alloc(Feature::MembVarAccess {
                    this: this.feature,
                    memb_var_ident,
                });
                msg_once!("compiled memb var access");
                (
                    FeatureExprKind::MembVarAccess {
                        this,
                        memb_var_ident,
                        opt_compiled: None,
                    },
                    feature,
                )
            }
            LazyOpnKind::MembFuncCall(_) => todo!(),
            LazyOpnKind::ElementAccess => todo!(),
            LazyOpnKind::TypeCall(_) => todo!(),
        }
    }
}
