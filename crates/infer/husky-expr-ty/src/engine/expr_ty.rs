mod application;
mod binary;
mod box_list;
mod literal;
mod prefix;
mod ritchie_call_ty;
mod suffix;

use super::*;
use husky_opn_syntax::*;

pub(crate) enum ExprTypeResolveProgress<E: ExpectLocalTerm> {
    Unresolved,
    Outcome(E::Outcome),
    ResolvedErr,
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_ty_resolved(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<ReducedTerm> {
        let ty = self.infer_new_expr_ty(expr_idx, expr_ty_expectation, local_term_region)?;
        match ty {
            LocalTerm::Resolved(ty) => Some(ty),
            LocalTerm::Unresolved(ty) => self.resolve_term(ty, local_term_region),
        }
    }

    pub(super) fn infer_new_expr_ty<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<LocalTerm> {
        let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation, local_term_region);
        let expectation_idx = match ty_result {
            Ok(ty) => {
                self.add_expectation_rule(expr_idx, ty, expr_ty_expectation, local_term_region)
            }
            Err(_) => Default::default(),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak, local_term_region);
        match expectation_idx.into_option() {
            Some(expectation_idx) => local_term_region[expectation_idx]
                .resolve_progress()
                .resolved_ok::<E::Outcome>()
                .map(|ok| ok.destination()),
            None => None,
        }
    }

    pub(super) fn infer_new_expr_ty_discarded<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
        local_term_region: &mut LocalTermRegion,
    ) {
        let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation, local_term_region);
        let expectation_idx = match ty_result {
            Ok(ty) => {
                self.add_expectation_rule(expr_idx, ty, expr_ty_expectation, local_term_region)
            }
            Err(_) => Default::default(),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
    }

    #[inline(always)]
    pub(super) fn infer_new_expr_ty_with_expectation_returned<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
        local_term_region: &mut LocalTermRegion,
    ) -> (OptionLocalTermExpectationIdx, Option<E::Outcome>)
    where
        E::Outcome: Clone,
    {
        let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation, local_term_region);
        let expectation_idx = match ty_result {
            Ok(ty) => {
                self.add_expectation_rule(expr_idx, ty, expr_ty_expectation, local_term_region)
            }
            Err(_) => Default::default(),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak, local_term_region);
        let resolved_ok = match expectation_idx.into_option() {
            Some(expectation_idx) => local_term_region[expectation_idx]
                .resolve_progress()
                .resolved_ok()
                .cloned(),
            None => None,
        };
        (expectation_idx, resolved_ok)
    }

    fn save_new_expr_ty(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: &impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<LocalTerm> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx) => self.calc_literal(
                expr_idx,
                literal_token_idx,
                expr_ty_expectation,
                local_term_region,
            ),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => {
                    match husky_ty::entity_path_ty(
                        self.db,
                        expr_ty_expectation.entity_path_ty_expectation(),
                        entity_path,
                    ) {
                        Ok(ty) => Ok(ty.into()),
                        Err(_) => Err(DerivedExprTypeError::EntityTypeError.into()),
                    }
                }
                None => Err(DerivedExprTypeError::EntityPathError.into()),
            },
            Expr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => match self.inherited_symbol_tys.get(inherited_symbol_idx) {
                Some(ty) => Ok((*ty).into()),
                None => Err(DerivedExprTypeError::InheritedSymbolTypeError.into()),
            },
            Expr::CurrentSymbol {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => self
                .current_symbol_tys
                .get(current_symbol_idx)
                .copied()
                .ok_or(DerivedExprTypeError::CurrentSymbolTypeError.into()),
            Expr::FrameVarDecl {
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => match self.self_ty {
                Some(_) => todo!(),
                None => Err(DerivedExprTypeError::SelfTypeNotInferredForSelfValue.into()),
            },
            Expr::BinaryOpn {
                lopd,
                opr,
                ropd,
                opr_token_idx,
                ..
            } => self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd, local_term_region),
            Expr::Be {
                src, ref target, ..
            } => {
                match self.infer_new_expr_ty(src, ExpectAnyOriginal, local_term_region) {
                    Some(src_ty) => match target {
                        Ok(target) => todo!(),
                        Err(_) => (),
                    },
                    None => (),
                };
                Ok(self.reduced_term_menu.bool().into())
            }
            Expr::PrefixOpn { opr, opd, .. } => self.calc_prefix_ty(opr, opd, local_term_region),
            Expr::SuffixOpn { opd, opr, .. } => self.calc_suffix_ty(opd, opr, local_term_region),
            Expr::ApplicationOrRitchieCall {
                function,
                ref implicit_arguments,
                ref items,
                ..
            } => {
                let (expectation_idx, expectation_ok) = self
                    .infer_new_expr_ty_with_expectation_returned(
                        function,
                        ExpectEqsFunctionType,
                        local_term_region,
                    );
                if let Some(implicit_arguments) = implicit_arguments {
                    todo!()
                }
                match expectation_ok {
                    Some(expectation_ok) => {
                        match expectation_ok.variant() {
                            ExpectEqsFunctionTypeOkVariant::Ritchie {
                                ritchie_kind,
                                parameter_liasoned_tys,
                            } => self.calc_ritchie_call_arguments_ty(
                                *ritchie_kind,
                                parameter_liasoned_tys.to_vec(),
                                *items,
                                local_term_region,
                            ),
                            ExpectEqsFunctionTypeOkVariant::Curry {} => todo!(),
                        }
                        Ok(expectation_ok.return_ty())
                    }
                    None => {
                        for item in items {
                            self.infer_new_expr_ty(item, ExpectAnyDerived, local_term_region);
                        }
                        Err(
                            DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                                .into(),
                        )
                    }
                }
                // match function_ty {
                //     Some(function_ty) => {
                //         match function_ty {
                //             LocalTerm::Resolved(function_ty) => match function_ty.term() {
                //                 Term::Category(_) => {
                //                     let Some(ty_term) = self.infer_new_expr_term(function)
                //                     else {
                //                         self.infer_new_expr_ty(argument, todo!(),local_term_region);
                //                         return Err(todo!())
                //                     };
                //                     match ty_term {
                //                         LocalTerm::Resolved(ty_term) => {
                //                             let ty_call_ty = match self.db.ty_call_ty(
                //                                 ty_term,
                //                                 self.toolchain,
                //                                 self.reduced_term_menu,
                //                             ) {
                //                                 Ok(ty_call_ty) => ty_call_ty,
                //                                 Err(error) => {
                //                                     self.infer_new_expr_ty(
                //                                         argument,
                //                                         todo!(),
                //                                         local_term_region,
                //                                     );
                //                                     return Err(match error {
                //                                         TypeError::Original(error) => OriginalExprTypeError::TypeCallTypeError(error).into(),
                //                                         TypeError::Derived(error) => DerivedExprTypeError::TypeCallTypeError(error).into(),
                //                                     });
                //                                 }
                //                             };
                //                             todo!()
                //                         }
                //                         LocalTerm::Unresolved(_) => todo!(),
                //                     }
                //                 }
                //                 Term::Ritchie(_) => self.calc_ritchie_call_ty(
                //                     Some(function_ty.into()),
                //                     None,
                //                     ExprIdxRange::new_single(argument),
                //                     local_term_region,
                //                 ),
                //                 _ => todo!(),
                //             },
                //             LocalTerm::Unresolved(_) => todo!(),
                //         }
                //     }
                //     None => {
                //         self.infer_new_expr_ty(
                //             argument,
                //             todo!(),
                //             local_term_region,
                //         );
                //         Err(DerivedExprTypeError::FunctionTypeNotInferredInApplicationOrFunctionCall.into())
                //     }
                // }
            }
            Expr::Field {
                owner, ident_token, ..
            } => {
                if let Some(owner_ty) =
                    self.infer_new_expr_ty(owner, ExpectAnyOriginal, local_term_region)
                {
                    match owner_ty {
                        LocalTerm::Resolved(owner_ty) => {
                            let field_ty = self.db.field_ty(owner_ty, ident_token.ident());
                            match field_ty {
                                Ok(_) => todo!(),
                                Err(e) => Err(match e {
                                    TypeError::Original(error) => {
                                        OriginalExprTypeError::FieldTypeError(error).into()
                                    }
                                    TypeError::Derived(error) => {
                                        DerivedExprTypeError::FieldTypeError(error).into()
                                    }
                                }),
                            }
                        }
                        LocalTerm::Unresolved(_) => todo!(),
                    }
                } else {
                    Err(DerivedExprTypeError::FieldOwnerTypeNotInferred.into())
                }
            }
            Expr::MethodCall {
                self_argument,
                ident_token,
                ref implicit_arguments,
                nonself_arguments,
                ..
            } => {
                let Some(self_expr_ty) =
                    self.infer_new_expr_ty_resolved( self_argument, ExpectAnyOriginal,local_term_region)
                    else {
                        if let Some(implicit_arguments) = implicit_arguments {
                            todo!()
                        }
                        for argument in nonself_arguments {
                            self.infer_new_expr_ty(argument, ExpectAnyDerived,local_term_region);
                        }
                        return Err(DerivedExprTypeError::MethodOwnerTypeNotInferred.into())
                    };
                let method_ty = match self.db.ty_method_ty(self_expr_ty, ident_token.ident()) {
                    Ok(_) => todo!(),
                    Err(error) => {
                        return Err(match error {
                            TypeError::Original(error) => {
                                OriginalExprTypeError::TypeMethodTypeError(error).into()
                            }
                            TypeError::Derived(error) => {
                                DerivedExprTypeError::TypeMethodTypeError(error).into()
                            }
                        })
                    }
                };
                self.calc_ritchie_call_arguments_ty(
                    method_ty,
                    todo!(),
                    nonself_arguments,
                    local_term_region,
                );
                todo!()
            }
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => {
                self.calc_application(function, argument, local_term_region)
            }
            Expr::Bracketed { item, .. } => self
                .infer_new_expr_ty(item, expr_ty_expectation.clone(), local_term_region)
                .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
            Expr::NewTuple { items, .. } => todo!(),
            Expr::NewBoxList { caller, items, .. } => {
                self.calc_new_box_list(expr_idx, items, local_term_region)
            }
            Expr::BoxColon { caller, .. } => todo!(),
            Expr::Block { stmts } => self
                .infer_new_block(stmts, expr_ty_expectation.clone(), local_term_region)
                .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }
}
