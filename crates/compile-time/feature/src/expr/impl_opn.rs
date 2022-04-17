use entity_kind::TypeKind;
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
                    let lopd = self.new_expr(opds[0].clone());
                    let ropd = self.new_expr(opds[1].clone());
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
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
                let feature = self.features.alloc(Feature::FuncCall {
                    func: routine.route,
                    uid,
                    inputs: opds.iter().map(|expr| expr.feature).collect(),
                });
                let routine_defn = self.db.entity_defn(routine.route).unwrap();
                let routine_uid = self.db.entity_uid(routine.route);
                let kind = FeatureExprKind::RoutineCall {
                    opt_linkage: self.db.linkage_table().routine(routine_uid),
                    opds,
                    instruction_sheet: self.db.entity_instruction_sheet(routine.route),
                    routine_defn,
                };
                (kind, feature)
            }
            LazyOpnKind::PatternCall => todo!(),
            LazyOpnKind::FieldAccess {
                field_ident,
                field_kind: field_access_kind,
            } => {
                let this = self.new_expr(opds[0].clone());
                match field_access_kind {
                    FieldKind::StructOriginal => {
                        let feature = self.features.alloc(Feature::StructFieldAccess {
                            this: this.feature,
                            field_ident: field_ident.ident,
                        });
                        msg_once!("compiled memb var access");
                        let this_ty_decl = self.db.type_decl(this.expr.ty).unwrap();
                        let this_ty_uid = self.db.entity_uid(this.expr.ty);
                        (
                            FeatureExprKind::StructFieldAccess {
                                this,
                                field_ident,
                                field_idx: this_ty_decl.field_idx(field_ident.ident),
                                contract,
                                opt_linkage: self
                                    .db
                                    .linkage_table()
                                    .struct_field_access(this_ty_uid, field_ident.ident),
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
                            FeatureExprKind::RecordFieldAccess {
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
                member_idx,
                ..
            } => {
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
                let feature = self.features.alloc(Feature::MethodCall {
                    method_ident: method_ident.ident,
                    opds: opds.iter().map(|opd| opd.feature).collect(),
                });
                let this_ty_defn = self.db.entity_defn(opds[0].expr.ty).unwrap();
                // let ty_defn = match this_ty_defn.kind() {
                //     EntityDefnVariant::Ty(ty) => ty,
                //     _ => panic!(),
                // };
                let method_defn = this_ty_defn.method(member_idx);
                let method_uid = self.db.entity_uid(method_route);
                let kind = match method_defn.variant {
                    EntityDefnVariant::Main(_) => todo!(),
                    EntityDefnVariant::Module {} => todo!(),
                    EntityDefnVariant::Feature { .. } => todo!(),
                    EntityDefnVariant::Pattern {} => todo!(),
                    EntityDefnVariant::EnumVariant { .. } => todo!(),
                    EntityDefnVariant::Builtin => todo!(),
                    EntityDefnVariant::TypeMethod {
                        ref method_variant, ..
                    } => match method_variant {
                        MethodDefnVariant::Func { .. } | MethodDefnVariant::Proc { .. } => {
                            FeatureExprKind::RoutineCall {
                                opds,
                                instruction_sheet: self.db.method_instruction_sheet(method_route),
                                opt_linkage: self.db.linkage_table().routine(method_uid),
                                routine_defn: method_defn.clone(),
                            }
                        }
                        MethodDefnVariant::Pattern { stmts } => todo!(),
                    },
                    _ => panic!(),
                };
                //  FeatureExprKind::MethodCall {
                //     field_ident: method_ident,
                //     // instruction_sheet: self.db.method_instruction_sheet(opds[0].ty, member_ident),
                //     // stmts: stmts.clone(),
                //     opds,
                //     // opt_compiled: None,
                // };
                (kind, feature)
            }
            LazyOpnKind::ElementAccess => todo!(),
            LazyOpnKind::StructCall(_) => todo!(),
            LazyOpnKind::RecordCall(ty) => {
                let uid = self.db.entity_uid(ty.route);
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
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
            FeatureExprKind::RecordFieldAccess { .. } => todo!(),
            FeatureExprKind::FeatureBlock { ref block, .. } => {
                self.derive_record_field_var_value_from_block(block, field_ident)
            }
            FeatureExprKind::NewRecord {
                ref entity,
                ref opds,
                ..
            } => match entity.kind() {
                EntityDefnVariant::Type { kind, .. } => match kind {
                    TypeKind::Record => {
                        todo!()
                        // p!(field_ident, ty.fields);
                        // let idx = ty.fields.position(field_ident).unwrap();
                        // opds[idx].clone()
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            FeatureExprKind::RoutineCall { .. }
            | FeatureExprKind::EnumLiteral { .. }
            | FeatureExprKind::PrimitiveBinaryOpr { .. }
            | FeatureExprKind::StructFieldAccess { .. }
            | FeatureExprKind::PrimitiveLiteral(_) => {
                panic!()
            }
            FeatureExprKind::This { ref repr } => todo!(),
            FeatureExprKind::GlobalInput => todo!(),
            FeatureExprKind::PatternCall {} => todo!(),
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
