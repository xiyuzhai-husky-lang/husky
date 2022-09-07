use super::*;
use husky_entity_kind::FieldKind;
use husky_entity_route::{EntityRoute, InternEntityRoute};
use husky_entity_semantics::EntityDefnVariant;
use husky_linkage_table::ResolveLinkage;
use husky_vm::{Binding, __root::__NEQ_LINKAGE};
use husky_vm::{__Linkage, __root::__EQ_LINKAGE};
use husky_vm_primitive_opr_linkage::resolve_primitive_pure_binary_opr_linkage;
use map_collect::MapCollect;
use thin_vec::thin_vec;

impl<'a> FeatureExprBuilder<'a> {
    pub(super) fn compile_opn(
        &self,
        opn_kind: LazyOpnKind,
        opds: &[Arc<LazyExpr>],
        expr: &Arc<LazyExpr>,
    ) -> (FeatureLazyExprVariant, FeaturePtr) {
        match opn_kind {
            LazyOpnKind::Binary { opr, this } => {
                let lopd = self.new_expr(opds[0].clone());
                let ropd = self.new_expr(opds[1].clone());
                self.compile_binary_opn(this, lopd, opr, ropd)
            }
            LazyOpnKind::Prefix(_) => todo!(),
            LazyOpnKind::FunctionModelCall(routine) => {
                let uid = self.db.comptime().entity_uid(routine.route);
                let opds = opds
                    .iter()
                    .map(|opd| self.new_expr(opd.clone()))
                    .collect::<Vec<_>>();
                let feature = self.feature_interner.intern(Feature::FunctionCall {
                    func: routine.route,
                    uid,
                    inputs: opds.iter().map(|expr| expr.feature).collect(),
                });
                let model_defn = self.db.comptime().entity_defn(routine.route).unwrap();
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
                let uid = self.db.comptime().entity_uid(routine.route);
                let opds = opds
                    .iter()
                    .map(|opd| self.new_expr(opd.clone()))
                    .collect::<Vec<_>>();
                let feature = self.feature_interner.intern(Feature::FunctionCall {
                    func: routine.route,
                    uid,
                    inputs: opds.iter().map(|expr| expr.feature).collect(),
                });
                let routine_defn = self.db.comptime().entity_defn(routine.route).unwrap();
                let opt_linkage = self.db.comptime().routine_linkage(routine.route);
                let kind = FeatureLazyExprVariant::RoutineCall {
                    opt_linkage,
                    opds,
                    has_this: false,
                    opt_instruction_sheet: self.db.entity_instruction_sheet(routine.route),
                    routine_defn,
                };
                (kind, feature)
            }
            LazyOpnKind::Field {
                field_ident,
                field_binding,
            } => self.compile_field_access(
                field_ident,
                FeatureLazyExpr::new(
                    self.db,
                    self.opt_this.clone(),
                    opds[0].clone(),
                    self.symbols,
                    self.opt_arrival_indicator,
                    self.feature_interner,
                )
                .into(),
                field_binding,
            ),
            LazyOpnKind::MethodCall {
                method_ident,
                method_route,
                ..
            } => self.compile_method_call(method_ident, method_route, opds),
            LazyOpnKind::Index { element_binding } => {
                self.compile_element_access(opds, element_binding)
            }
            LazyOpnKind::StructCall(_) => todo!(),
            LazyOpnKind::RecordCall(ty) => {
                let uid = self.db.comptime().entity_uid(ty.route);
                let opds = opds
                    .iter()
                    .map(|opd| self.new_expr(opd.clone()))
                    .collect::<Vec<_>>();
                let feature = self.feature_interner.intern(Feature::RecordTypeCall {
                    ty: ty.route,
                    uid,
                    opds: opds.iter().map(|opd| opd.feature).collect(),
                });
                let kind = FeatureLazyExprVariant::NewRecord {
                    ty,
                    entity: self.db.comptime().entity_defn(ty.route).unwrap(),
                    opds,
                };
                (kind, feature)
            }
            LazyOpnKind::NewVecFromList => {
                let ty = expr.intrinsic_ty();
                let elements = opds
                    .iter()
                    .map(|opd| self.new_expr(opd.clone()))
                    .collect::<Vec<_>>();
                let feature = self.feature_interner.intern(Feature::NewVecFromList {
                    elements: elements.iter().map(|elem| elem.feature).collect(),
                });
                let kind = FeatureLazyExprVariant::NewVecFromList {
                    elements,
                    linkage: self.db.comptime().type_call_linkage(ty).unwrap(),
                };
                (kind, feature)
            }
        }
    }

    fn compile_binary_opn(
        &self,
        this: EntityRoutePtr,
        lopd: Arc<FeatureLazyExpr>,
        opr: PureBinaryOpr,
        ropd: Arc<FeatureLazyExpr>,
    ) -> (FeatureLazyExprVariant, interner::InternedPtr<Feature>) {
        match this {
            EntityRoutePtr::Root(RootIdentifier::Void)
            | EntityRoutePtr::Root(RootIdentifier::I32)
            | EntityRoutePtr::Root(RootIdentifier::F32)
            | EntityRoutePtr::Root(RootIdentifier::F64)
            | EntityRoutePtr::Root(RootIdentifier::B32)
            | EntityRoutePtr::Root(RootIdentifier::B64) => {
                let feature = self.feature_interner.intern(Feature::PrimitiveBinaryOpr {
                    opr,
                    lopd: lopd.feature,
                    ropd: ropd.feature,
                });
                (
                    FeatureLazyExprVariant::PrimitiveBinaryOpr {
                        opr,
                        linkage: resolve_primitive_pure_binary_opr_linkage(
                            lopd.expr.intrinsic_ty().root(),
                            opr,
                            ropd.expr.intrinsic_ty().root(),
                        ),
                        opds: vec![lopd, ropd],
                    },
                    feature,
                )
            }
            _ => self.compile_custom_binary_opn(lopd, opr, ropd),
        }
    }

    fn compile_custom_binary_opn(
        &self,
        lopd: Arc<FeatureLazyExpr>,
        opr: PureBinaryOpr,
        ropd: Arc<FeatureLazyExpr>,
    ) -> (FeatureLazyExprVariant, interner::InternedPtr<Feature>) {
        let (opt_instruction_sheet, opt_linkage) = match opr {
            PureBinaryOpr::Eq => (None, Some(__EQ_LINKAGE)),
            PureBinaryOpr::Neq => (None, Some(__NEQ_LINKAGE)),
            PureBinaryOpr::Add => todo!(),
            PureBinaryOpr::And => todo!(),
            PureBinaryOpr::BitAnd => todo!(),
            PureBinaryOpr::BitOr => todo!(),
            PureBinaryOpr::BitXor => todo!(),
            PureBinaryOpr::Div => todo!(),
            PureBinaryOpr::Geq => todo!(),
            PureBinaryOpr::Greater => todo!(),
            PureBinaryOpr::Leq => todo!(),
            PureBinaryOpr::Less => todo!(),
            PureBinaryOpr::Mul => todo!(),
            PureBinaryOpr::RemEuclid => todo!(),
            PureBinaryOpr::Or => {
                p!(lopd.expr.qualified_ty);
                todo!()
            }
            PureBinaryOpr::Power => todo!(),
            PureBinaryOpr::Shl => todo!(),
            PureBinaryOpr::Shr => todo!(),
            PureBinaryOpr::Sub => todo!(),
        };
        let feature = self.feature_interner.intern(Feature::PrimitiveBinaryOpr {
            opr,
            lopd: lopd.feature,
            ropd: ropd.feature,
        });
        (
            FeatureLazyExprVariant::CustomBinaryOpr {
                opr,
                opds: vec![lopd, ropd],
                opt_instruction_sheet,
                opt_linkage,
            },
            feature,
        )
    }

    fn compile_method_call(
        &self,
        method_ident: RangedCustomIdentifier,
        method_route: EntityRoutePtr,
        opds: &[Arc<LazyExpr>],
    ) -> (FeatureLazyExprVariant, FeaturePtr) {
        let opds = opds
            .iter()
            .map(|opd| self.new_expr(opd.clone()))
            .collect::<Vec<_>>();
        let feature = self.feature_interner.intern(Feature::MethodCall {
            method_ident: method_ident.ident,
            opds: opds.iter().map(|opd| opd.feature).collect(),
        });
        let this_expr = &opds[0].expr;
        let this_ty_defn = self
            .db
            .comptime()
            .entity_defn(this_expr.intrinsic_ty())
            .unwrap();
        let member_idx = self.db.comptime().member_idx(method_route);
        let method_defn = this_ty_defn.method(member_idx);
        let kind = match method_defn.variant {
            EntityDefnVariant::Method { .. } => FeatureLazyExprVariant::RoutineCall {
                opt_instruction_sheet: self.db.method_opt_instruction_sheet(method_route),
                opt_linkage: self.db.comptime().method_linkage(method_route),
                opds,
                has_this: true,
                routine_defn: method_defn.clone(),
            },
            _ => panic!(
                "unexpected entity variant {:?} for method `{method_route}`",
                method_defn.variant
            ),
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
        let this_ty_decl = self.db.comptime().ty_decl(this_ty.intrinsic()).unwrap();
        let field_kind = this_ty_decl.field_kind(field_ident.ident);
        match field_kind {
            FieldKind::StructRegular | FieldKind::StructDefault | FieldKind::StructDerived => {
                let feature = self.feature_interner.intern(Feature::FieldAccess {
                    this: this.feature(),
                    field_ident: field_ident.ident,
                });
                (
                    FeatureLazyExprVariant::StructOriginalField {
                        field_ident,
                        field_idx: this_ty_decl
                            .field_idx(field_ident.ident)
                            .try_into()
                            .unwrap(),
                        field_binding,
                        opt_linkage: self.db.comptime().field_linkage_resolved(
                            this_ty.intrinsic(),
                            field_ident.ident,
                            field_binding,
                        ),
                        this,
                    },
                    feature,
                )
            }
            FieldKind::StructMemo { .. } => {
                let this_ty = this.ty();
                let this_ty_defn = self.db.comptime().entity_defn(this_ty).unwrap();
                let lazy_field_route =
                    self.db
                        .comptime()
                        .subroute(this_ty, field_ident.ident, thin_vec![]);
                let field_uid = self.db.comptime().entity_uid(lazy_field_route);
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
                                        FeatureLazyExprVariant::StructDerivedLazyField {
                                            this,
                                            field_ident,
                                            field_uid,
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
            FieldKind::RecordRegular => {
                let repr = self
                    .db
                    .record_field_repr(this.clone().into(), field_ident.ident);
                let feature = repr.feature();
                (
                    FeatureLazyExprVariant::RecordOriginalField {
                        this,
                        field_ident,
                        repr,
                    },
                    feature,
                )
            }
            FieldKind::RecordProperty => {
                let this_ty_defn = self.db.comptime().entity_defn(this.ty()).unwrap();
                let field_uid =
                    self.db
                        .comptime()
                        .entity_uid(
                            self.db
                                .comptime()
                                .intern_entity_route(EntityRoute::subroute(
                                    this.ty(),
                                    field_ident.ident,
                                    thin_vec![],
                                )),
                        );
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
                                        FeatureLazyExprVariant::RecordDerivedField {
                                            this,
                                            field_ident,
                                            field_uid,
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
        element_binding: Binding,
    ) -> (FeatureLazyExprVariant, FeaturePtr) {
        let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
        let feature = self.feature_interner.intern(Feature::Index {
            opds: opds.map(|opd| opd.feature),
        });
        let feature_expr_kind = FeatureLazyExprVariant::Index {
            linkage: self
                .db
                .comptime()
                .index_linkage(opds.map(|opd| opd.expr.intrinsic_ty()))
                .bind(element_binding),
            opds,
        };
        (feature_expr_kind, feature)
    }
}
