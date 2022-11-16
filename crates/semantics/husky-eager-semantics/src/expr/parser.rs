use husky_expr_syntax::*;
use husky_path::PathItd;
use husky_term::Ty;

use husky_text::RangedCustomIdentifier;

use crate::*;

use super::EagerOpnVariant;

pub trait ParseEagerExpr<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn file(&self) -> PathItd;
    fn target_entrance(&self) -> PathItd;

    fn parse_eager_expr(
        &mut self,
        _idx: RawExprIdx,
        _opt_expectation: Option<Ty>,
    ) -> SemanticResult<Arc<EagerExpr>> {
        todo!()
        /*
        let raw_expr = &self.arena()[idx];
        let variant = match raw_expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => {
                let variable_qt = self
                    .eager_variable_qualified_ty(varname.into(), init_range)
                    .unwrap();

                let contract = match self.eager_expr_contract(idx) {
                    Ok(contract) => contract,
                    Err(_) => {
                        p!(self.file(), idx, raw_expr);
                        p!(self.contract_sheet().eager_expr_contract_results);
                        panic!("what's the contract?");
                    }
                };
                EagerExprVariant::Variable {
                    varname,
                    binding: variable_qt.qual.binding(contract),
                }
            }
            RawExprVariant::FrameVariable { varname, .. } => EagerExprVariant::Variable {
                varname,
                binding: Binding::Copy,
            },
            RawExprVariant::Unrecognized(ident) => {
                err!(format!(
                    "unrecognized identifier {} at {}:{:?}",
                    ident,
                    self.file().to_str().unwrap(),
                    raw_expr.range()
                ))
            }
            RawExprVariant::Entity { route, kind } => match kind {
                EntityKind::Module => todo!(),
                EntityKind::EnumVariant => match route {
                    EntityRoutePtr::Root(RootBuiltinIdentifier::True) => {
                        EagerExprVariant::PrimitiveLiteral(RawLiteralData::Bool(true))
                    }
                    EntityRoutePtr::Root(RootBuiltinIdentifier::False) => {
                        EagerExprVariant::PrimitiveLiteral(RawLiteralData::Bool(false))
                    }
                    EntityRoutePtr::Custom(_) => EagerExprVariant::EnumKindLiteral(route),
                    _ => todo!(),
                },
                EntityKind::Type(_) => todo!(),
                EntityKind::Trait => todo!(),
                EntityKind::Function { .. } | EntityKind::Member(_) => {
                    EagerExprVariant::EntityThickFp { route }
                }
                EntityKind::Feature => EagerExprVariant::EntityFeature { route },
                EntityKind::Main => panic!(),
            },
            RawExprVariant::PrimitiveLiteral(value) => EagerExprVariant::PrimitiveLiteral(value),
            RawExprVariant::Bracketed(expr) => {
                EagerExprVariant::Bracketed(self.parse_eager_expr(expr, Default::default())?)
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
                ..
            } => self.parse_opn(opr, opds, idx)?,
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisValue {
                opt_this_ty,
                opt_this_liason,
            } => EagerExprVariant::ThisValue {
                binding: {
                    todo!()
                    // let this_contract = self.eager_expr_contract(idx).unwrap();
                    // let this_qual = EagerExprQualifier::parameter_use_eager_qualifier(
                    //     self.decl_db(),
                    //     opt_this_ty.unwrap(),
                    //     opt_this_liason.unwrap(),
                    //     this_contract,
                    //     raw_expr.range,
                    // )
                    // .unwrap();
                    // let this_qt = EagerExprQualifiedTy::new(this_qual, opt_this_ty.unwrap());
                    // this_qt.binding(self.decl_db(), this_contract)
                },
            },
            RawExprVariant::ThisField {
                field_ident,
                opt_this_ty,
                opt_this_liason,
                field_liason,
                opt_field_ty,
            } => {
                todo!()
                // let this_contract = EagerContract::member_self_eager_contract(
                //     field_liason,
                //     field_contract,
                //     opt_field_ty.unwrap().route,
                //     self.arena()[idx].range,
                // )?;
                // let this_qual = EagerExprQualifier::parameter_use_eager_qualifier(
                //     self.decl_db(),
                //     opt_this_ty.unwrap(),
                //     opt_this_liason.unwrap(),
                //     this_contract,
                //     raw_expr.range,
                // )
                // .unwrap();
                // let this_qt = EagerExprQualifiedTy::new(this_qual, opt_this_ty.unwrap());
                // let ty_decl = self.decl_db().ty_decl(opt_this_ty.unwrap()).unwrap();
                // EagerExprVariant::ThisField {
                //     field_ident,
                //     field_idx: ty_decl.field_idx(field_ident.ident).try_into().unwrap(),
                //     this_ty: opt_this_ty.unwrap(),
                //     this_binding: this_qt.binding(self.decl_db(), this_contract),
                //     field_binding: { field_qt.binding(self.decl_db(), field_contract) },
                // }
            }
        };
        let qualified_ty = self.eager_expr_qualified_ty(idx).unwrap();
        Ok(Arc::new(EagerExpr {
            range: raw_expr.range(),
            variant,
            file: self.file(),
            instruction_id: Default::default(),
            implicit_conversion: ImplicitConversion::from_opt_expectation(
                opt_expectation,
                &qualified_ty,
            ),
            qualified_ty,
            contract: self.eager_expr_contract(idx).unwrap(),
        })) */
    }

    fn parse_opn(
        &mut self,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
        idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        match opr {
            RawOpnVariant::Binary(opr) => self.parse_binary_opr(*opr, opds),
            RawOpnVariant::Prefix(opr) => self.parse_prefix_opr(*opr, opds),
            RawOpnVariant::Suffix(opr) => self.parse_suffix_opr(opr, opds),
            RawOpnVariant::CurlBracketed => todo!(),
            RawOpnVariant::List(opr) => match opr {
                ListOpr::NewTuple => todo!(),
                ListOpr::NewVec => self.parse_new_vec_from_list(idx, opds.clone()),
                ListOpr::NewDict => todo!(),
                ListOpr::FunctionCall => self.parse_function_call(opds.clone()),
                ListOpr::MethodCall { ranged_ident, .. } => self.parse_method_call(
                    opds.start,
                    (opds.start + 1)..opds.end,
                    *ranged_ident,
                    idx,
                ),
                ListOpr::Index => self.parse_element_access(opds.clone(), idx),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
            },
            RawOpnVariant::Field(field_ident) => {
                self.parse_field_access(field_ident.unwrap(), opds, idx)
            }
            RawOpnVariant::Abstraction => todo!(),
        }
    }

    fn parse_new_vec_from_list(
        &mut self,
        _idx: RawExprIdx,
        _opds: RawExprRange,
    ) -> SemanticResult<EagerExprVariant> {
        todo!()
        // let intrinsic_ty = self.eager_expr_qualified_ty(idx).unwrap().intrinsic_ty();
        // let elem_ty = intrinsic_ty.entity_route_argument(0);

        // let elements: Vec<_> = opds
        //     .map(|opd| self.parse_eager_expr(opd, Some(elem_ty)))
        //     .collect::<SemanticResult<_>>()?;
        // let opn_variant = EagerOpnVariant::NewVecFromList;
        // Ok(EagerExprVariant::Opn {
        //     opn_variant,
        //     opds: elements,
        // })
    }

    fn parse_binary_opr(
        &mut self,
        _opr: BinaryOpr,
        _raw_opds: &RawExprRange,
    ) -> SemanticResult<EagerExprVariant> {
        todo!()
        // let (lopd_opt_expectation, ropd_opt_expectation) = match opr {
        //     BinaryOpr::PureClosed(BinaryPureClosedOpr::Eq) => {
        //         let lopd_raw_ty = self.expr_raw_ty(raw_opds.start).unwrap();
        //         let ropd_raw_ty = self.expr_raw_ty(raw_opds.start + 1).unwrap();
        //         if lopd_raw_ty == ropd_raw_ty {
        //             (None, None)
        //         } else {
        //             assert_eq!(lopd_raw_ty.intrinsic(), ropd_raw_ty.intrinsic());
        //             if lopd_raw_ty.is_option() {
        //                 (None, Some(lopd_raw_ty))
        //             } else if ropd_raw_ty.is_option() {
        //                 (Some(ropd_raw_ty), None)
        //             } else {
        //                 todo!()
        //             }
        //         }
        //     }
        //     _ => (None, None),
        // };
        // let lopd = self.parse_eager_expr(raw_opds.start, lopd_opt_expectation)?;
        // let ropd = self.parse_eager_expr(raw_opds.start + 1, ropd_opt_expectation)?;
        // Ok(EagerExprVariant::Opn {
        //     opn_variant: EagerOpnVariant::Binary { opr },
        //     opds: vec![lopd, ropd],
        // })
    }

    fn parse_prefix_opr(
        &mut self,
        opr: PrefixOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<EagerExprVariant> {
        let opd_idx = raw_opds.start;
        let opd = self.parse_eager_expr(opd_idx, None)?;
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::Prefix { opr },
            opds: vec![opd],
        })
    }

    fn parse_suffix_opr(
        &mut self,
        raw_opr: &RawSuffixOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<EagerExprVariant> {
        let opd_idx = raw_opds.start;
        let opd = self.parse_eager_expr(opd_idx, None)?;
        let opr = match raw_opr {
            RawSuffixOpr::Incr => EagerSuffixOpr::Incr,
            RawSuffixOpr::Decr => EagerSuffixOpr::Decr,
            RawSuffixOpr::BePattern(_) => todo!(),
            RawSuffixOpr::Unveil => EagerSuffixOpr::Unveil,
        };
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::Suffix { opr },
            opds: vec![opd],
        })
    }

    fn parse_field_access(
        &mut self,
        _field_ident: RangedCustomIdentifier,
        _raw_opds: &RawExprRange,
        _raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        todo!()
        // let this_idx = raw_opds.start;
        // let this = self.parse_eager_expr(this_idx, None)?;
        // let this_ty_decl = self.decl_db().ty_decl(this.intrinsic_ty()).unwrap();
        // let field_decl = this_ty_decl.field_decl(field_ident).unwrap();
        // let field_liason = field_decl.modifier;
        // let field_contract = self.eager_expr_contract(raw_expr_idx).unwrap();
        // let field_qt = self.eager_expr_qualified_ty(raw_expr_idx).unwrap();
        // Ok(EagerExprVariant::Opn {
        //     opn_variant: EagerOpnVariant::Field {
        //         field_ident,
        //         this_ty: this.intrinsic_ty(),
        //         field_liason,
        //         field_binding: field_qt.binding(self.decl_db(), field_contract),
        //         field_kind: field_decl.field_kind,
        //     },
        //     opds: vec![this],
        // })
    }

    fn parse_function_call(&mut self, _opds: RawExprRange) -> SemanticResult<EagerExprVariant> {
        todo!()
        // let call = &self.arena()[opds.start];
        // let input_opd_idx_range = (opds.start + 1)..opds.end;
        // match call.variant {
        //     RawExprVariant::Entity {
        //         route,
        //         kind:
        //             EntityKind::Function {
        //                 requires_lazy: is_lazy,
        //             },
        //         ..
        //     } => {
        //         if is_lazy {
        //             todo!()
        //         }
        //         let arguments: Vec<_> = input_opd_idx_range
        //             .clone()
        //             .map(|raw| self.parse_eager_expr(raw, None))
        //             .collect::<SemanticResult<_>>()?;
        //         Ok(EagerExprVariant::Opn {
        //             opn_variant: EagerOpnVariant::RoutineCall(Ty {
        //                 route,
        //                 range: call.range(),
        //             }),
        //             opds: arguments,
        //         })
        //     }
        //     RawExprVariant::Entity {
        //         route,
        //         kind: EntityKind::Type(_),
        //         ..
        //     } => {
        //         let arguments: Vec<_> = input_opd_idx_range
        //             .map(|raw| self.parse_eager_expr(raw, None))
        //             .collect::<SemanticResult<_>>()?;
        //         Ok(EagerExprVariant::Opn {
        //             opn_variant: EagerOpnVariant::TypeCall {
        //                 ranged_ty: Ty {
        //                     route,
        //                     range: call.range(),
        //                 },
        //                 ty_decl: derived_unwrap!(self.decl_db().ty_decl(route)),
        //             },
        //             opds: arguments,
        //         })
        //     }
        //     RawExprVariant::Entity { .. } => todo!(),
        //     _ => Ok(EagerExprVariant::Opn {
        //         opn_variant: EagerOpnVariant::ValueCall,
        //         opds: opds
        //             .map(|raw| self.parse_eager_expr(raw, None))
        //             .collect::<SemanticResult<_>>()?,
        //     }),
        // }
    }

    fn parse_method_call(
        &mut self,
        _this_idx: RawExprIdx,
        _arguments: RawExprRange,
        _method_ident: RangedCustomIdentifier,
        _idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        todo!()
        // let this = self.parse_eager_expr(this_idx, None)?;
        // let this_ty_decl = self.decl_db().ty_decl(this.intrinsic_ty()).unwrap();
        // let output_binding = {
        //     let output_contract = self.eager_expr_contract(idx).unwrap();
        //     let output_qt = self.eager_expr_qualified_ty(idx).unwrap();
        //     output_qt.binding(self.decl_db(), output_contract)
        // };
        // let method_decl = self.method_call_form_decl(this_idx).unwrap();
        // let opds = {
        //     let mut opds = vec![this];
        //     msg_once!("keyword parameters and variadics");
        //     let arguments =
        //         std::iter::zip(arguments.into_iter(), method_decl.primary_parameters.iter())
        //             .map(|(idx, primary_parameter)| {
        //                 self.parse_eager_expr(idx, Some(primary_parameter.ty()))
        //             })
        //             .collect::<SemanticResult<Vec<_>>>()?;
        //     opds.extend(arguments);
        //     opds
        // };
        // Ok(EagerExprVariant::Opn {
        //     opn_variant: EagerOpnVariant::MethodCall {
        //         method_ident,
        //         this_ty_decl,
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

    fn parse_element_access(
        &mut self,
        _opds: RawExprRange,
        _idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        todo!()
        // Ok(EagerExprVariant::Opn {
        //     opn_variant: EagerOpnVariant::Index {
        //         element_binding: {
        //             let element_qt = self.eager_expr_qualified_ty(idx).unwrap();
        //             let contract = self.eager_expr_contract(idx).unwrap();
        //             element_qt.binding(self.decl_db(), contract)
        //         },
        //     },
        //     opds: opds
        //         .map(|raw| self.parse_eager_expr(raw, None))
        //         .collect::<SemanticResult<_>>()?,
        // })
    }
}
