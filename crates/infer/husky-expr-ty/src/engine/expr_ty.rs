mod application;
mod binary;
mod box_list;
mod literal;
mod prefix;
mod ritchie_call;
mod suffix;

use super::*;
use husky_opn_syntax::*;

pub(crate) enum ExprTypeResolveProgress<E: ExpectLocalTerm> {
    Unresolved,
    ResolvedOk(E::ResolvedOk),
    ResolvedErr,
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_ty_resolved(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: impl ExpectLocalTerm,
    ) -> Option<ReducedTerm> {
        let ty = self.infer_new_expr_ty(expr_idx, expr_ty_expectation)?;
        match ty {
            LocalTerm::Resolved(ty) => Some(ty),
            LocalTerm::Unresolved(ty) => self.resolve_term(ty),
        }
    }

    pub(super) fn infer_new_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: impl ExpectLocalTerm,
    ) -> Option<LocalTerm> {
        self.infer_new_expr_ty_with_expectation_rule(expr_idx, expr_ty_expectation)
            .1
            .map(|resolved_ok| resolved_ok.destination())
    }

    #[inline(always)]
    pub(super) fn infer_new_expr_ty_with_expectation_rule<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
    ) -> (OptionLocalTermExpectationIdx, Option<&E::ResolvedOk>) {
        let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation);
        let expectation_idx = match ty_result {
            Ok(ty) => self.add_expectation_rule(expr_idx, ty, expr_ty_expectation),
            Err(_) => Default::default(),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak);
        let resolved_ok = match expectation_idx.into_option() {
            Some(expectation_idx) => self.local_term_table[expectation_idx]
                .resolve_progress()
                .resolved_ok(),
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
    ) -> ExprTypeResult<LocalTerm> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx) => {
                self.calc_literal(expr_idx, literal_token_idx, expr_ty_expectation)
            }
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => match self.db.entity_ty(entity_path) {
                    Ok(ty) => Ok(ty.into()),
                    Err(_) => Err(DerivedExprTypeError::EntityTypeError.into()),
                },
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
            } => self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd),
            Expr::Be {
                src, ref target, ..
            } => {
                match self.infer_new_expr_ty(src, ExpectInsSort::default()) {
                    Some(src_ty) => match target {
                        Ok(target) => todo!(),
                        Err(_) => (),
                    },
                    None => (),
                };
                Ok(self.reduced_term_menu.bool().into())
            }
            Expr::PrefixOpn { opr, opd, .. } => self.calc_prefix_ty(opr, opd),
            Expr::SuffixOpn { opd, opr, .. } => self.calc_suffix_ty(opd, opr),
            Expr::ApplicationOrRitchieCall {
                function, argument, ..
            } => {
                let function_ty = self.infer_new_expr_ty(function, ExpectInsSort::default());
                match function_ty {
                    Some(function_ty) => {
                        match function_ty {
                            LocalTerm::Resolved(function_ty) => match function_ty.term() {
                                Term::Category(_) => {
                                    let Some(ty_term) = self.infer_new_expr_term(function)
                                    else {
                                        self.infer_new_expr_ty(argument, ExpectInsSort::default());
                                        return Err(todo!())
                                    };
                                    match ty_term {
                                        LocalTerm::Resolved(ty_term) => {
                                            let ty_call_ty = match self.db.ty_call_ty(
                                                ty_term,
                                                self.toolchain,
                                                self.reduced_term_menu,
                                            ) {
                                                Ok(ty_call_ty) => ty_call_ty,
                                                Err(error) => {
                                                    self.infer_new_expr_ty(
                                                        argument,
                                                        ExpectInsSort::default(),
                                                    );
                                                    return Err(match error {
                                                        TypeError::Original(error) => OriginalExprTypeError::TypeCallTypeError(error).into(),
                                                        TypeError::Derived(error) => DerivedExprTypeError::TypeCallTypeError(error).into(),
                                                    });
                                                }
                                            };
                                            todo!()
                                        }
                                        LocalTerm::Unresolved(_) => todo!(),
                                    }
                                }
                                Term::Ritchie(_) => self.calc_ritchie_call_ty(
                                    Some(function_ty.into()),
                                    None,
                                    ExprIdxRange::new_single(argument),
                                ),
                                _ => todo!(),
                            },
                            LocalTerm::Unresolved(_) => todo!(),
                        }
                    }
                    None => {
                        self.infer_new_expr_ty(argument, ExpectInsSort::default());
                        Err(DerivedExprTypeError::FunctionTypeNotInferredInApplicationOrFunctionCall.into())
                    }
                }
            }
            Expr::RitchieCall {
                function,
                ref implicit_arguments,
                arguments,
                ..
            } => {
                let function_ty = self.infer_new_expr_ty(function, ExpectEqsRitchieCallType);
                self.calc_ritchie_call_ty(function_ty, implicit_arguments.as_ref(), arguments)
            }
            Expr::Field {
                owner, ident_token, ..
            } => {
                if let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectInsSort::default()) {
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
                    self.infer_new_expr_ty_resolved( self_argument, ExpectInsSort::default(),)
                    else {
                        if let Some(implicit_arguments) = implicit_arguments {
                            todo!()
                        }
                        for argument in nonself_arguments {
                            self.infer_new_expr_ty(argument, ExpectInsSort::default());
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
                self.calc_ritchie_call_ty(method_ty, implicit_arguments.as_ref(), nonself_arguments)
            }
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => self.calc_application(function, argument),
            Expr::Bracketed { item, .. } => self
                .infer_new_expr_ty(item, expr_ty_expectation.clone())
                .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
            Expr::NewTuple { items, .. } => todo!(),
            Expr::NewBoxList { caller, items, .. } => self.calc_new_box_list(expr_idx, items),
            Expr::BoxColon { caller, .. } => todo!(),
            Expr::Block { stmts } => self
                .infer_new_block(stmts, expr_ty_expectation.clone())
                .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }
}
