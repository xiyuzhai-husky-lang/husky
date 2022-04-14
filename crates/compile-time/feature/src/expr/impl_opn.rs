use entity_syntax::TyKind;
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
                let entity_defn = self.db.entity_defn(routine.route).unwrap();
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
                field_ident,
                field_kind: field_access_kind,
            } => {
                let this = self.new_expr(&opds[0]);
                match field_access_kind {
                    FieldKind::StructOriginal => {
                        let feature = self.features.alloc(Feature::StructFieldAccess {
                            this: this.feature,
                            field_ident: field_ident.ident,
                        });
                        msg_once!("compiled memb var access");
                        let this_ty_decl = self.db.ty_decl(this.ty).unwrap();
                        (
                            FeatureExprKind::StructMembVarAccess {
                                this,
                                field_ident,
                                field_idx: this_ty_decl.field_idx(field_ident.ident),
                                contract,
                                opt_compiled: None,
                            },
                            feature,
                        )
                    }
                    FieldKind::StructDerived => todo!(),
                    FieldKind::RecordOriginal => {
                        let repr = self
                            .db
                            .record_field_repr(this.clone().into(), field_ident.ident);
                        let feature = repr.feature();
                        (
                            FeatureExprKind::RecordMembAccess {
                                this,
                                field_ident,
                                repr,
                            },
                            feature,
                        )
                    }
                    FieldKind::RecordDerived => todo!(),
                }
            }
            LazyOpnKind::MethodCall {
                method_ident,
                method_route,
                ..
            } => {
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd)).collect();
                let feature = self.features.alloc(Feature::MethodCall {
                    method_ident: method_ident.ident,
                    opds: opds.iter().map(|opd| opd.feature).collect(),
                });
                // let this_ty_defn = self.db.opt_entity_defn(opds[0].ty).unwrap().unwrap();
                // let ty_defn = match this_ty_defn.kind() {
                //     EntityDefnVariant::Ty(ty) => ty,
                //     _ => panic!(),
                // };
                let method_defn = self.db.method_defn(method_route);
                // let kind = match method.kind {
                //     MethodKind::Func { ref stmts } => FeatureExprKind::MethodCall {
                //         field_ident: field_ident.ident,
                //         instruction_sheet: self
                //             .db
                //             .method_instruction_sheet(opds[0].ty, field_ident.ident),
                //         stmts: stmts.clone(),
                //         opds,
                //         opt_compiled: None,
                //     },
                //     MethodKind::Proc { ref stmts } => todo!(),
                // };
                // (kind, feature)
                todo!()
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
                let kind = FeatureExprKind::NewRecord {
                    ty,
                    entity: self.db.entity_defn(ty.route).unwrap(),
                    opds,
                };
                (kind, feature)
            }
        }
    }

    fn record_field_var_value(
        &self,
        this: &FeatureExpr,
        field_ident: CustomIdentifier,
    ) -> Arc<FeatureExpr> {
        match this.kind {
            FeatureExprKind::Variable { .. } => todo!(),
            FeatureExprKind::RecordMembAccess { .. } => todo!(),
            FeatureExprKind::MembPattCall { .. } => todo!(),
            FeatureExprKind::FeatureBlock { ref block, .. } => {
                self.derive_record_field_var_value_from_block(block, field_ident)
            }
            FeatureExprKind::NewRecord {
                ref entity,
                ref opds,
                ..
            } => match entity.kind() {
                EntityDefnVariant::Ty(ty) => match ty.kind {
                    TyKind::Record => {
                        todo!()
                        // p!(field_ident, ty.fields);
                        // let idx = ty.fields.position(field_ident).unwrap();
                        // opds[idx].clone()
                    }
                    _ => panic!(),
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
            FeatureExprKind::This { ref repr } => todo!(),
            FeatureExprKind::GlobalInput => todo!(),
        }
    }

    // RecordMembExpr {
    //     feature: result.feature,
    //     kind: RecordMembExprKind::Expr(result.clone()),
    // },
    fn derive_record_field_var_value_from_block(
        &self,
        block: &FeatureBlock,
        field_ident: CustomIdentifier,
    ) -> Arc<FeatureExpr> {
        let stmt_features = block.stmt_features();
        if stmt_features.len() == 1 {
            match block.stmts.last().unwrap().kind {
                FeatureStmtKind::Return { ref result } => {
                    self.record_field_var_value(result, field_ident)
                }
                FeatureStmtKind::BranchGroup { kind, ref branches } => todo!(),
                _ => panic!(),
            }
        } else {
            todo!()
        }
    }
}
