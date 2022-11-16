use std::sync::Arc;

use crate::*;

use husky_expr_syntax::*;
use husky_file::PathItd;
use husky_term::Ty;

use husky_term_infer::TermInferDb;
use husky_text::RangedCustomIdentifier;
use husky_word::RootBuiltinIdentifier;

use super::*;
use husky_semantics_error::*;

// todo: opt_expectation

pub trait LazyExprParser<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn file(&self) -> PathItd;
    fn db(&self) -> &dyn TermInferDb;

    fn parse_lazy_expr(
        &mut self,
        _idx: RawExprIdx,
        _opt_expectation: Option<Ty>,
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
        //         route: entity_path,
        //         kind,
        //         ..
        //     } => match kind {
        //         EntityKind::Module => todo!(),
        //         EntityKind::EnumVariant => match entity_path {
        //             EntityRoutePtr::Root(RootBuiltinIdentifier::True) => {
        //                 LazyExprVariant::PrimitiveLiteral(RawLiteralData::Bool(true))
        //             }
        //             EntityRoutePtr::Root(RootBuiltinIdentifier::False) => {
        //                 LazyExprVariant::PrimitiveLiteral(RawLiteralData::Bool(false))
        //             }
        //             EntityRoutePtr::Custom(_) => LazyExprVariant::EnumLiteral { entity_path },
        //             _ => todo!(),
        //         },
        //         EntityKind::Type(_) => todo!(),
        //         EntityKind::Trait => todo!(),
        //         EntityKind::Function { .. } => {
        //             todo!()
        //         }
        //         EntityKind::Feature => LazyExprVariant::EntityFeature { entity_path },
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
            RawOpnVariant::CurlBracketed => todo!(),
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
            RawOpnVariant::Abstraction => todo!(),
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
            BinaryOpr::PureClosed(opr) => opr,
            BinaryOpr::Assign(_) => todo!(),
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => todo!(),
            BinaryOpr::As => todo!(),
            BinaryOpr::Comparison(_) => todo!(),
            BinaryOpr::ShortcuitLogic(_) => todo!(),
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
        pure_binary_opr: BinaryPureClosedOpr,
        lopd_ty: Ty,
        ropd_ty: Ty,
    ) -> SemanticResult<Ty> {
        todo!()
        // match lopd_ty {
        //     Ty::Root(lopd_root_ty) => match ropd_ty {
        //         Ty::Root(ropd_root_ty) => self.infer_root_pure_binary_opr_type(
        //             pure_binary_opr,
        //             lopd_root_ty,
        //             ropd_root_ty,
        //         ),
        //         Ty::Custom(_) => todo!(),
        //     },
        //     Ty::Custom(_) => {
        //         self.infer_custom_pure_binary_opr_type(pure_binary_opr, lopd_ty, ropd_ty)
        //     }
        // }
    }

    fn infer_root_pure_binary_opr_type(
        &self,
        ord_cmp_opr: BinaryComparisonOpr,
        lopd_root_ty: RootBuiltinIdentifier,
        ropd_root_ty: RootBuiltinIdentifier,
    ) -> SemanticResult<Ty> {
        todo!()
        // Ok(match ord_cmp_opr {
        //     BinaryComparisonOpr::Less
        //     | BinaryComparisonOpr::Leq
        //     | BinaryComparisonOpr::Greater
        //     | BinaryComparisonOpr::Geq => {
        //         if lopd_root_ty != ropd_root_ty {
        //             err!("expect use of \"<, <=, >, >=\" on same types")
        //         }
        //         match lopd_root_ty {
        //             RootBuiltinIdentifier::I32 | RootBuiltinIdentifier::F32 => (),
        //             _ => err!("expect use of \"<, <=, >, >=\" on i32 or f32"),
        //         }
        //         RootBuiltinIdentifier::Bool
        //     }
        //     BinaryComparisonOpr::Eq | BinaryComparisonOpr::Neq => {
        //         if lopd_root_ty != ropd_root_ty {
        //             err!("expect use of \"!=\" on same types")
        //         }
        //         RootBuiltinIdentifier::Bool
        //     }
        //     BinaryPureClosedOpr::Shl => todo!(),
        //     BinaryPureClosedOpr::Shr => todo!(),
        //     BinaryPureClosedOpr::Add
        //     | BinaryPureClosedOpr::Sub
        //     | BinaryPureClosedOpr::Mul
        //     | BinaryPureClosedOpr::Div
        //     | BinaryPureClosedOpr::Power => {
        //         if lopd_root_ty != ropd_root_ty {
        //             err!("expect use of \"+, -, *, /, **\" on same types")
        //         }
        //         match lopd_root_ty {
        //             RootBuiltinIdentifier::I32 | RootBuiltinIdentifier::F32 => (),
        //             _ => err!("expect use of \"+, -, *, /, **\" on i32 or f32"),
        //         }
        //         lopd_root_ty
        //     }
        //     BinaryShortcuitLogicOpr::And => todo!(),
        //     BinaryPureClosedOpr::BitAnd => todo!(),
        //     BinaryShortcuitLogicOpr::Or => todo!(),
        //     BinaryPureClosedOpr::BitXor => todo!(),
        //     BinaryPureClosedOpr::BitOr => todo!(),
        //     BinaryPureClosedOpr::RemEuclid => todo!(),
        // }
        // .into())
    }

    fn infer_custom_pure_binary_opr_type(
        &self,
        pure_binary_opr: BinaryPureClosedOpr,
        lopd_ty: Ty,
        ropd_ty: Ty,
    ) -> SemanticResult<Ty> {
        todo!()
        // match pure_binary_opr {
        //     BinaryComparisonOpr::Eq | BinaryComparisonOpr::Neq => {
        //         if lopd_ty.deref_route() == ropd_ty.deref_route() {
        //             Ok(RootBuiltinIdentifier::Bool.into())
        //         } else {
        //             todo!()
        //         }
        //     }
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
        //     BinaryShortcuitLogicOpr::Or => todo!(),
        //     BinaryPureClosedOpr::Power => todo!(),
        //     BinaryPureClosedOpr::Shl => todo!(),
        //     BinaryPureClosedOpr::Shr => todo!(),
        //     BinaryPureClosedOpr::Sub => todo!(),
        // }
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
        _field_ident: RangedCustomIdentifier,
        _idx: RawExprIdx,
        _raw_expr_idx: RawExprIdx,
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

    fn parse_function_call(&mut self, _opds: &RawExprRange) -> SemanticResult<LazyExprVariant> {
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
        //                 TyKind::Record => LazyOpnKind::RecordCall(Ty {
        //                     route,
        //                     range: call.range(),
        //                 }),
        //                 TyKind::Struct => LazyOpnKind::StructCall(Ty {
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
        //                     LazyOpnKind::FunctionModelCall(Ty {
        //                         route,
        //                         range: call.range(),
        //                     })
        //                 } else {
        //                     LazyOpnKind::FunctionRoutineCall(Ty {
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
        _idx: RawExprIdx,
        _this_idx: RawExprIdx,
        _inputs: RawExprRange,
        _method_ident: RangedCustomIdentifier,
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
        _idx: RawExprIdx,
        _opds: RawExprRange,
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
