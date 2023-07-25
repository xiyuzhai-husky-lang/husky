use super::*;

use husky_item_semantics::EntityDefnVariant;
use husky_linkage_table::ResolveLinkage;
use husky_vm::{Binding, __root::__NEQ_LINKAGE};
use husky_vm::{__Linkage, __root::__EQ_LINKAGE};
use husky_vm_primitive_opr_linkage::{
    resolve_primitive_prefix_opr_linkage, resolve_primitive_pure_binary_opr_linkage,
};
use map_collect::MapCollect;

impl<'a> FeatureExprBuilder<'a> {
    pub(super) fn compile_opn(
        &self,
        opn_kind: LazyOpnKind,
        opds: &[Arc<LazyExpr>],
        expr: &Arc<LazyExpr>,
    ) -> (FeatureLazyExprVariant, FeatureItd) {
        todo!()
        // match opn_kind {
        //     LazyOpnKind::Binary { opr, this } => {
        //         let lopd = self.new_expr(opds[0].clone());
        //         let ropd = self.new_expr(opds[1].clone());
        //         self.compile_binary_opn(this, lopd, opr, ropd)
        //     }
        //     LazyOpnKind::Prefix(_) => todo!(),
        //     LazyOpnKind::FunctionModelCall(routine) => {
        //         let uid = self.db.item_uid(routine.route);
        //         let opds = opds
        //             .iter()
        //             .map(|opd| self.new_expr(opd.clone()))
        //             .collect::<Vec<_>>();
        //         let feature = self.feature_interner.intern(Feature::FunctionCall {
        //             func: routine.route,
        //             uid,
        //             inputs: opds.iter().map(|expr| expr.feature).collect(),
        //         });
        //         let model_defn = self.db.item_defn(routine.route).unwrap();
        //         let internal = match model_defn.variant {
        //             EntityDefnVariant::Function {
        //                 source: CallFormSource::Static(__Linkage::Model(model)),
        //                 ..
        //             } => self.db.train(model, self.opt_arrival_indicator, &opds),
        //             _ => todo!(),
        //         };
        //         let kind = FeatureLazyExprVariant::ModelCall {
        //             opds,
        //             has_this: false,
        //             model_defn,
        //             internal,
        //             opt_arrival_indicator: self
        //                 .opt_arrival_indicator
        //                 .map(|branch_indicator| branch_indicator.clone()),
        //         };
        //         (kind, feature)
        //     }
        //     LazyOpnKind::FunctionRoutineCall(routine) => {
        //         let uid = self.db.item_uid(routine.route);
        //         let opds = opds
        //             .iter()
        //             .map(|opd| self.new_expr(opd.clone()))
        //             .collect::<Vec<_>>();
        //         let feature = self.feature_interner.intern(Feature::FunctionCall {
        //             func: routine.route,
        //             uid,
        //             inputs: opds.iter().map(|expr| expr.feature).collect(),
        //         });
        //         let routine_defn = self.db.item_defn(routine.route).unwrap();
        //         let opt_linkage = self.db.routine_linkage(routine.route);
        //         let kind = FeatureLazyExprVariant::RoutineCall {
        //             opt_linkage,
        //             opds,
        //             has_this: false,
        //             opt_instruction_sheet: self.db.item_instruction_sheet(routine.route),
        //             routine_defn,
        //         };
        //         (kind, feature)
        //     }
        //     LazyOpnKind::Field {
        //         field_ident,
        //         field_binding,
        //     } => self.compile_field_access(
        //         field_ident,
        //         FeatureLazyExpr::new(
        //             self.db,
        //             self.opt_this.clone(),
        //             opds[0].clone(),
        //             self.symbols,
        //             self.opt_arrival_indicator,
        //             self.feature_interner,
        //         )
        //         .into(),
        //         field_binding,
        //     ),
        //     LazyOpnKind::MethodCall {
        //         method_ident,
        //         method_route,
        //         ..
        //     } => self.compile_method_call(method_ident, method_route, opds),
        //     LazyOpnKind::Index { element_binding } => self.compile_index(opds, element_binding),
        //     LazyOpnKind::StructCall(_) => todo!(),
        //     LazyOpnKind::RecordCall(ty) => {
        //         let uid = self.db.item_uid(ty.route);
        //         let opds = opds
        //             .iter()
        //             .map(|opd| self.new_expr(opd.clone()))
        //             .collect::<Vec<_>>();
        //         let feature = self.feature_interner.intern(Feature::RecordTypeCall {
        //             ty: ty.route,
        //             uid,
        //             opds: opds.iter().map(|opd| opd.feature).collect(),
        //         });
        //         let kind = FeatureLazyExprVariant::NewRecord {
        //             ty,
        //             item: self.db.item_defn(ty.route).unwrap(),
        //             opds,
        //         };
        //         (kind, feature)
        //     }
        //     LazyOpnKind::NewVecFromList => {
        //         let ty = expr.intrinsic_ty();
        //         let elements = opds
        //             .iter()
        //             .map(|opd| self.new_expr(opd.clone()))
        //             .collect::<Vec<_>>();
        //         let feature = self.feature_interner.intern(Feature::NewVecFromList {
        //             elements: elements.iter().map(|elem| elem.feature).collect(),
        //         });
        //         let kind = FeatureLazyExprVariant::NewVecFromList {
        //             elements,
        //             linkage: self.db.type_call_linkage(ty).unwrap(),
        //         };
        //         (kind, feature)
        //     }
        // }
    }

    fn compile_binary_opn(
        &self,
        this: EtherealTerm,
        lopd: Arc<FeatureLazyExpr>,
        opr: BinaryPureClosedOpr,
        ropd: Arc<FeatureLazyExpr>,
    ) -> (FeatureLazyExprVariant, FeatureItd) {
        todo!()
        // match this {
        //     EtherealTerm::Root(RootBuiltinIdentifier::Void)
        //     | EtherealTerm::Root(RootBuiltinIdentifier::I32)
        //     | EtherealTerm::Root(RootBuiltinIdentifier::F32)
        //     | EtherealTerm::Root(RootBuiltinIdentifier::F64)
        //     | EtherealTerm::Root(RootBuiltinIdentifier::B32)
        //     | EtherealTerm::Root(RootBuiltinIdentifier::B64) => {
        //         let feature = self.feature_interner.intern(Feature::PrimitiveBinaryOpr {
        //             opr,
        //             lopd: lopd.feature,
        //             ropd: ropd.feature,
        //         });
        //         (
        //             FeatureLazyExprVariant::PrimitiveBinaryOpr {
        //                 opr,
        //                 linkage: resolve_primitive_pure_binary_opr_linkage(
        //                     lopd.expr.intrinsic_ty().root(),
        //                     opr,
        //                     ropd.expr.intrinsic_ty().root(),
        //                 ),
        //                 opds: vec![lopd, ropd],
        //             },
        //             feature,
        //         )
        //     }
        //     EtherealTerm::Root(RootBuiltinIdentifier::Bool) => {
        //         let feature = self.feature_interner.intern(Feature::PrimitiveBinaryOpr {
        //             opr,
        //             lopd: lopd.feature,
        //             ropd: ropd.feature,
        //         });
        //         todo!()
        //         // match opr {
        //         //     BinaryShortcuitLogicOpr::And | BinaryShortcuitLogicOpr::Or => (
        //         //         FeatureLazyExprVariant::ShortCircuitBinaryOpr {
        //         //             opr,
        //         //             opds: vec![lopd, ropd],
        //         //         },
        //         //         feature,
        //         //     ),
        //         //     BinaryPureClosedOpr::Add => todo!(),
        //         //     BinaryPureClosedOpr::BitAnd => todo!(),
        //         //     BinaryPureClosedOpr::BitOr => todo!(),
        //         //     BinaryPureClosedOpr::BitXor => todo!(),
        //         //     BinaryPureClosedOpr::Div => todo!(),
        //         //     BinaryComparisonOpr::Eq => todo!(),
        //         //     BinaryComparisonOpr::Geq => todo!(),
        //         //     BinaryComparisonOpr::Greater => todo!(),
        //         //     BinaryComparisonOpr::Leq => todo!(),
        //         //     BinaryComparisonOpr::Less => todo!(),
        //         //     BinaryPureClosedOpr::Mul => todo!(),
        //         //     BinaryComparisonOpr::Neq => todo!(),
        //         //     BinaryPureClosedOpr::RemEuclid => todo!(),
        //         //     BinaryPureClosedOpr::Power => todo!(),
        //         //     BinaryPureClosedOpr::Shl => todo!(),
        //         //     BinaryPureClosedOpr::Shr => todo!(),
        //         //     BinaryPureClosedOpr::Sub => todo!(),
        //         // }
        //     }
        //     _ => self.compile_custom_binary_opn(lopd, opr, ropd),
        // }
    }

    fn compile_prefix_opn(
        &self,
        opd: Arc<FeatureLazyExpr>,
        opr: PrefixOpr,
    ) -> (FeatureLazyExprVariant, FeaturePtr) {
        let feature = self.feature_interner.intern(Feature::PrefixOpr {
            opr,
            opd: opd.feature,
        });
        (
            FeatureLazyExprVariant::PrefixOpr {
                opr,
                linkage: resolve_primitive_prefix_opr_linkage(opr, opd.expr.intrinsic_ty().root()),
                opds: vec![opd],
            },
            feature,
        )
    }

    fn compile_custom_binary_opn(
        &self,
        lopd: Arc<FeatureLazyExpr>,
        opr: BinaryPureClosedOpr,
        ropd: Arc<FeatureLazyExpr>,
    ) -> (FeatureLazyExprVariant, FeatureItd) {
        todo!()
        // let (opt_instruction_sheet, opt_linkage) = match opr {
        //     BinaryComparisonOpr::Eq => (None, Some(__EQ_LINKAGE)),
        //     BinaryComparisonOpr::Neq => (None, Some(__NEQ_LINKAGE)),
        //     BinaryPureClosedOpr::Add => todo!(),
        //     BinaryShortcuitLogicOpr::And => todo!(),
        //     BinaryPureClosedOpr::BitAnd => todo!(),
        //     BinaryPureClosedOpr::BitOr => todo!(),
        //     BinaryPureClosedOpr::BitXor => todo!(),
        //     BinaryPureClosedOpr::Div => todo!(),
        //     BinaryComparisonOpr::Geq => todo!(),
        //     BinaryComparisonOpr::Greater => todo!(),
        //     BinaryComparisonOpr::Leq => todo!(),
        //     BinaryComparisonOpr::Less => todo!(),
        //     BinaryPureClosedOpr::Mul => todo!(),
        //     BinaryPureClosedOpr::RemEuclid => todo!(),
        //     BinaryShortcuitLogicOpr::Or => {
        //         p!(lopd.expr.ty);
        //         todo!()
        //     }
        //     BinaryPureClosedOpr::Power => todo!(),
        //     BinaryPureClosedOpr::Shl => todo!(),
        //     BinaryPureClosedOpr::Shr => todo!(),
        //     BinaryPureClosedOpr::Sub => todo!(),
        // };
        // let feature = self.feature_interner.intern(Feature::PrimitiveBinaryOpr {
        //     opr,
        //     lopd: lopd.feature,
        //     ropd: ropd.feature,
        // });
        // (
        //     FeatureLazyExprVariant::CustomBinaryOpr {
        //         opr,
        //         opds: vec![lopd, ropd],
        //         opt_instruction_sheet,
        //         opt_linkage,
        //     },
        //     feature,
        // )
    }

    fn compile_method_call(
        &self,
        _method_ident: RangedIdentifier,
        _method_route: EtherealTerm,
        _opds: &[Arc<LazyExpr>],
    ) -> (FeatureLazyExprVariant, FeatureItd) {
        todo!()
        // let opds = opds
        //     .iter()
        //     .map(|opd| self.new_expr(opd.clone()))
        //     .collect::<Vec<_>>();
        // let feature = self.feature_interner.intern(Feature::MethodCall {
        //     method_ident: method_ident.ident,
        //     opds: opds.iter().map(|opd| opd.feature).collect(),
        // });
        // let self_expr = &opds[0].expr;
        // let this_ty_defn = self.db.item_defn(self_expr.intrinsic_ty()).unwrap();
        // let member_idx = self.db.member_idx(method_route);
        // let method_defn = this_ty_defn.method(member_idx);
        // let kind = match method_defn.variant {
        //     EntityDefnVariant::Method { .. } => FeatureLazyExprVariant::RoutineCall {
        //         opt_instruction_sheet: self.db.method_opt_instruction_sheet(method_route),
        //         opt_linkage: self.db.method_linkage(method_route),
        //         opds,
        //         has_this: true,
        //         routine_defn: method_defn.clone(),
        //     },
        //     _ => panic!(
        //         "unexpected item variant {:?} for method `{method_route}`",
        //         method_defn.variant
        //     ),
        // };
        // (kind, feature)
    }

    pub(super) fn compile_field_access(
        &self,
        _field_ident: RangedIdentifier,
        _this: ValRepr,
        _field_binding: Binding,
    ) -> (FeatureLazyExprVariant, FeatureItd) {
        todo!()
        // let this_ty = this.ty();
        // let this_ty_decl = self.db.ty_decl(this_ty.intrinsic()).unwrap();
        // let field_kind = this_ty_decl.field_kind(field_ident.ident);
        // match field_kind {
        //     FieldKind::StructRegular | FieldKind::StructDefault | FieldKind::StructDerived => {
        //         let feature = self.feature_interner.intern(Feature::FieldAccess {
        //             this: this.feature(),
        //             field_ident: field_ident.ident,
        //         });
        //         (
        //             FeatureLazyExprVariant::StructOriginalField {
        //                 field_ident,
        //                 field_idx: this_ty_decl
        //                     .field_idx(field_ident.ident)
        //                     .try_into()
        //                     .unwrap(),
        //                 field_binding,
        //                 opt_linkage: self.db.field_linkage_resolved(
        //                     this_ty.intrinsic(),
        //                     field_ident.ident,
        //                     field_binding,
        //                 ),
        //                 this,
        //             },
        //             feature,
        //         )
        //     }
        //     FieldKind::StructMemo { .. } => {
        //         let this_ty = this.ty();
        //         let this_ty_defn = self.db.item_defn(this_ty).unwrap();
        //         let lazy_field_route = self.db.subroute(this_ty, field_ident.ident, thin_vec![]);
        //         let field_uid = self.db.item_uid(lazy_field_route);
        //         match this_ty_defn.variant {
        //             EntityDefnVariant::EtherealTerm { ref ty_members, .. } => {
        //                 match ty_members.get_entry(field_ident.ident).unwrap().variant {
        //                     EntityDefnVariant::TyField {
        //                         ref field_variant, ..
        //                     } => match field_variant {
        //                         FieldDefnVariant::StructDerivedLazy { ref defn_repr } => {
        //                             let repr = ValRepr::from_defn(
        //                                 self.db,
        //                                 Some(this.clone().into()),
        //                                 defn_repr,
        //                                 self.db.feature_interner(),
        //                             );
        //                             let feature = repr.feature();
        //                             let feature_expr_kind =
        //                                 FeatureLazyExprVariant::StructDerivedLazyField {
        //                                     this,
        //                                     field_ident,
        //                                     field_uid,
        //                                     repr,
        //                                 };
        //                             (feature_expr_kind, feature)
        //                         }
        //                         _ => {
        //                             panic!()
        //                         }
        //                     },
        //                     _ => panic!(),
        //                 }
        //             }
        //             _ => panic!(),
        //         }
        //     }
        //     FieldKind::RecordRegular => {
        //         let repr = self
        //             .db
        //             .record_field_repr(this.clone().into(), field_ident.ident);
        //         let feature = repr.feature();
        //         (
        //             FeatureLazyExprVariant::RecordOriginalField {
        //                 this,
        //                 field_ident,
        //                 repr,
        //             },
        //             feature,
        //         )
        //     }
        //     FieldKind::RecordProperty => {
        //         let this_ty_defn = self.db.item_defn(this.ty()).unwrap();
        //         let field_uid =
        //             self.db
        //                 .item_uid(self.db.intern_item_route(EntityRoute::subroute(
        //                     this.ty(),
        //                     field_ident.ident,
        //                     thin_vec![],
        //                 )));
        //         match this_ty_defn.variant {
        //             EntityDefnVariant::EtherealTerm { ref ty_members, .. } => {
        //                 match ty_members.get_entry(field_ident.ident).unwrap().variant {
        //                     EntityDefnVariant::TyField {
        //                         ref field_variant, ..
        //                     } => match field_variant {
        //                         FieldDefnVariant::RecordDerived { defn_repr } => {
        //                             let repr = ValRepr::from_defn(
        //                                 self.db,
        //                                 Some(this.clone().into()),
        //                                 defn_repr,
        //                                 self.db.feature_interner(),
        //                             );
        //                             let feature =
        //                                 self.db.feature_interner().intern(Feature::FieldAccess {
        //                                     this: this.feature(),
        //                                     field_ident: field_ident.ident,
        //                                 });
        //                             let feature_expr_kind =
        //                                 FeatureLazyExprVariant::RecordDerivedField {
        //                                     this,
        //                                     field_ident,
        //                                     field_uid,
        //                                     repr,
        //                                 };
        //                             (feature_expr_kind, feature)
        //                         }
        //                         _ => {
        //                             panic!()
        //                         }
        //                     },
        //                     _ => panic!(),
        //                 }
        //             }
        //             _ => panic!(),
        //         }
        //     }
        // }
    }

    fn compile_index(
        &self,
        opds: &[Arc<LazyExpr>],
        element_binding: Binding,
    ) -> (FeatureLazyExprVariant, FeatureItd) {
        let opds: Vec<_> = opds.iter().map(|opd| self.new_expr(opd.clone())).collect();
        let feature = self.feature_interner.intern(Feature::Index {
            opds: opds.map(|opd| opd.feature),
        });
        let feature_expr_kind = FeatureLazyExprVariant::Index {
            linkage: self
                .db
                .index_linkage(opds.map(|opd| opd.expr.intrinsic_ty()))
                .bind(element_binding),
            opds,
        };
        (feature_expr_kind, feature)
    }
}
