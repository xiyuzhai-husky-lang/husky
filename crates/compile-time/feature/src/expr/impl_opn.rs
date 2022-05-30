use entity_kind::{FieldKind, TyKind};
use entity_route::EntityRoute;
use map_collect::MapCollect;
use static_defn::LinkageSource;
use vm::{Binding, LazyContract};

use super::*;

impl<'a> FeatureExprBuilder<'a> {
    pub(super) fn compile_opn(
        &self,
        opn_kind: LazyOpnKind,
        opds: &[Arc<LazyExpr>],
        expr: &Arc<LazyExpr>,
    ) -> (FeatureExprVariant, FeaturePtr) {
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
                        FeatureExprVariant::PrimitiveBinaryOpr { opr, lopd, ropd },
                        feature,
                    )
                }
                _ => todo!(),
            },
            LazyOpnKind::Prefix(_) => todo!(),
            LazyOpnKind::NormalRoutineCall(routine) => {
                let uid = self.db.entity_uid(routine.route);
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
                let feature = self.features.alloc(Feature::FuncCall {
                    func: routine.route,
                    uid,
                    inputs: opds.iter().map(|expr| expr.feature).collect(),
                });
                let routine_defn = self.db.entity_defn(routine.route).unwrap();
                let kind = FeatureExprVariant::RoutineCall {
                    opt_linkage: self.db.routine_linkage(routine.route),
                    opds,
                    has_this: false,
                    opt_instruction_sheet: self.db.entity_instruction_sheet(routine.route),
                    routine_defn,
                };
                (kind, feature)
            }
            LazyOpnKind::PatternCall => todo!(),
            LazyOpnKind::FieldAccess {
                field_ident,
                field_kind,
                field_binding,
            } => self.compile_field_access(field_ident, field_kind, opds, field_binding),
            LazyOpnKind::MethodCall {
                method_ident,
                method_route,
                opt_output_binding,
            } => self.compile_method_call(method_ident, method_route, opds, opt_output_binding),
            LazyOpnKind::ElementAccess { element_binding } => {
                self.compile_element_access(opds, expr, element_binding)
            }
            LazyOpnKind::StructCall(_) => todo!(),
            LazyOpnKind::RecordCall(ty) => {
                let uid = self.db.entity_uid(ty.route);
                let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
                let feature = self.features.alloc(Feature::RecordTypeCall {
                    ty: ty.route,
                    uid,
                    opds: opds.iter().map(|opd| opd.feature).collect(),
                });
                let kind = FeatureExprVariant::NewRecord {
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
        opt_output_binding: Option<Binding>,
    ) -> (FeatureExprVariant, FeaturePtr) {
        let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
        let feature = self.features.alloc(Feature::MethodCall {
            method_ident: method_ident.ident,
            opds: opds.iter().map(|opd| opd.feature).collect(),
        });
        let this_expr = &opds[0].expr;
        let this_ty_defn = self.db.entity_defn(this_expr.ty()).unwrap();
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
                FeatureExprVariant::RoutineCall {
                    opt_instruction_sheet: self.db.method_opt_instruction_sheet(method_route),
                    opt_linkage: self.db.method_linkage(method_route, opt_output_binding),
                    opds,
                    has_this: true,
                    routine_defn: method_defn.clone(),
                }
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
        field_binding: Binding,
    ) -> (FeatureExprVariant, FeaturePtr) {
        let this = self.new_expr(opds[0].clone());
        let this_ty_decl = self.db.ty_decl(this.expr.ty()).unwrap();
        match field_access_kind {
            FieldKind::StructOriginal => {
                let feature = self.features.alloc(Feature::FieldAccess {
                    this: this.feature,
                    field_ident: field_ident.ident,
                });
                (
                    FeatureExprVariant::StructOriginalFieldAccess {
                        field_ident,
                        field_idx: this_ty_decl.field_idx(field_ident.ident),
                        field_binding,
                        opt_linkage: self
                            .db
                            .struct_field_access(this.expr.ty(), field_ident.ident)
                            .map(|linkage_source| match linkage_source {
                                LinkageSource::MemberAccess {
                                    ref_access,
                                    move_access,
                                    ref_mut_access: borrow_mut_access,
                                    ..
                                } => todo!(),
                                LinkageSource::Transfer(_) => todo!(),
                            }),
                        this,
                    },
                    feature,
                )
            }
            FieldKind::StructDerivedLazy { .. } => {
                let this_ty_defn = self.db.entity_defn(this.expr.ty()).unwrap();
                let field_uid =
                    self.db
                        .entity_uid(self.db.intern_entity_route(EntityRoute::subroute(
                            this.expr.ty(),
                            field_ident.ident,
                            vec![],
                        )));
                match this_ty_defn.variant {
                    EntityDefnVariant::Type { ref ty_members, .. } => {
                        match ty_members.get_entry(field_ident.ident).unwrap().variant {
                            EntityDefnVariant::TypeField {
                                ref field_variant, ..
                            } => match field_variant {
                                FieldDefnVariant::StructDerived { ref defn_repr } => {
                                    let repr = FeatureRepr::from_defn(
                                        self.db,
                                        Some(this.clone().into()),
                                        defn_repr,
                                        self.db.features(),
                                    );
                                    let feature = repr.feature();
                                    let feature_expr_kind =
                                        FeatureExprVariant::StructDerivedFieldAccess {
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
                    FeatureExprVariant::RecordOriginalFieldAccess {
                        this,
                        field_ident,
                        repr,
                    },
                    feature,
                )
            }
            FieldKind::RecordDerived => {
                let this_ty_defn = self.db.entity_defn(this.expr.ty()).unwrap();
                let field_uid =
                    self.db
                        .entity_uid(self.db.intern_entity_route(EntityRoute::subroute(
                            this.expr.ty(),
                            field_ident.ident,
                            vec![],
                        )));
                match this_ty_defn.variant {
                    EntityDefnVariant::Type { ref ty_members, .. } => {
                        match ty_members.get_entry(field_ident.ident).unwrap().variant {
                            EntityDefnVariant::TypeField {
                                ref field_variant, ..
                            } => match field_variant {
                                FieldDefnVariant::RecordDerived { defn_repr } => {
                                    todo!()
                                    // let block = FeatureLazyBlock::new(
                                    //     self.db,
                                    //     Some(this.clone().into()),
                                    //     defn_repr.clone(),
                                    //     &[],
                                    //     self.db.features(),
                                    // );
                                    // let feature = self.db.features().alloc(
                                    //     Feature::RecordDerivedFieldAccess {
                                    //         this: this.feature,
                                    //         field_uid,
                                    //     },
                                    // );
                                    // let feature_expr_kind =
                                    //     FeatureExprVariant::RecordDerivedFieldAccess {
                                    //         this,
                                    //         field_ident,
                                    //         block,
                                    //     };
                                    // (feature_expr_kind, feature)
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
            FieldKind::StructDefault => todo!(),
            FieldKind::StructDerivedEager => todo!(),
        }
    }

    fn compile_element_access(
        &self,
        opds: &[Arc<LazyExpr>],
        expr: &Arc<LazyExpr>,
        element_binding: Binding,
    ) -> (FeatureExprVariant, FeaturePtr) {
        let opds: Vec<_> = opds.map(|opd| self.new_expr(opd.clone()));
        let feature = self.features.alloc(Feature::ElementAccess {
            opds: opds.map(|opd| opd.feature),
        });
        let feature_expr_kind = FeatureExprVariant::ElementAccess {
            linkage: self
                .db
                .element_access_linkage(opds.map(|opd| opd.expr.ty()), element_binding),
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
            FeatureExprVariant::Variable { .. } => todo!(),
            FeatureExprVariant::RecordOriginalFieldAccess { .. } => todo!(),
            FeatureExprVariant::EntityFeature {
                repr: ref block, ..
            } => todo!(),
            // self.derive_record_field_value_from_block(block, field_ident),
            FeatureExprVariant::NewRecord {
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
            FeatureExprVariant::RoutineCall { .. }
            | FeatureExprVariant::EnumKindLiteral { .. }
            | FeatureExprVariant::PrimitiveBinaryOpr { .. }
            | FeatureExprVariant::StructOriginalFieldAccess { .. }
            | FeatureExprVariant::PrimitiveLiteral(_) => {
                panic!()
            }
            FeatureExprVariant::This { ref repr } => todo!(),
            FeatureExprVariant::GlobalInput => todo!(),
            FeatureExprVariant::PatternCall {} => todo!(),
            FeatureExprVariant::RecordDerivedFieldAccess { .. } => todo!(),
            FeatureExprVariant::ElementAccess { ref opds, .. } => todo!(),
            FeatureExprVariant::StructDerivedFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => todo!(),
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
