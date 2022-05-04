use entity_kind::{FieldKind, TyKind};
use entity_route::EntityRoute;
use linkage_table::MemberAccessKind;
use map_collect::MapCollect;
use static_defn::LinkageSource;
use vm::LazyContract;

use super::*;

impl<'a> FeatureExprBuilder<'a> {
    pub(super) fn compile_opn(
        &self,
        opn_kind: LazyOpnKind,
        opds: &[Arc<LazyExpr>],
        expr: &Arc<LazyExpr>,
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
                let kind = FeatureExprKind::RoutineCall {
                    opt_linkage: self.db.routine_linkage(routine.route),
                    opds,
                    instruction_sheet: self.db.entity_instruction_sheet(routine.route),
                    routine_defn,
                };
                (kind, feature)
            }
            LazyOpnKind::PatternCall => todo!(),
            LazyOpnKind::FieldAccess {
                field_ident,
                field_kind,
            } => self.compile_field_access(field_ident, field_kind, opds, expr.contract),
            LazyOpnKind::MethodCall {
                method_ident,
                method_route,
                ..
            } => self.compile_method_call(method_ident, method_route, opds),
            LazyOpnKind::ElementAccess => self.compile_element_access(opds, expr),
            LazyOpnKind::StructCall(_) => todo!(),
            LazyOpnKind::RecordCall(ty) => {
                let uid = self.db.entity_uid(ty.route);
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
                let feature = self.features.alloc(Feature::RecordTypeCall {
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

    fn compile_method_call(
        &self,
        method_ident: RangedCustomIdentifier,
        method_route: EntityRoutePtr,
        opds: &[Arc<LazyExpr>],
    ) -> (FeatureExprKind, FeaturePtr) {
        let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
        let feature = self.features.alloc(Feature::MethodCall {
            method_ident: method_ident.ident,
            opds: opds.iter().map(|opd| opd.feature).collect(),
        });
        let this_ty_defn = self.db.entity_defn(opds[0].expr.ty).unwrap();
        let member_idx = self.db.member_idx(method_route);
        let method_defn = this_ty_defn.method(member_idx);
        let kind = match method_defn.variant {
            EntityDefnVariant::Method {
                ref method_variant, ..
            } => {
                let source = match method_variant {
                    MethodDefnVariant::TypeMethod { method_source, .. } => method_source,
                    MethodDefnVariant::TraitMethod {
                        trai,
                        opt_default_source,
                    } => todo!(),
                    MethodDefnVariant::TraitMethodImpl { trai, opt_source } => todo!(),
                };
                let opt_instruction_sheet = match source {
                    MethodSource::Func { .. } | MethodSource::Proc { .. } => {
                        self.db.method_instruction_sheet(method_route)
                    }
                    MethodSource::Pattern { stmts } => todo!(),
                    MethodSource::Static(_) => todo!(),
                };
                FeatureExprKind::RoutineCall {
                    instruction_sheet: self.db.method_instruction_sheet(method_route),
                    opt_linkage: self.db.method_linkage_source(method_route).as_ref().map(
                        |linkage_source| match linkage_source {
                            LinkageSource::MemberAccess {
                                ref_access,
                                move_access,
                                ..
                            } => match opds[0].expr.contract {
                                LazyContract::Move => *move_access,
                                LazyContract::GlobalRef => *ref_access,
                                LazyContract::Pure => *ref_access,
                            },
                            LinkageSource::PureOutput(_) => todo!(),
                        },
                    ),
                    opds,
                    routine_defn: method_defn.clone(),
                }
                //     match method_variant {
                //     MethodSource::Func { .. } | MethodSource::Proc { .. } => {
                //         FeatureExprKind::RoutineCall {
                //             opds,
                //             instruction_sheet: self.db.method_instruction_sheet(method_route),
                //             opt_linkage: self.db.method_linkage(method_route),
                //             routine_defn: method_defn.clone(),
                //         }
                //     }
                //     MethodSource::Pattern { stmts } => todo!(),
                //     MethodSource::StaticMemberAccess {
                //         ref_access,
                //         move_access,
                //         borrow_mut_access,
                //     } => todo!(),
                // }
            }
            _ => panic!(),
        };
        (kind, feature)
    }

    fn compile_field_access(
        &self,
        field_ident: RangedCustomIdentifier,
        field_access_kind: FieldKind,
        opds: &[Arc<LazyExpr>],
        contract: LazyContract,
    ) -> (FeatureExprKind, FeaturePtr) {
        let this = self.new_expr(opds[0].clone());
        let this_ty_decl = self.db.ty_decl(this.expr.ty).unwrap();
        match field_access_kind {
            FieldKind::StructOriginal => {
                let feature = self.features.alloc(Feature::StructOriginalFieldAccess {
                    this: this.feature,
                    field_ident: field_ident.ident,
                });
                (
                    FeatureExprKind::StructOriginalFieldAccess {
                        field_ident,
                        field_idx: this_ty_decl.field_idx(field_ident.ident),
                        contract,
                        opt_linkage: self
                            .db
                            .struct_field_access(this.expr.ty, field_ident.ident)
                            .map(|linkage_source| match linkage_source {
                                LinkageSource::MemberAccess {
                                    ref_access,
                                    move_access,
                                    borrow_mut_access,
                                    ..
                                } => todo!(),
                                LinkageSource::PureOutput(_) => todo!(),
                            }),
                        this,
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
                    FeatureExprKind::RecordOriginalFieldAccess {
                        this,
                        field_ident,
                        repr,
                    },
                    feature,
                )
            }
            FieldKind::RecordDerived => {
                let this_ty_defn = self.db.entity_defn(this.expr.ty).unwrap();
                let field_uid =
                    self.db
                        .entity_uid(self.db.intern_entity_route(EntityRoute::child_route(
                            this.expr.ty,
                            field_ident.ident,
                            vec![],
                        )));
                match this_ty_defn.variant {
                    EntityDefnVariant::Type {
                        ty_members: ref type_members,
                        ..
                    } => match type_members.get(field_ident.ident).unwrap().variant {
                        EntityDefnVariant::TypeField {
                            ref field_variant, ..
                        } => match field_variant {
                            FieldDefnVariant::StructOriginal | FieldDefnVariant::RecordOriginal => {
                                panic!()
                            }
                            FieldDefnVariant::StructDerived { stmts } => todo!(),
                            FieldDefnVariant::RecordDerived { stmts } => {
                                let block = FeatureBlock::new(
                                    self.db,
                                    Some(this.clone().into()),
                                    stmts,
                                    &[],
                                    self.db.features(),
                                );
                                let feature =
                                    self.db.features().alloc(Feature::RecordDerivedFieldAccess {
                                        this: this.feature,
                                        field_uid,
                                    });
                                let feature_expr_kind = FeatureExprKind::RecordDerivedFieldAccess {
                                    this,
                                    field_ident,
                                    block,
                                };
                                (feature_expr_kind, feature)
                            }
                        },
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
        }
    }

    fn compile_element_access(
        &self,
        opds: &[Arc<LazyExpr>],
        expr: &Arc<LazyExpr>,
    ) -> (FeatureExprKind, FeaturePtr) {
        let opds: Vec<_> = opds.map(|opd| self.new_expr(opd.clone()));
        let feature = self.features.alloc(Feature::ElementAccess {
            opds: opds.map(|opd| opd.feature),
        });
        let feature_expr_kind = FeatureExprKind::ElementAccess {
            linkage: self.db.element_access_linkage(
                opds.map(|opd| opd.expr.ty),
                match opds[0].expr.contract {
                    LazyContract::Move => todo!(),
                    LazyContract::GlobalRef => todo!(),
                    LazyContract::Pure => {
                        if self.db.is_copyable(expr.ty) {
                            MemberAccessKind::Copy
                        } else {
                            MemberAccessKind::Ref
                        }
                    }
                },
            ),
            opds,
        };
        (feature_expr_kind, feature)
    }

    fn record_field_var_value(
        &self,
        this: &FeatureExpr,
        field_ident: CustomIdentifier,
    ) -> Arc<FeatureExpr> {
        match this.kind {
            FeatureExprKind::Variable { .. } => todo!(),
            FeatureExprKind::RecordOriginalFieldAccess { .. } => todo!(),
            FeatureExprKind::EntityFeature { ref block, .. } => {
                self.derive_record_field_var_value_from_block(block, field_ident)
            }
            FeatureExprKind::NewRecord {
                ref entity,
                ref opds,
                ..
            } => match entity.variant {
                EntityDefnVariant::Type { kind, .. } => match kind {
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
            FeatureExprKind::RoutineCall { .. }
            | FeatureExprKind::EnumLiteral { .. }
            | FeatureExprKind::PrimitiveBinaryOpr { .. }
            | FeatureExprKind::StructOriginalFieldAccess { .. }
            | FeatureExprKind::PrimitiveLiteral(_) => {
                panic!()
            }
            FeatureExprKind::This { ref repr } => todo!(),
            FeatureExprKind::GlobalInput => todo!(),
            FeatureExprKind::PatternCall {} => todo!(),
            FeatureExprKind::RecordDerivedFieldAccess { .. } => todo!(),
            FeatureExprKind::ElementAccess { ref opds, .. } => todo!(),
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
