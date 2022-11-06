use std::sync::Arc;

use crate::*;
use husky_entity_kind::TyKind;
use husky_entity_route::{EntityKind, EntityRoutePtr, RangedEntityRoute};
use husky_expr_syntax::*;
use husky_file::FileItd;
use husky_print_utils::p;
use husky_term_infer::TermInferDb;
use husky_text::RangedCustomIdentifier;
use husky_word::RootBuiltinIdentifier;

use super::*;
use husky_semantics_error::*;

// todo: opt_expectation

pub trait LazyExprParser<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn file(&self) -> FileItd;
    fn db(&self) -> &dyn TermInferDb;

    fn parse_lazy_expr(
        &mut self,
        idx: RawExprIdx,
        opt_expectation: Option<EntityRoutePtr>,
    ) -> SemanticResult<Arc<LazyExpr>> {
        todo!()
        // let raw_expr = &self.arena()[idx];
        // let kind: LazyExprVariant = match raw_expr.variant {
        //     RawExprVariant::Variable {
        //         varname,
        //         init_range,
        //     } => {
        //         let variable_qt = self
        //             .lazy_variable_qualified_ty(varname.into(), init_range)
        //             .unwrap();
        //         let contract = self.lazy_expr_contract(idx).unwrap();
        //         LazyExprVariant::Variable {
        //             varname,
        //             binding: variable_qt.qual.binding(contract),
        //         }
        //     }
        //     RawExprVariant::Unrecognized(ident) => {
        //         err!(format!(
        //             "unrecognized identifier {} at {:?}",
        //             ident,
        //             raw_expr.range()
        //         ))
        //     }
        //     RawExprVariant::Entity {
        //         route: entity_route,
        //         kind,
        //         ..
        //     } => match kind {
        //         EntityKind::Module => todo!(),
        //         EntityKind::EnumVariant => match entity_route {
        //             EntityRoutePtr::Root(RootBuiltinIdentifier::True) => {
        //                 LazyExprVariant::PrimitiveLiteral(RawLiteralData::Bool(true))
        //             }
        //             EntityRoutePtr::Root(RootBuiltinIdentifier::False) => {
        //                 LazyExprVariant::PrimitiveLiteral(RawLiteralData::Bool(false))
        //             }
        //             EntityRoutePtr::Custom(_) => LazyExprVariant::EnumLiteral { entity_route },
        //             _ => todo!(),
        //         },
        //         EntityKind::Type(_) => todo!(),
        //         EntityKind::Trait => todo!(),
        //         EntityKind::Function { .. } => {
        //             todo!()
        //         }
        //         EntityKind::Feature => LazyExprVariant::EntityFeature { entity_route },
        //         EntityKind::Member(_) => todo!(),
        //         EntityKind::Main => panic!(),
        //     },
        //     RawExprVariant::PrimitiveLiteral(value) => LazyExprVariant::PrimitiveLiteral(value),
        //     RawExprVariant::Bracketed(bracketed_expr) => {
        //         LazyExprVariant::Bracketed(self.parse_lazy_expr(bracketed_expr, opt_expectation)?)
        //     }
        //     RawExprVariant::Opn {
        //         opn_variant: ref opr,
        //         ref opds,
        //         ..
        //     } => self.parse_opn(idx, opr, opds)?,
        //     RawExprVariant::Lambda(_, _) => todo!(),
        //     RawExprVariant::ThisValue {
        //         opt_this_ty,
        //         opt_this_liason,
        //     } => LazyExprVariant::ThisValue {
        //         binding: {
        //             let this_contract = self.lazy_expr_contract(idx).unwrap();
        //             let this_qual = LazyExprQualifier::parameter_use_lazy_qualifier(
        //                 opt_this_liason.unwrap(),
        //                 self.decl_db().is_copyable(opt_this_ty.unwrap()).unwrap(),
        //                 this_contract,
        //             )
        //             .unwrap();
        //             let this_qt = LazyExprQualifiedTy::new(this_qual, opt_this_ty.unwrap());
        //             this_qt.binding(self.decl_db(), this_contract)
        //         },
        //     },
        //     RawExprVariant::ThisField {
        //         field_ident,
        //         opt_this_ty,
        //         opt_this_liason,
        //         field_liason,
        //         opt_field_ty,
        //     } => {
        //         let field_contract = self.lazy_expr_contract(idx).unwrap();
        //         let field_qt = self.lazy_expr_qualified_ty(idx).unwrap();
        //         let this_contract = LazyContract::member_self_lazy_contract(
        //             field_liason,
        //             field_contract,
        //             opt_field_ty.unwrap().route,
        //         )?;
        //         let this_qual = LazyExprQualifier::parameter_use_lazy_qualifier(
        //             opt_this_liason.unwrap(),
        //             self.decl_db().is_copyable(opt_this_ty.unwrap())?,
        //             this_contract,
        //             // raw_expr.range,
        //         )
        //         .unwrap();
        //         let this_qt = LazyExprQualifiedTy::new(this_qual, opt_this_ty.unwrap());
        //         LazyExprVariant::ThisField {
        //             field_ident,
        //             this_ty: opt_this_ty.unwrap(),
        //             this_binding: this_qt.binding(self.decl_db(), this_contract),
        //             field_binding: { field_qt.binding(self.decl_db(), field_contract) },
        //         }
        //     }
        //     RawExprVariant::FrameVariable { .. } => todo!(),
        // };
        // let qualified_ty = self.lazy_expr_qualified_ty(idx)?;
        // Ok(Arc::new(LazyExpr {
        //     range: raw_expr.range().clone(),
        //     qualified_ty,
        //     variant: kind,
        //     file: self.file(),
        //     contract: self.lazy_expr_contract(idx).unwrap(),
        //     instruction_id: Default::default(),
        //     implicit_conversion: ImplicitConversion::from_opt_expectation(
        //         opt_expectation,
        //         &qualified_ty,
        //     ),
        // }))
    }

    fn parse_opn(
        &mut self,
        idx: RawExprIdx,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
    ) -> SemanticResult<LazyExprVariant> {
        match opr {
            RawOpnVariant::Binary(opr) => self.parse_binary_opr(*opr, opds),
            RawOpnVariant::Prefix(_) => todo!(),
            RawOpnVariant::Suffix(opr) => self.parse_suffix_opr(opr, opds.start),
            RawOpnVariant::List(opr) => match opr {
                ListOpr::NewTuple => todo!(),
                ListOpr::NewVec => self.parse_new_vec_from_list(opds.clone()),
                ListOpr::NewDict => todo!(),
                ListOpr::FunctionCall => self.parse_function_call(opds),
                ListOpr::Index => self.parse_index(idx, opds.clone()),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
                ListOpr::MethodCall { ranged_ident, .. } => self.parse_method_call(
                    idx,
                    opds.start,
                    (opds.start + 1)..opds.end,
                    *ranged_ident,
                ),
            },
            RawOpnVariant::Field(field_ident) => {
                self.parse_field_access(field_ident.unwrap(), opds.start, idx)
            }
        }
    }

    fn parse_binary_opr(
        &mut self,
        opr: BinaryOpr,
        opds: &RawExprRange,
    ) -> SemanticResult<LazyExprVariant> {
        // let raw_opds = &self.arena()[raw_opds];
        let lopd = self.parse_lazy_expr(opds.start, None)?;
        let ropd = self.parse_lazy_expr(opds.start + 1, None)?;
        let opr = match opr {
            BinaryOpr::Pure(opr) => opr,
            BinaryOpr::Assign(_) => todo!(),
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => todo!(),
            BinaryOpr::As => todo!(),
        };
        Ok(LazyExprVariant::Opn {
            opn_kind: LazyOpnKind::Binary {
                opr,
                this: lopd.intrinsic_ty(),
            },
            opds: vec![lopd, ropd],
        })
    }

    fn infer_pure_binary_opr_type(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_ty: EntityRoutePtr,
        ropd_ty: EntityRoutePtr,
    ) -> SemanticResult<EntityRoutePtr> {
        match lopd_ty {
            EntityRoutePtr::Root(lopd_root_ty) => match ropd_ty {
                EntityRoutePtr::Root(ropd_root_ty) => self.infer_root_pure_binary_opr_type(
                    pure_binary_opr,
                    lopd_root_ty,
                    ropd_root_ty,
                ),
                EntityRoutePtr::Custom(_) => todo!(),
            },
            EntityRoutePtr::Custom(_) => {
                self.infer_custom_pure_binary_opr_type(pure_binary_opr, lopd_ty, ropd_ty)
            }
        }
    }

    fn infer_root_pure_binary_opr_type(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_root_ty: RootBuiltinIdentifier,
        ropd_root_ty: RootBuiltinIdentifier,
    ) -> SemanticResult<EntityRoutePtr> {
        Ok(match pure_binary_opr {
            PureBinaryOpr::Less
            | PureBinaryOpr::Leq
            | PureBinaryOpr::Greater
            | PureBinaryOpr::Geq => {
                if lopd_root_ty != ropd_root_ty {
                    err!("expect use of \"<, <=, >, >=\" on same types")
                }
                match lopd_root_ty {
                    RootBuiltinIdentifier::I32 | RootBuiltinIdentifier::F32 => (),
                    _ => err!("expect use of \"<, <=, >, >=\" on i32 or f32"),
                }
                RootBuiltinIdentifier::Bool
            }
            PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                if lopd_root_ty != ropd_root_ty {
                    err!("expect use of \"!=\" on same types")
                }
                RootBuiltinIdentifier::Bool
            }
            PureBinaryOpr::Shl => todo!(),
            PureBinaryOpr::Shr => todo!(),
            PureBinaryOpr::Add
            | PureBinaryOpr::Sub
            | PureBinaryOpr::Mul
            | PureBinaryOpr::Div
            | PureBinaryOpr::Power => {
                if lopd_root_ty != ropd_root_ty {
                    err!("expect use of \"+, -, *, /, **\" on same types")
                }
                match lopd_root_ty {
                    RootBuiltinIdentifier::I32 | RootBuiltinIdentifier::F32 => (),
                    _ => err!("expect use of \"+, -, *, /, **\" on i32 or f32"),
                }
                lopd_root_ty
            }
            PureBinaryOpr::And => todo!(),
            PureBinaryOpr::BitAnd => todo!(),
            PureBinaryOpr::Or => todo!(),
            PureBinaryOpr::BitXor => todo!(),
            PureBinaryOpr::BitOr => todo!(),
            PureBinaryOpr::RemEuclid => todo!(),
        }
        .into())
    }

    fn infer_custom_pure_binary_opr_type(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_ty: EntityRoutePtr,
        ropd_ty: EntityRoutePtr,
    ) -> SemanticResult<EntityRoutePtr> {
        match pure_binary_opr {
            PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                if lopd_ty.deref_route() == ropd_ty.deref_route() {
                    Ok(RootBuiltinIdentifier::Bool.into())
                } else {
                    todo!()
                }
            }
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
            PureBinaryOpr::Or => todo!(),
            PureBinaryOpr::Power => todo!(),
            PureBinaryOpr::Shl => todo!(),
            PureBinaryOpr::Shr => todo!(),
            PureBinaryOpr::Sub => todo!(),
        }
    }

    fn parse_suffix_opr(
        &mut self,
        opr: &RawSuffixOpr,
        opd: RawExprIdx,
    ) -> SemanticResult<LazyExprVariant> {
        let this = self.parse_lazy_expr(opd, None)?;
        Ok(match opr {
            RawSuffixOpr::Incr => panic!(),
            RawSuffixOpr::Decr => panic!(),
            RawSuffixOpr::AsTy(_) => todo!(),
            RawSuffixOpr::BePattern(raw_patt) => LazyExprVariant::BePattern {
                patt: Arc::new(PurePattern::from_raw(
                    self.db(),
                    raw_patt,
                    this.intrinsic_ty(),
                )),
                this,
            },
            RawSuffixOpr::Unveil => panic!(),
        })
    }

    fn parse_field_access(
        &mut self,
        field_ident: RangedCustomIdentifier,
        idx: RawExprIdx,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<LazyExprVariant> {
        todo!()
        // let this = self.parse_lazy_expr(idx, None)?;
        // let field_contract = self.lazy_expr_contract(raw_expr_idx).unwrap();
        // let field_qt = self.lazy_expr_qualified_ty(raw_expr_idx).unwrap();
        // Ok(LazyExprVariant::Opn {
        //     opn_kind: LazyOpnKind::Field {
        //         field_ident,
        //         field_binding: field_qt.binding(self.decl_db(), field_contract),
        //     },
        //     opds: vec![this],
        // })
    }

    fn parse_new_vec_from_list(&mut self, opds: RawExprRange) -> SemanticResult<LazyExprVariant> {
        let elements: Vec<_> = opds
            .map(|raw| self.parse_lazy_expr(raw, None))
            .collect::<SemanticResult<_>>()?;
        let opn_kind = LazyOpnKind::NewVecFromList;
        Ok(LazyExprVariant::Opn {
            opn_kind,
            opds: elements,
        })
    }

    fn parse_function_call(&mut self, opds: &RawExprRange) -> SemanticResult<LazyExprVariant> {
        todo!()
        // let call = &self.arena()[opds.start];
        // match call.variant {
        //     RawExprVariant::Entity { route, kind, .. } => {
        //         let arguments: Vec<_> = ((opds.start + 1)..opds.end)
        //             .map(|raw| self.parse_lazy_expr(raw, None))
        //             .collect::<SemanticResult<_>>()?;
        //         let opn_kind = match kind {
        //             EntityKind::Module => todo!(),
        //             EntityKind::Type(ty_kind) => match ty_kind {
        //                 TyKind::Enum => todo!(),
        //                 TyKind::Record => LazyOpnKind::RecordCall(RangedEntityRoute {
        //                     route,
        //                     range: call.range(),
        //                 }),
        //                 TyKind::Struct => LazyOpnKind::StructCall(RangedEntityRoute {
        //                     route,
        //                     range: call.range(),
        //                 }),
        //                 TyKind::Primitive => todo!(),
        //                 TyKind::Vec => todo!(),
        //                 TyKind::Array => todo!(),
        //                 TyKind::Slice => todo!(),
        //                 TyKind::CyclicSlice => todo!(),
        //                 TyKind::Tuple => todo!(),
        //                 TyKind::Mor => todo!(),
        //                 TyKind::ThickFp => todo!(),
        //                 TyKind::AssociatedAny => todo!(),
        //                 TyKind::TargetOutputAny => todo!(),
        //                 TyKind::ThisAny => todo!(),
        //                 TyKind::SpatialPlaceholderAny => todo!(),
        //                 TyKind::BoxAny => todo!(),
        //                 TyKind::HigherKind => todo!(),
        //                 TyKind::Ref => todo!(),
        //                 TyKind::Option => todo!(),
        //             },
        //             EntityKind::Trait => todo!(),
        //             EntityKind::Function {
        //                 requires_lazy: is_lazy,
        //             } => {
        //                 if is_lazy {
        //                     LazyOpnKind::FunctionModelCall(RangedEntityRoute {
        //                         route,
        //                         range: call.range(),
        //                     })
        //                 } else {
        //                     LazyOpnKind::FunctionRoutineCall(RangedEntityRoute {
        //                         route,
        //                         range: call.range(),
        //                     })
        //                 }
        //             }
        //             EntityKind::Feature => todo!(),
        //             EntityKind::EnumVariant => todo!(),
        //             EntityKind::Member(_) => todo!(),
        //             EntityKind::Main => panic!(),
        //         };
        //         Ok(LazyExprVariant::Opn {
        //             opn_kind,
        //             opds: arguments,
        //         })
        //     }
        //     RawExprVariant::Variable { .. } => todo!(),
        //     RawExprVariant::Unrecognized(_) => panic!(),
        //     RawExprVariant::PrimitiveLiteral(_) => todo!(),
        //     RawExprVariant::Bracketed(_) => todo!(),
        //     RawExprVariant::Opn { .. } => todo!(),
        //     RawExprVariant::Lambda(_, _) => todo!(),
        //     RawExprVariant::ThisValue { .. } => todo!(),
        //     RawExprVariant::ThisField { .. } => todo!(),
        //     RawExprVariant::FrameVariable { .. } => todo!(),
        // }
    }

    fn parse_method_call(
        &mut self,
        idx: RawExprIdx,
        this_idx: RawExprIdx,
        inputs: RawExprRange,
        method_ident: RangedCustomIdentifier,
    ) -> SemanticResult<LazyExprVariant> {
        todo!()
        // let this = self.parse_lazy_expr(this_idx, None)?;
        // let output_binding = {
        //     let output_contract = self.lazy_expr_contract(idx).unwrap();
        //     let output_qt = self.lazy_expr_qualified_ty(idx).unwrap();
        //     output_qt.binding(self.decl_db(), output_contract)
        // };
        // let inputs = inputs
        //     .map(|idx| self.parse_lazy_expr(idx, None))
        //     .collect::<SemanticResult<Vec<_>>>()?;
        // let mut opds = vec![this];
        // opds.extend(inputs);
        // Ok(LazyExprVariant::Opn {
        //     opn_kind: LazyOpnKind::MethodCall {
        //         method_ident,
        //         method_route: self
        //             .entity_route_sheet()
        //             .opt_method_call_route(this_idx)
        //             .unwrap()
        //             .unwrap(),
        //         output_binding,
        //     },
        //     opds,
        // })
    }

    fn parse_index(
        &mut self,
        idx: RawExprIdx,
        opds: RawExprRange,
    ) -> SemanticResult<LazyExprVariant> {
        todo!()
        // Ok(LazyExprVariant::Opn {
        //     opn_kind: LazyOpnKind::Index {
        //         element_binding: {
        //             let element_contract = self.lazy_expr_contract(idx).unwrap();
        //             let element_qt = self.lazy_expr_qualified_ty(idx).unwrap();
        //             element_qt.binding(self.decl_db(), element_contract)
        //         },
        //     },
        //     opds: opds
        //         .map(|raw| self.parse_lazy_expr(raw, None))
        //         .collect::<SemanticResult<_>>()?,
        // })
    }
}
