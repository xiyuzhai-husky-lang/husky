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
    ) -> (SemaExprIdx, Option<FluffyTerm>) {
        todo!()
        // self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
        // self.expr_ty_infos[expr_idx].ty().ok()
    }

    /// infer the type of a new expression but don't need the result for now
    pub(super) fn build_new_expr_ty_discarded<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> SemaExprIdx {
        todo!();
        // self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
    }

    #[inline(always)]
    pub(super) fn build_new_sema_expr_with_outcome<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> (SemaExprIdx, Option<E::Outcome>)
    where
        E::Outcome: Clone,
    {
        todo!()
        // let expectation_idx = self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
        // self.fluffy_term_region
        //     .resolve_as_much_as_possible(self.db());
        // let outcome = match expectation_idx {
        //     Some(expectation_idx) => self.fluffy_term_region[expectation_idx]
        //         .resolve_progress()
        //         .outcome::<E>()
        //         .cloned(),
        //     None => None,
        // };
        // outcome
    }

    #[inline(always)]
    fn infer_new_expr_ty_aux<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> Option<FluffyTermExpectationIdx> {
        todo!()
        // let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation);
        // let expectation_idx = match ty_result {
        //     Ok((_, Ok(ty))) => self.fluffy_term_region.add_expectation(
        //         ExpectationSource::new_expr(expr_idx),
        //         ty,
        //         expr_ty_expectation,
        //     ),
        //     _ => Default::default(),
        // };
        // self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
        // self.fluffy_term_region
        //     .resolve_as_much_as_possible(self.db());
        // expectation_idx
    }

    fn save_new_expr_ty(&mut self, expr_idx: SynExprIdx) {
        todo!()
        // self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> (SemaExprResult<SemaExprData>, SemaExprResult<FluffyTerm>) {
        match self.expr_region_data[expr_idx] {
            SynExprData::Literal(literal_token_idx, literal_data) => Ok((
                SemaExprData::Literal(literal_token_idx, literal_data),
                self.calc_literal_expr_ty(expr_idx, literal_token_idx, expr_ty_expectation),
            )),
            SynExprData::PrincipalEntityPath {
                path_expr_idx,
                opt_path,
            } => self.calc_principal_item_path_expr_ty(opt_path, expr_ty_expectation),
            SynExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => self.calc_associated_item_ty(expr_idx, parent_path, ident_token),
            SynExprData::InheritedSymbol {
                ident,
                regional_token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => Ok((
                SemaExprData::InheritedSymbol {
                    ident,
                    regional_token_idx,
                    inherited_symbol_idx,
                    inherited_symbol_kind,
                },
                match self
                    .symbol_tys
                    .inherited_symbol_map()
                    .get(inherited_symbol_idx)
                {
                    Some(ty) => Ok((*ty).into()),
                    None => Err(DerivedSemaExprError::InheritedSymbolTypeError.into()),
                },
            )),
            SynExprData::CurrentSymbol {
                ident,
                regional_token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => Ok((
                SemaExprData::CurrentSymbol {
                    ident,
                    regional_token_idx,
                    current_symbol_idx,
                    current_symbol_kind,
                },
                self.get_current_symbol_ty(expr_idx, current_symbol_idx),
            )),
            SynExprData::FrameVarDecl {
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            SynExprData::SelfType(regional_token_idx) => Ok((
                SemaExprData::SelfType(regional_token_idx),
                match self.self_ty_term {
                    Some(self_ty) => match self_ty.ty_unchecked(self.db)? {
                        Left(self_ty_ty) => Ok(self_ty_ty.into()),
                        Right(_) => unreachable!(),
                    }, // todo: impl binding
                    None => Err(DerivedSemaExprError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            SynExprData::SelfValue(regional_token_idx) => Ok((
                SemaExprData::SelfType(regional_token_idx),
                match self.self_ty_term {
                    Some(self_ty) => Ok(self_ty.into()), // todo: impl binding
                    None => Err(DerivedSemaExprError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            SynExprData::Binary {
                lopd, opr, ropd, ..
            } => Ok((
                SemaExprData::Trivial,
                self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd),
            )),
            SynExprData::Be {
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
                    SemaExprData::Trivial,
                    Ok(self.term_menu.bool_ty_ontology().into()),
                ))
            }
            SynExprData::Prefix { opr, opd, .. } => self.calc_prefix_expr_ty(
                expr_idx,
                opr,
                opd,
                expr_ty_expectation.final_destination(self),
            ),
            SynExprData::Suffix { opd, opr, .. } => self.calc_suffix_expr_ty(
                expr_idx,
                opd,
                opr,
                expr_ty_expectation.final_destination(self),
            ),
            SynExprData::FunctionApplicationOrCall {
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
            SynExprData::FunctionCall {
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
            SynExprData::Field {
                owner, ident_token, ..
            } => self.calc_field_expr_ty(owner, ident_token),
            SynExprData::MethodApplicationOrCall {
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
            SynExprData::TemplateInstantiation {
                template,
                ref generic_arguments,
            } => todo!(),
            SynExprData::ExplicitApplication {
                function_expr_idx,
                argument_expr_idx,
            } => self.build_explicit_application_sema_expr(
                expr_idx,
                function_expr_idx,
                argument_expr_idx,
                expr_ty_expectation.final_destination(self),
            ),
            SynExprData::Bracketed {
                lpar_regional_token_idx,
                item,
                rpar_regional_token_idx,
            } => {
                let (item, infer_new_expr_ty) =
                    self.infer_new_expr_ty(item, expr_ty_expectation.clone());
                Ok((
                    SemaExprData::Bracketed {
                        lpar_regional_token_idx,
                        item,
                        rpar_regional_token_idx,
                    },
                    infer_new_expr_ty.ok_or(DerivedSemaExprError::BracketedItemTypeError.into()),
                ))
            }
            SynExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => Ok((
                SemaExprData::At {
                    at_regional_token_idx,
                    place_label_regional_token,
                },
                Ok(self.term_menu.ex_inv_ty0_to_ty0().into()),
            )),
            SynExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => Ok((
                SemaExprData::Unit {
                    lpar_regional_token_idx,
                    rpar_regional_token_idx,
                },
                match expr_ty_expectation.final_destination(self) {
                    FinalDestination::Sort => Ok(self.term_menu.ty0().into()),
                    FinalDestination::TypeOntology
                    | FinalDestination::AnyOriginal
                    | FinalDestination::AnyDerived => Ok(self.term_menu.unit_ty_ontology().into()),
                    FinalDestination::Ritchie(_) => todo!(),
                },
            )),
            SynExprData::NewTuple { ref items, .. } => todo!(),
            SynExprData::IndexOrCompositionWithList {
                owner,
                items: ref indices,
                ..
            } => self.calc_index_or_compose_with_list_expr_ty(expr_idx, owner, indices),
            SynExprData::List { ref items, .. } => {
                Ok(match expr_ty_expectation.disambiguate_ty_path(self) {
                    TypePathDisambiguation::OntologyConstructor => {
                        // ad hoc, assume universe is 1
                        match items.len() {
                            0 => (
                                SemaExprData::ListFunctor.into(),
                                Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                            ),
                            1 => (
                                SemaExprData::ArrayFunctor.into(),
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
                            self.build_new_expr_ty_discarded(
                                item.expr_idx(),
                                ExpectCoersion::new_move(element_ty),
                            );
                        }
                        (
                            SemaExprData::NewList.into(),
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
            SynExprData::BoxColonList { ref items, .. } => match items.len() {
                0 => Ok((
                    SemaExprData::BoxColonList {
                        lbox_regional_token_idx: todo!(),
                        colon_regional_token_idx: todo!(),
                        items: todo!(),
                        rbox_regional_token_idx: todo!(),
                    },
                    Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                )),
                _ => todo!(),
            },
            SynExprData::Block { stmts } => Ok((
                SemaExprData::Trivial,
                self.infer_new_block(stmts, expr_ty_expectation.clone())
                    .ok_or(DerivedSemaExprError::BlockTypeError.into()),
            )),
            SynExprData::EmptyHtmlTag { .. } => Ok((
                SemaExprData::EmptyHtmlTag {
                    empty_html_bra_idx: todo!(),
                    function_ident: todo!(),
                    arguments: todo!(),
                    empty_html_ket: todo!(),
                },
                Ok(self.term_menu.html_ty_ontology().into()),
            )),
            SynExprData::Ritchie {
                ref parameter_ty_items,
                return_ty_expr,
                ..
            } => {
                for parameter_ty in parameter_ty_items {
                    self.build_new_expr_ty_discarded(
                        parameter_ty.expr_idx(),
                        self.expect_ty0_subtype(),
                    );
                }
                return_ty_expr.map(|return_ty_expr| {
                    self.build_new_expr_ty_discarded(return_ty_expr, self.expect_ty0_subtype())
                });
                Ok((SemaExprData::Trivial, Ok(self.term_menu.ty0().into())))
            }
            SynExprData::Sorry {
                regional_token_idx: token_idx,
            } => todo!(),
            SynExprData::Todo {
                regional_token_idx: token_idx,
            } => Ok((SemaExprData::Trivial, Ok(self.term_menu.never().into()))),
            SynExprData::Unreachable { regional_token_idx } => todo!(),
            SynExprData::Err(_) => Err(DerivedSemaExprError::ExprError.into()),
        }
    }

    fn build_explicit_application_sema_expr(
        &mut self,
        expr_idx: SynExprIdx,
        function_expr_idx: SynExprIdx,
        argument_expr_idx: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (SemaExprResult<SemaExprData>, SemaExprResult<FluffyTerm>) {
        (
            Ok(SemaExprData::ExplicitApplication {
                function_sema_expr_idx: (),
                argument_sema_expr_idx: (),
            }),
            self.calc_function_application_expr_ty(
                expr_idx,
                function_expr_idx,
                argument_expr_idx,
                final_destination,
            ),
        )
    }
}
