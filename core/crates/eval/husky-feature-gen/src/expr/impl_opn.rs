use super::*;
use entity_kind::{FieldKind, TyKind};
use husky_ast::FieldAstKind;
use husky_entity_route::{EntityRoute, InternEntityRoute};
use husky_entity_semantics::EntityDefnVariant;
use husky_linkage_table::ResolveLinkage;
use map_collect::MapCollect;
use thin_vec::{thin_vec, ThinVec};
use vm::__Linkage;
use vm::{Binding, ModelLinkage, __EvalResult};

impl<'a> FeatureExprBuilder<'a> {
    pub(super) fn compile_opn(
        &self,
        opn_kind: LazyOpnKind,
        opds: &[Arc<LazyExpr>],
        expr: &Arc<LazyExpr>,
    ) -> (FeatureLazyExprVariant, FeaturePtr) {
        match opn_kind {
            LazyOpnKind::Binary { opr, this } => match this {
                EntityRoutePtr::Root(RootIdentifier::Void)
                | EntityRoutePtr::Root(RootIdentifier::I32)
                | EntityRoutePtr::Root(RootIdentifier::F32)
                | EntityRoutePtr::Root(RootIdentifier::B32)
                | EntityRoutePtr::Root(RootIdentifier::B64) => {
                    let lopd = self.new_expr(opds[0].clone());
                    let ropd = self.new_expr(opds[1].clone());
                    let feature = self.features.intern(Feature::PrimitiveBinaryOpr {
                        opr,
                        lopd: lopd.feature,
                        ropd: ropd.feature,
                    });
                    (
                        FeatureLazyExprVariant::PrimitiveBinaryOpr { opr, lopd, ropd },
                        feature,
                    )
                }
                _ => todo!(),
            },
            LazyOpnKind::Prefix(_) => todo!(),
            LazyOpnKind::FunctionModelCall(routine) => {
                let uid = self.db.compile_time().entity_uid(routine.route);
                let opds = opds
                    .iter()
                    .map(|opd| self.new_expr(opd.clone()))
                    .collect::<Vec<_>>();
                let feature = self.features.intern(Feature::FunctionCall {
                    func: routine.route,
                    uid,
                    inputs: opds.iter().map(|expr| expr.feature).collect(),
                });
                let model_defn = self.db.compile_time().entity_defn(routine.route).unwrap();
                let internal = match model_defn.variant {
                    EntityDefnVariant::Function {
                        source: CallFormSource::Static(__Linkage::Model(model)),
                        ..
                    } => self.db.train(model, self.opt_arrival_indicator, &opds),
                    // model.train(
                    //     self.opt_arrival_indicator.map(|r| r as &dyn std::any::Any),
                    //     &opds,
                    // ),
                    _ => todo!(),
                };
                let kind = FeatureLazyExprVariant::ModelCall {
                    opds,
                    has_this: false,
                    model_defn,
                    internal,
                    opt_arrival_indicator: self
                        .opt_arrival_indicator
                        .map(|branch_indicator| branch_indicator.clone()),
                };
                (kind, feature)
            }
            LazyOpnKind::FunctionRoutineCall(routine) => {
                let uid = self.db.compile_time().entity_uid(routine.route);
                let opds = opds
                    .iter()
                    .map(|opd| self.new_expr(opd.clone()))
                    .collect::<Vec<_>>();
                let feature = self.features.intern(Feature::FunctionCall {
                    func: routine.route,
                    uid,
                    inputs: opds.iter().map(|expr| expr.feature).collect(),
                });
                let routine_defn = self.db.compile_time().entity_defn(routine.route).unwrap();
                let opt_linkage = self.db.compile_time().routine_linkage(routine.route);
                let kind = FeatureLazyExprVariant::RoutineCall {
                    opt_linkage,
                    opds,
                    has_this: false,
                    opt_instruction_sheet: self.db.entity_instruction_sheet(routine.route),
                    routine_defn,
                };
                (kind, feature)
            }
            LazyOpnKind::FieldAccess {
                field_ident,
                field_binding,
            } => self.compile_field_access(
                field_ident,
                FeatureExpr::new(
                    self.db,
                    self.opt_this.clone(),
                    opds[0].clone(),
                    self.symbols,
                    self.opt_arrival_indicator,
                    self.features,
                )
                .into(),
                field_binding,
            ),
            LazyOpnKind::MethodCall {
                method_ident,
                method_route,
                output_binding,
            } => self.compile_method_call(method_ident, method_route, opds, output_binding),
            LazyOpnKind::ElementAccess { element_binding } => {
                self.compile_element_access(opds, expr, element_binding)
            }
            LazyOpnKind::StructCall(_) => todo!(),
            LazyOpnKind::RecordCall(ty) => {
                let uid = self.db.compile_time().entity_uid(ty.route);
                let opds = opds
                    .iter()
                    .map(|opd| self.new_expr(opd.clone()))
                    .collect::<Vec<_>>();
                let feature = self.features.intern(Feature::RecordTypeCall {
                    ty: ty.route,
                    uid,
                    opds: opds.iter().map(|opd| opd.feature).collect(),
                });
                let kind = FeatureLazyExprVariant::NewRecord {
                    ty,
                    entity: self.db.compile_time().entity_defn(ty.route).unwrap(),
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
        output_binding: Binding,
    ) -> (FeatureLazyExprVariant, FeaturePtr) {
        let opds = opds
            .iter()
            .map(|opd| self.new_expr(opd.clone()))
            .collect::<Vec<_>>();
        let feature = self.features.intern(Feature::MethodCall {
            method_ident: method_ident.ident,
            opds: opds.iter().map(|opd| opd.feature).collect(),
        });
        let this_expr = &opds[0].expr;
        let this_ty_defn = self.db.compile_time().entity_defn(this_expr.ty()).unwrap();
        let member_idx = self.db.compile_time().member_idx(method_route);
        let method_defn = this_ty_defn.method(member_idx);
        let kind = match method_defn.variant {
            EntityDefnVariant::Method { .. } => FeatureLazyExprVariant::RoutineCall {
                opt_instruction_sheet: self.db.method_opt_instruction_sheet(method_route),
                opt_linkage: self.db.compile_time().method_linkage(method_route),
                opds,
                has_this: true,
                routine_defn: method_defn.clone(),
            },
            _ => panic!(),
        };
        (kind, feature)
    }

    pub(super) fn compile_field_access(
        &self,
        field_ident: RangedCustomIdentifier,
        this: FeatureRepr,
        field_binding: Binding,
    ) -> (FeatureLazyExprVariant, FeaturePtr) {
        let this_ty = this.ty();
        let this_ty_decl = self.db.compile_time().ty_decl(this_ty).unwrap();
        let field_kind = this_ty_decl.field_kind(field_ident.ident);
        match field_kind {
            FieldKind::StructOriginal
            | FieldKind::StructDefault
            | FieldKind::StructDerivedEager => {
                let feature = self.features.intern(Feature::FieldAccess {
                    this: this.feature(),
                    field_ident: field_ident.ident,
                });
                (
                    FeatureLazyExprVariant::StructOriginalFieldAccess {
                        field_ident,
                        field_idx: this_ty_decl.field_idx(field_ident.ident),
                        field_binding,
                        opt_linkage: self.db.compile_time().struct_field_access_linkage(
                            this_ty,
                            field_ident.ident,
                            field_binding,
                        ),
                        this,
                    },
                    feature,
                )
            }
            FieldKind::StructDerivedLazy { .. } => {
                let this_ty = this.ty();
                let this_ty_defn = self.db.compile_time().entity_defn(this_ty).unwrap();
                let field_uid =
                    self.db
                        .compile_time()
                        .entity_uid(self.db.compile_time().intern_entity_route(
                            EntityRoute::subroute(this_ty, field_ident.ident, thin_vec![]),
                        ));
                match this_ty_defn.variant {
                    EntityDefnVariant::Ty { ref ty_members, .. } => {
                        match ty_members.get_entry(field_ident.ident).unwrap().variant {
                            EntityDefnVariant::TyField {
                                ref field_variant, ..
                            } => match field_variant {
                                FieldDefnVariant::StructDerivedLazy { ref defn_repr } => {
                                    let repr = FeatureRepr::from_defn(
                                        self.db,
                                        Some(this.clone().into()),
                                        defn_repr,
                                        self.db.feature_interner(),
                                    );
                                    let feature = repr.feature();
                                    let feature_expr_kind =
                                        FeatureLazyExprVariant::StructDerivedLazyFieldAccess {
                                            this,
                                            field_ident,
                                            repr,
                                        };
                                    (feature_expr_kind, feature)
                                }
                                _ => {
                                    panic!()
                                }
                            },
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            FieldKind::RecordOriginal => {
                let repr = self
                    .db
                    .record_field_repr(this.clone().into(), field_ident.ident);
                let feature = repr.feature();
                (
                    FeatureLazyExprVariant::RecordOriginalFieldAccess {
                        this,
                        field_ident,
                        repr,
                    },
                    feature,
                )
            }
            FieldKind::RecordDerived => {
                let this_ty_defn = self.db.compile_time().entity_defn(this.ty()).unwrap();
                let field_uid =
                    self.db
                        .compile_time()
                        .entity_uid(self.db.compile_time().intern_entity_route(
                            EntityRoute::subroute(this.ty(), field_ident.ident, thin_vec![]),
                        ));
                match this_ty_defn.variant {
                    EntityDefnVariant::Ty { ref ty_members, .. } => {
                        match ty_members.get_entry(field_ident.ident).unwrap().variant {
                            EntityDefnVariant::TyField {
                                ref field_variant, ..
                            } => match field_variant {
                                FieldDefnVariant::RecordDerived { defn_repr } => {
                                    let repr = FeatureRepr::from_defn(
                                        self.db,
                                        Some(this.clone().into()),
                                        defn_repr,
                                        self.db.feature_interner(),
                                    );
                                    let feature =
                                        self.db.feature_interner().intern(Feature::FieldAccess {
                                            this: this.feature(),
                                            field_ident: field_ident.ident,
                                        });
                                    let feature_expr_kind =
                                        FeatureLazyExprVariant::RecordDerivedFieldAccess {
                                            this,
                                            field_ident,
                                            repr,
                                        };
                                    (feature_expr_kind, feature)
                                }
                                _ => {
                                    panic!()
                                }
                            },
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }

    fn compile_element_access(
        &self,
        opds: &[Arc<LazyExpr>],
        expr: &Arc<LazyExpr>,
        element_binding: Binding,
    ) -> (FeatureLazyExprVariant, FeaturePtr) {
        let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
        let feature = self.features.intern(Feature::ElementAccess {
            opds: opds.map(|opd| opd.feature),
        });
        let feature_expr_kind = FeatureLazyExprVariant::ElementAccess {
            linkage: self
                .db
                .compile_time()
                .element_access_linkage(opds.map(|opd| opd.expr.ty()))
                .bind(element_binding),
            opds,
        };
        (feature_expr_kind, feature)
    }

    fn record_field_value(
        &self,
        this: &FeatureExpr,
        field_ident: CustomIdentifier,
    ) -> Arc<FeatureExpr> {
        match this.variant {
            FeatureLazyExprVariant::Variable { .. } => todo!(),
            FeatureLazyExprVariant::RecordOriginalFieldAccess { .. } => todo!(),
            FeatureLazyExprVariant::EntityFeature {
                repr: ref block, ..
            } => todo!(),
            // self.derive_record_field_value_from_block(block, field_ident),
            FeatureLazyExprVariant::NewRecord {
                ref entity,
                ref opds,
                ..
            } => match entity.variant {
                EntityDefnVariant::Ty { kind, .. } => todo!(),
                // p!(field_ident, ty.fields);
                // let idx = ty.fields.position(field_ident).unwrap();
                // opds[idx].clone()
                _ => panic!(),
            },
            FeatureLazyExprVariant::RoutineCall { .. }
            | FeatureLazyExprVariant::EnumKindLiteral { .. }
            | FeatureLazyExprVariant::PrimitiveBinaryOpr { .. }
            | FeatureLazyExprVariant::StructOriginalFieldAccess { .. }
            | FeatureLazyExprVariant::PrimitiveLiteral(_) => {
                panic!()
            }
            FeatureLazyExprVariant::ThisValue { ref repr } => todo!(),
            FeatureLazyExprVariant::EvalInput => todo!(),
            FeatureLazyExprVariant::RecordDerivedFieldAccess { .. } => todo!(),
            FeatureLazyExprVariant::ElementAccess { ref opds, .. } => todo!(),
            FeatureLazyExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => todo!(),
            _ => todo!(),
        }
    }

    // RecordMembExpr {
    //     feature: result.feature,
    //     kind: RecordMembExprKind::Expr(result.clone()),
    // },
    fn derive_record_field_value_from_block(
        &self,
        block: &FeatureLazyBlock,
        field_ident: CustomIdentifier,
    ) -> Arc<FeatureExpr> {
        todo!()
        // let stmt_features = block.stmt_features();
        // if stmt_features.len() == 1 {
        //     match block.stmts.last().unwrap().variant {
        //         FeatureStmtVariant::Return { ref result } => {
        //             self.record_field_value(result, field_ident)
        //         }
        //         FeatureStmtVariant::ConditionFlow { ref branches } => todo!(),
        //         _ => panic!(),
        //     }
        // } else {
        //     todo!()
        // }
    }
}
