mod associated_item;
mod binary;
mod box_list;
mod current_symbol;
mod field;
mod function_application;
mod function_call;
mod index_or_compose_with_list;
mod literal;
mod method;
mod prefix;
mod principal_entity_path;
mod ritchie_call_arguments_ty;
mod suffix;
mod utils;

pub use self::ritchie_call_arguments_ty::*;
pub(crate) use self::suffix::*;

use super::*;
use husky_opr::*;

pub(crate) enum ExprTypeResolveProgress<E: ExpectFluffyTerm> {
    Unresolved,
    Outcome(E::Outcome),
    ResolvedErr,
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_ty<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> Option<FluffyTerm> {
        self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
        self.expr_ty_infos[expr_idx].ty().ok()
    }

    /// infer the type of a new expression but don't need the result for now
    pub(super) fn infer_new_expr_ty_discarded<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) {
        self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
    }

    #[inline(always)]
    pub(super) fn infer_new_expr_ty_for_outcome<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> Option<E::Outcome>
    where
        E::Outcome: Clone,
    {
        let expectation_idx = self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
        self.fluffy_term_region
            .resolve_as_much_as_possible(self.db());
        let outcome = match expectation_idx {
            Some(expectation_idx) => self.fluffy_term_region[expectation_idx]
                .resolve_progress()
                .outcome::<E>()
                .cloned(),
            None => None,
        };
        outcome
    }

    #[inline(always)]
    fn infer_new_expr_ty_aux<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> Option<FluffyTermExpectationIdx> {
        let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation);
        let expectation_idx = match ty_result {
            Ok((_, Ok(ty))) => self.fluffy_term_region.add_expectation(
                ExpectationSource::new_expr(expr_idx),
                ty,
                expr_ty_expectation,
            ),
            _ => Default::default(),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
        self.fluffy_term_region
            .resolve_as_much_as_possible(self.db());
        expectation_idx
    }

    fn save_new_expr_ty(&mut self, expr_idx: SynExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> SemaExprResult<(SynExprDisambiguation, SemaExprResult<FluffyTerm>)> {
        match self.expr_region_data[expr_idx] {
            SynExpr::Literal(literal_token_idx, _) => Ok((
                SynExprDisambiguation::Trivial,
                self.calc_literal_expr_ty(expr_idx, literal_token_idx, expr_ty_expectation),
            )),
            SynExpr::PrincipalEntityPath {
                path_expr_idx,
                opt_path,
            } => self.calc_principal_item_path_expr_ty(opt_path, expr_ty_expectation),
            SynExpr::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => self.calc_associated_item_ty(expr_idx, parent_path, ident_token),
            SynExpr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => Ok((
                SynExprDisambiguation::Trivial,
                match self
                    .symbol_tys
                    .inherited_symbol_map()
                    .get(inherited_symbol_idx)
                {
                    Some(ty) => Ok((*ty).into()),
                    None => Err(DerivedSemaExprError::InheritedSymbolTypeError.into()),
                },
            )),
            SynExpr::CurrentSymbol {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => Ok((
                SynExprDisambiguation::Trivial,
                self.get_current_symbol_ty(expr_idx, current_symbol_idx),
            )),
            SynExpr::FrameVarDecl {
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            SynExpr::SelfType(_) => Ok((
                SynExprDisambiguation::Trivial,
                match self.self_ty_term {
                    Some(self_ty) => match self_ty.ty_unchecked(self.db)? {
                        Left(self_ty_ty) => Ok(self_ty_ty.into()),
                        Right(_) => unreachable!(),
                    }, // todo: impl binding
                    None => Err(DerivedSemaExprError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            SynExpr::SelfValue(_) => Ok((
                SynExprDisambiguation::Trivial,
                match self.self_ty_term {
                    Some(self_ty) => Ok(self_ty.into()), // todo: impl binding
                    None => Err(DerivedSemaExprError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            SynExpr::Binary {
                lopd, opr, ropd, ..
            } => Ok((
                SynExprDisambiguation::Trivial,
                self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd),
            )),
            SynExpr::Be {
                src, ref target, ..
            } => {
                match self.infer_new_expr_ty(src, ExpectAnyOriginal) {
                    Some(src_ty) => match target {
                        Ok(target) => self.infer_pattern_and_symbols_ty(
                            target.syn_pattern_root(),
                            src_ty,
                            target.variables(),
                        ),
                        Err(_) => (),
                    },
                    None => (),
                };
                Ok((
                    SynExprDisambiguation::Trivial,
                    Ok(self.term_menu.bool_ty_ontology().into()),
                ))
            }
            SynExpr::Prefix { opr, opd, .. } => self.calc_prefix_expr_ty(
                expr_idx,
                opr,
                opd,
                expr_ty_expectation.final_destination(self),
            ),
            SynExpr::Suffix { opd, opr, .. } => self.calc_suffix_expr_ty(
                expr_idx,
                opd,
                opr,
                expr_ty_expectation.final_destination(self),
            ),
            SynExpr::FunctionApplicationOrCall {
                function,
                ref generic_arguments,
                ref items,
                ..
            } => self.calc_function_application_or_call_expr_ty(
                expr_idx,
                function,
                expr_ty_expectation,
                generic_arguments.as_ref(),
                items,
            ),
            SynExpr::FunctionCall {
                function,
                ref generic_arguments,
                ref items,
                ..
            } => self.calc_function_call_expr_ty(
                expr_idx,
                function,
                expr_ty_expectation.final_destination(self),
                generic_arguments.as_ref(),
                items,
            ),
            SynExpr::Field {
                owner, ident_token, ..
            } => self.calc_field_expr_ty(owner, ident_token),
            SynExpr::MethodApplicationOrCall {
                self_argument,
                ident_token,
                ref generic_arguments,
                ref items,
                ..
            } => self.calc_method_application_or_call_ty(
                expr_idx,
                self_argument,
                ident_token,
                generic_arguments.as_ref(),
                items,
            ),
            SynExpr::TemplateInstantiation {
                template,
                ref generic_arguments,
            } => todo!(),
            SynExpr::ExplicitApplication {
                function_expr_idx,
                argument_expr_idx,
            } => self.calc_explicit_application(
                expr_idx,
                function_expr_idx,
                argument_expr_idx,
                expr_ty_expectation.final_destination(self),
            ),
            SynExpr::Bracketed { item, .. } => Ok((
                SynExprDisambiguation::Trivial,
                self.infer_new_expr_ty(item, expr_ty_expectation.clone())
                    .ok_or(DerivedSemaExprError::BracketedItemTypeError.into()),
            )),
            SynExpr::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => Ok((
                SynExprDisambiguation::Trivial,
                Ok(self.term_menu.ex_inv_ty0_to_ty0().into()),
            )),
            SynExpr::Unit { .. } => Ok((
                SynExprDisambiguation::Trivial,
                Ok(self.term_menu.unit_ty_ontology().into()),
            )),
            SynExpr::NewTuple { ref items, .. } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                items: ref indices,
                ..
            } => self.calc_index_or_compose_with_list_expr_ty(expr_idx, owner, indices),
            SynExpr::List { ref items, .. } => {
                Ok(match expr_ty_expectation.disambiguate_ty_path(self) {
                    TypePathDisambiguation::OntologyConstructor => {
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
                    TypePathDisambiguation::InstanceConstructor => {
                        let element_ty: FluffyTerm = match expr_ty_expectation
                            .destination_term_data(self.db(), self.fluffy_term_region.terms())
                        {
                            Some(ty_pattern) => match ty_pattern {
                                FluffyTermData::Literal(_) => todo!(),
                                FluffyTermData::TypeOntology {
                                    refined_ty_path,
                                    ty_arguments,
                                    ..
                                } => match refined_ty_path {
                                    Left(PreludeTypePath::List) => {
                                        assert_eq!(ty_arguments.len(), 1);
                                        ty_arguments[0]
                                    }
                                    Left(PreludeTypePath::Arr(_)) => {
                                        assert_eq!(ty_arguments.len(), 1);
                                        ty_arguments[0]
                                    }
                                    _ => todo!(),
                                },
                                // ad hoc
                                // FluffyTermData::TypeOntologyAtPlace {
                                //     refined_ty_path,
                                //     ty_arguments,
                                //     ..
                                // } => match refined_ty_path {
                                //     Left(PreludeTypePath::List) => {
                                //         assert_eq!(ty_arguments.len(), 1);
                                //         ty_arguments[0]
                                //     }
                                //     Left(PreludeTypePath::Array) => todo!(),
                                //     _ => todo!(),
                                // },
                                FluffyTermData::Curry {
                                    curry_kind,
                                    variance,
                                    parameter_variable,
                                    parameter_ty,
                                    return_ty,
                                    ty_ethereal_term,
                                } => todo!(),
                                FluffyTermData::Hole(_, _) => todo!(),
                                FluffyTermData::Category(_) => todo!(),
                                FluffyTermData::Ritchie {
                                    ritchie_kind,
                                    parameter_contracted_tys,
                                    return_ty,
                                    ..
                                } => todo!(),
                                FluffyTermData::Symbol { .. } => todo!(),
                                FluffyTermData::Variable { ty } => todo!(),
                                FluffyTermData::TypeVariant { path } => todo!(),
                            },
                            None => self.new_hole(expr_idx, HoleKind::Any).into(),
                        };
                        for item in items {
                            self.infer_new_expr_ty_discarded(
                                item.expr_idx(),
                                ExpectCoersion::new_move(element_ty),
                            );
                        }
                        (
                            ListExprDisambiguation::NewList.into(),
                            FluffyTerm::new_application(
                                self,
                                expr_idx,
                                self.term_menu.list_ty_ontology(),
                                element_ty,
                            )
                            .map_err(|_| todo!()),
                        )
                    }
                })
            }
            SynExpr::BoxColonList { ref items, .. } => match items.len() {
                0 => Ok((
                    SynExprDisambiguation::Trivial,
                    Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                )),
                _ => todo!(),
            },
            SynExpr::Block { stmts } => Ok((
                SynExprDisambiguation::Trivial,
                self.infer_new_block(stmts, expr_ty_expectation.clone())
                    .ok_or(DerivedSemaExprError::BlockTypeError.into()),
            )),
            SynExpr::EmptyHtmlTag { .. } => Ok((
                SynExprDisambiguation::Trivial,
                Ok(self.term_menu.html_ty_ontology().into()),
            )),
            SynExpr::Ritchie {
                ref parameter_ty_items,
                return_ty_expr,
                ..
            } => {
                for parameter_ty in parameter_ty_items {
                    self.infer_new_expr_ty_discarded(
                        parameter_ty.expr_idx(),
                        self.expect_ty0_subtype(),
                    );
                }
                return_ty_expr.map(|return_ty_expr| {
                    self.infer_new_expr_ty_discarded(return_ty_expr, self.expect_ty0_subtype())
                });
                Ok((
                    SynExprDisambiguation::Trivial,
                    Ok(self.term_menu.ty0().into()),
                ))
            }
            SynExpr::Sorry {
                regional_token_idx: token_idx,
            } => todo!(),
            SynExpr::Todo {
                regional_token_idx: token_idx,
            } => Ok((
                SynExprDisambiguation::Trivial,
                Ok(self.term_menu.never().into()),
            )),
            SynExpr::Unreachable { regional_token_idx } => todo!(),
            SynExpr::Err(_) => Err(DerivedSemaExprError::ExprError.into()),
        }
    }

    fn calc_explicit_application(
        &mut self,
        expr_idx: SynExprIdx,
        function_expr_idx: SynExprIdx,
        argument_expr_idx: SynExprIdx,
        final_destination: FinalDestination,
    ) -> SemaExprResult<(SynExprDisambiguation, SemaExprResult<FluffyTerm>)> {
        Ok((
            SynExprDisambiguation::Trivial,
            self.calc_function_application_expr_ty(
                expr_idx,
                function_expr_idx,
                argument_expr_idx,
                final_destination,
            ),
        ))
    }
}
