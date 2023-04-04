mod binary;
mod box_list;
mod current_symbol;
mod explicit_application;
mod field;
mod literal;
mod method;
mod prefix;
mod ritchie_call_ty;
mod suffix;
mod utils;

use super::*;
use husky_opn_syntax::*;
use husky_term::has_ty::HasTypeGivenDisambiguation;
use husky_ty_expectation::TypePathDisambiguation;

pub(crate) enum ExprTypeResolveProgress<E: ExpectLocalTerm> {
    Unresolved,
    Outcome(E::Outcome),
    ResolvedErr,
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_ty<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E, 
    ) -> Option<LocalTerm> {
        self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation,  );
        self.expr_ty_infos[expr_idx].ty().ok()
    }

    pub(super) fn infer_new_expr_ty_discarded<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E, 
    ) {
        self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation,  );
    }

    #[inline(always)]
    pub(super) fn infer_new_expr_ty_for_outcome<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E, 
    ) -> Option<E::Outcome>
    where
        E::Outcome: Clone,
    {
        let expectation_idx =
            self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation,  );
        self.local_term_region.resolve_as_much_as_possible(self.db(), LocalTermResolveLevel::Weak);
        let outcome = match expectation_idx.into_option() {
            Some(expectation_idx) => self.local_term_region[expectation_idx]
                .resolve_progress()
                .outcome::<E>()
                .cloned(),
            None => None,
        };
        outcome
    }

    #[inline(always)]
    fn infer_new_expr_ty_aux<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E, 
    ) -> OptionLocalTermExpectationIdx {
        let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation,  );
        let expectation_idx = match ty_result {
            Ok((_, Ok(ty))) => {
                self.local_term_region.add_expectation_rule(expr_idx, ty, expr_ty_expectation)
            }
            _ => Default::default(),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
        self.local_term_region.resolve_as_much_as_possible(self.db(), LocalTermResolveLevel::Weak);
        expectation_idx
    }

    fn save_new_expr_ty(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: &impl ExpectLocalTerm, 
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx) => Ok((
                ExprDisambiguation::Trivial,
                self.calc_literal_expr_ty(
                    expr_idx,
                    literal_token_idx,
                    expr_ty_expectation, 
                ),
            )),
            Expr::EntityPath {
                entity_path_expr,
                path,
            } => self.calc_entity_path_expr_ty(path, expr_ty_expectation, ),
            Expr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                match self.inherited_symbol_tys.get(inherited_symbol_idx) {
                    Some(ty) => Ok((*ty).into()),
                    None => Err(DerivedExprTypeError::InheritedSymbolTypeError.into()),
                },
            )),
            Expr::CurrentSymbol {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_current_symbol_ty(
                    expr_idx,
                    expr_ty_expectation,
                    current_symbol_idx,
                    current_symbol_kind, 
                ),
            )),
            Expr::FrameVarDecl {
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            Expr::SelfType(_) => Ok((
                ExprDisambiguation::Trivial,
                match self.self_ty {
                    Some(self_ty) => Ok(self_ty.ty(self.db, self.toolchain)?.into()), // todo: impl binding
                    None => Err(DerivedExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            Expr::SelfValue(_) => Ok((
                ExprDisambiguation::Trivial,
                match self.self_ty {
                    Some(self_ty) => Ok(self_ty.into()), // todo: impl binding
                    None => Err(DerivedExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            Expr::Binary {
                lopd,
                opr,
                ropd,
                opr_token_idx,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd,  ),
            )),
            Expr::Be {
                src, ref target, ..
            } => {
                match self.infer_new_expr_ty(src, ExpectAnyOriginal,  ) {
                    Some(src_ty) => match target {
                        Ok(target) => self.infer_pattern_and_symbols_ty(
                            target.pattern_expr(),
                            src_ty,
                            target.variables(),
                        ),
                        Err(_) => (),
                    },
                    None => (),
                };
                Ok((
                    ExprDisambiguation::Trivial,
                    Ok(self.term_menu.bool_ty_ontology().into()),
                ))
            }
            Expr::Prefix { opr, opd, .. } => self.calc_prefix_expr_ty(
                opr,
                opd,
                expr_ty_expectation
                    .final_destination(self.db, self.local_term_region.unresolved_terms()), 
            ),
            Expr::Suffix { opd, opr, .. } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_suffix_expr_ty(opd, opr,  ),
            )),
            Expr::ExplicitApplicationOrRitchieCall {
                function,
                ref implicit_arguments,
                ref items,
                ..
            } => self.calc_explicit_application_or_ritchie_call_expr_ty(
                expr_idx,
                function,
                expr_ty_expectation, 
                implicit_arguments,
                items,
            ),
            Expr::Field {
                owner, ident_token, ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_field_expr_ty(owner, ident_token,  ),
            )),
            Expr::MethodCall {
                self_argument,
                ident_token,
                ref implicit_arguments,
                nonself_arguments,
                ..
            } => self.calc_method_expr_ty(
                expr_idx,
                self_argument,
                ident_token,
                implicit_arguments.as_ref(),
                nonself_arguments,  
            ),
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::ExplicitApplication { function, argument } => self.calc_explicit_application(
                function,
                argument,
                expr_ty_expectation
                    .final_destination(self.db(), self.local_term_region.unresolved_terms()), 
            ),
            Expr::Bracketed { item, .. } => Ok((
                ExprDisambiguation::Trivial,
                self.infer_new_expr_ty(item, expr_ty_expectation.clone(),  )
                    .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
            )),
            Expr::Unit { .. } => Ok((
                ExprDisambiguation::Trivial,
                Ok(self.term_menu.unit().into()),
            )),
            Expr::NewTuple { items, .. } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner,
                items: indices,
                ..
            } => self.calc_index_or_compose_with_list_expr_ty(
                expr_idx,
                owner,
                indices, 
            ),
            Expr::List { items, .. } => {
                Ok(
                    match expr_ty_expectation
                        .disambiguate_ty_path(self.db(),self. local_term_region.unresolved_terms())
                    {
                        TypePathDisambiguation::Ontology => {
                            // ad hoc, assume universe is 1
                            match items.len() {
                                0 => (
                                    ListExprDisambiguation::ListFunctor.into(),
                                    Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                                ),
                                1 => (
                                    ListExprDisambiguation::ArrayFunctor.into(),
                                    Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                                ),
                                _ => {
                                    print_debug_expr!(self, expr_idx);
                                    todo!()
                                }
                            }
                        }
                        TypePathDisambiguation::Constructor => {
                            let element_ty: LocalTerm = match expr_ty_expectation
                                .destination_pattern(
                                    self.db(),
                              self.      local_term_region.unresolved_terms(),
                                ) {
                                Some(ty_pattern) => match ty_pattern {
                                    LocalTermPattern::Literal(_) => todo!(),
                                    LocalTermPattern::TypeOntology {
                                        path,
                                        refined_path,
                                        argument_tys: arguments,
                                    } => match refined_path {
                                        Right(PreludeTypePath::List) => {
                                            assert_eq!(arguments.len(), 1);
                                            arguments[0]
                                        }
                                        Right(PreludeTypePath::Array) => todo!(),
                                        _ => todo!(),
                                    },
                                    LocalTermPattern::Curry {
                                        curry_kind,
                                        variance,
                                        parameter_variable: parameter_symbol,
                                        parameter_ty,
                                        return_ty,
                                    } => todo!(),
                                    LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
                                    LocalTermPattern::Category(_) => todo!(),
                                    LocalTermPattern::Ritchie {
                                        ritchie_kind,
                                        parameter_liasoned_tys,
                                        return_ty,
                                    } => todo!(),
                                },
                                None =>self. local_term_region
                                    .new_implicit_symbol(
                                        expr_idx,
                                        ImplicitSymbolVariant::ImplicitType,
                                    )
                                    .into(),
                            };
                            for item in items {
                                self.infer_new_expr_ty_discarded(
                                    item,
                                    ExpectImplicitlyConvertible::new_transient(element_ty), 
                                );
                            }
                            (
                                ListExprDisambiguation::NewList.into(),
                                LocalTerm::new_application(
                                    self.db,&mut self.
                                    local_term_region,
                                    expr_idx,
                                    self.term_menu.list_ty_ontology(),
                                    element_ty,
                                )
                                .map_err(|_| todo!()),
                            )
                        }
                    },
                )
            }
            Expr::BoxColonList { .. } => todo!(),
            Expr::Block { stmts } => Ok((
                ExprDisambiguation::Trivial,
                self.infer_new_block(stmts, expr_ty_expectation.clone(),  )
                    .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            )),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }

    fn calc_entity_path_expr_ty(
        &mut self,
        path: Option<EntityPath>,
        expr_ty_expectation: &impl ExpectLocalTerm, 
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        let disambiguation = expr_ty_expectation
            .disambiguate_ty_path(self.db(),self.local_term_region.unresolved_terms());
        Ok((
            disambiguation.into(),
            Ok(path
                .ok_or(DerivedExprTypeError::EntityPathError)?
                .ty(self.db, disambiguation)?
                .into()),
        ))
    }

    fn calc_explicit_application_or_ritchie_call_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        function: ExprIdx,
        expr_ty_expectation: &impl ExpectLocalTerm, 
        implicit_arguments: &Option<ImplicitArgumentList>,
        items: &idx_arena::ArenaIdxRange<Expr>,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        let Some(expectation_ok) = self.infer_new_expr_ty_for_outcome(
                function,
                ExpectEqsFunctionType::new(expr_ty_expectation.final_destination(self.db(), self.local_term_region.unresolved_terms())), 
            ) else {
                for item in items {
                    self.infer_new_expr_ty(item, ExpectAnyDerived,   );
                }
                return
                    Err(
                        DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                            .into(),
                    )
            };
        if let Some(implicit_arguments) = implicit_arguments {
            todo!()
        }
        match expectation_ok.variant() {
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                ritchie_kind,
                parameter_liasoned_tys,
            } => {
                self.calc_ritchie_call_nonself_arguments_expr_ty(
                    expr_idx,
                    &parameter_liasoned_tys,
                    *items, 
                );
                Ok((
                    ExprDisambiguation::ExplicitApplicationOrRitchieCall(
                        ApplicationOrRitchieCallExprDisambiguation::RitchieCall,
                    ),
                    Ok(expectation_ok.return_ty()),
                ))
            }
            ExpectEqsFunctionTypeOutcomeVariant::Curry { .. } => todo!(),
        }
    }

    fn calc_index_or_compose_with_list_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        owner: ExprIdx,
        indices: ExprIdxRange, 
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        let Some(owner_ty) = self.infer_new_expr_ty(
            owner, ExpectAnyOriginal,  
        ) else {
            for index in indices {
                self.infer_new_expr_ty(index, ExpectAnyDerived,  );
            }
            let e = DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred;
            return Err(
                DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred.into()
            )
        };
        Err(OriginalExprTypeError::TodoIndexOrComposeWithList.into())
    }

    fn calc_explicit_application(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
        final_destination: FinalDestination, 
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        Ok((
            ExprDisambiguation::Trivial,
            self.calc_explicit_application_expr_ty(
                function,
                argument,
                final_destination, 
            ),
        ))
    }
}
