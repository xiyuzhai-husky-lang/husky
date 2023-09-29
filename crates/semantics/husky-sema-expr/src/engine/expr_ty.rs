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
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) enum ExprTypeResolveProgress<E: ExpectFluffyTerm> {
    Unresolved,
    Outcome(E::Outcome),
    ResolvedErr,
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_expr_with_its_ty_returned<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> (SemaExprIdx, Option<FluffyTerm>) {
        let (sema_expr_idx, _) = self.build_sema_expr_aux(expr_idx, expr_ty_expectation);
        (sema_expr_idx, sema_expr_idx.ok_ty(&self.sema_expr_arena))
    }

    /// infer the type of a new expression but don't need the result for now
    pub(crate) fn build_sema_expr<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> SemaExprIdx {
        let (sema_expr_idx, _) = self.build_sema_expr_aux(expr_idx, expr_ty_expectation);
        sema_expr_idx
    }

    #[inline(always)]
    pub(crate) fn build_sema_expr_with_outcome<E: ExpectFluffyTerm>(
        &mut self,
        syn_expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> (SemaExprIdx, Option<E::Outcome>)
    where
        E::Outcome: Clone,
    {
        let (sema_expr_idx, expectation_idx) =
            self.build_sema_expr_aux(syn_expr_idx, expr_ty_expectation);
        self.fluffy_term_region
            .resolve_as_much_as_possible(self.db());
        let outcome = match expectation_idx {
            Some(expectation_idx) => self.fluffy_term_region[expectation_idx]
                .resolve_progress()
                .outcome::<E>()
                .cloned(),
            None => None,
        };
        (sema_expr_idx, outcome)
    }

    #[inline(always)]
    fn build_sema_expr_aux<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: E,
    ) -> (SemaExprIdx, Option<FluffyTermExpectationIdx>) {
        let (data_result, ty_result) =
            self.build_sema_expr_data_and_ty_result(expr_idx, &expr_ty_expectation);
        let expectation_idx = match ty_result {
            Ok(ty) => self.fluffy_term_region.add_expectation(
                ExpectationSource::new_expr(expr_idx),
                ty,
                expr_ty_expectation,
            ),
            _ => None,
        };
        let sema_expr_idx = self.sema_expr_arena.alloc_one(data_result, ty_result);
        self.fluffy_term_region
            .resolve_as_much_as_possible(self.db());
        (sema_expr_idx, expectation_idx)
    }

    fn build_sema_expr_data_and_ty_result(
        &mut self,
        expr_idx: SynExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        match self.syn_expr_region_data[expr_idx] {
            SynExprData::Literal(literal_token_idx, literal_data) => (
                Ok(SemaExprData::Literal(literal_token_idx, literal_data)),
                self.calc_literal_expr_ty(expr_idx, literal_token_idx, expr_ty_expectation),
            ),
            SynExprData::PrincipalEntityPath {
                path_expr_idx,
                opt_path,
            } => match opt_path {
                Some(path) => {
                    let ty_path_disambiguation = expr_ty_expectation.disambiguate_ty_path(self);
                    let ty_result = self.calc_principal_item_path_expr_ty(
                        path,
                        expr_ty_expectation,
                        ty_path_disambiguation,
                    );
                    (
                        Ok(SemaExprData::PrincipalEntityPath {
                            path_expr_idx,
                            path,
                            ty_path_disambiguation,
                        }),
                        ty_result,
                    )
                }
                None => {
                    // let Some(path) = path else {
                    //     Err(DerivedSemaExprTypeError::EntityPathError)?
                    // };
                    todo!()
                }
            },
            SynExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => {
                let (static_dispatch_result, ty_result) =
                    self.calc_associated_item_ty(expr_idx, parent_path, ident_token);
                let data_result =
                    static_dispatch_result.map(|static_dispatch| SemaExprData::AssociatedItem {
                        parent_expr_idx,
                        parent_path,
                        colon_colon_regional_token,
                        ident_token,
                        static_dispatch,
                    });
                (data_result, ty_result)
            }
            SynExprData::InheritedSymbol {
                ident,
                regional_token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => (
                Ok(SemaExprData::InheritedSymbol {
                    ident,
                    regional_token_idx,
                    inherited_symbol_idx,
                    inherited_symbol_kind,
                }),
                match self
                    .symbol_tys
                    .inherited_symbol_map()
                    .get(inherited_symbol_idx)
                {
                    Some(ty) => Ok((*ty).into()),
                    None => Err(DerivedSemaExprTypeError::InheritedSymbolTypeError.into()),
                },
            ),
            SynExprData::CurrentSymbol {
                ident,
                regional_token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => (
                Ok(SemaExprData::CurrentSymbol {
                    ident,
                    regional_token_idx,
                    current_symbol_idx,
                    current_symbol_kind,
                }),
                self.get_current_symbol_ty(expr_idx, current_symbol_idx),
            ),
            SynExprData::FrameVarDecl {
                ident,
                regional_token_idx,
                frame_var_symbol_idx,
                current_symbol_kind,
            } => (
                Ok(SemaExprData::FrameVarDecl {
                    ident,
                    regional_token_idx,
                    frame_var_symbol_idx,
                    current_symbol_kind,
                }),
                self.get_current_symbol_ty(expr_idx, frame_var_symbol_idx),
            ),
            SynExprData::SelfType(regional_token_idx) => (
                Ok(SemaExprData::SelfType(regional_token_idx)),
                match self.self_ty_term {
                    Some(self_ty) => match self_ty.ty_unchecked(self.db) {
                        Ok(Left(self_ty_ty)) => Ok(self_ty_ty.into()),
                        Err(e) => Err(e.into()),
                        Ok(Right(_)) => unreachable!(),
                    }, // todo: impl binding
                    None => Err(DerivedSemaExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
            ),
            SynExprData::SelfValue(regional_token_idx) => (
                Ok(SemaExprData::SelfType(regional_token_idx)),
                match self.self_ty_term {
                    Some(self_ty) => Ok(self_ty.into()), // todo: impl binding
                    None => Err(DerivedSemaExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
            ),
            SynExprData::Binary {
                lopd,
                opr,
                opr_regional_token_idx,
                ropd,
            } => {
                let (lopd, ropd, ty_result) = self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd);
                (
                    Ok(SemaExprData::Binary {
                        lopd,
                        opr,
                        opr_regional_token_idx,
                        ropd,
                    }),
                    ty_result,
                )
            }
            SynExprData::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => {
                let (src, src_ty) =
                    self.build_sema_expr_with_its_ty_returned(src, ExpectAnyOriginal);
                match src_ty {
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
                let data_result = target
                    .as_ref()
                    .map(|&target| SemaExprData::Be {
                        src,
                        be_regional_token_idx,
                        target,
                    })
                    .map_err(|e| e.into());
                (data_result, Ok(self.term_menu.bool_ty_ontology().into()))
            }
            SynExprData::Prefix {
                opr,
                opr_regional_token_idx,
                opd,
            } => {
                let (opd_sema_expr_idx_and_opr_result, ty_result) = self.build_prefix_sema_expr(
                    expr_idx,
                    opr,
                    opd,
                    expr_ty_expectation.final_destination(self),
                );
                match opd_sema_expr_idx_and_opr_result {
                    Ok((opd_sema_expr_idx, opr)) => (
                        Ok(SemaExprData::Prefix {
                            opr,
                            opr_regional_token_idx,
                            opd_sema_expr_idx,
                        }),
                        ty_result,
                    ),
                    Err(_) => todo!(),
                }
            }
            SynExprData::Suffix {
                opd,
                opr,
                opr_regional_token_idx,
            } => {
                let (opd_sema_expr_idx_and_opr_result, ty_result) = self.calc_suffix_expr_ty(
                    expr_idx,
                    opd,
                    opr,
                    expr_ty_expectation.final_destination(self),
                );
                match opd_sema_expr_idx_and_opr_result {
                    Ok((opd_sema_expr_idx, opr)) => (
                        Ok(SemaExprData::Suffix {
                            opd_sema_expr_idx,
                            opr,
                            opr_regional_token_idx,
                        }),
                        ty_result,
                    ),
                    Err(_) => todo!(),
                }
            }
            SynExprData::FunctionApplicationOrCall {
                function,
                ref template_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => self.build_function_application_or_call_sema_expr(
                expr_idx,
                function,
                expr_ty_expectation,
                template_arguments.as_ref(),
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
            ),
            SynExprData::FunctionCall {
                function,
                ref template_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => self.build_function_call_sema_expr(
                expr_idx,
                function,
                expr_ty_expectation.final_destination(self),
                template_arguments.as_ref(),
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
            ),
            SynExprData::Field {
                owner,
                dot_regional_token_idx,
                ident_token,
            } => self.calc_field_expr_ty(owner, dot_regional_token_idx, ident_token),
            SynExprData::MethodApplicationOrCall {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                lpar_regional_token_idx,
                template_arguments: ref generic_arguments,
                ref items,
                rpar_regional_token_idx,
            } => self.calc_method_application_or_call_ty(
                expr_idx,
                self_argument,
                dot_regional_token_idx,
                ident_token,
                generic_arguments.as_ref(),
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
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
                    self.build_sema_expr_with_its_ty_returned(item, expr_ty_expectation.clone());
                (
                    Ok(SemaExprData::Bracketed {
                        lpar_regional_token_idx,
                        item,
                        rpar_regional_token_idx,
                    }),
                    infer_new_expr_ty
                        .ok_or(DerivedSemaExprTypeError::BracketedItemTypeError.into()),
                )
            }
            SynExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => (
                Ok(SemaExprData::At {
                    at_regional_token_idx,
                    place_label_regional_token,
                }),
                Ok(self.term_menu.ex_inv_ty0_to_ty0().into()),
            ),
            SynExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => (
                Ok(SemaExprData::Unit {
                    lpar_regional_token_idx,
                    rpar_regional_token_idx,
                }),
                match expr_ty_expectation.final_destination(self) {
                    FinalDestination::Sort => Ok(self.term_menu.ty0().into()),
                    FinalDestination::TypeOntology
                    | FinalDestination::AnyOriginal
                    | FinalDestination::AnyDerived => Ok(self.term_menu.unit_ty_ontology().into()),
                    FinalDestination::Ritchie(_) => todo!(),
                },
            ),
            SynExprData::NewTuple { ref items, .. } => todo!(),
            SynExprData::IndexOrCompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => self.calc_index_or_compose_with_list_expr_ty(
                expr_idx,
                owner,
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            ),
            SynExprData::List {
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => {
                match expr_ty_expectation.disambiguate_ty_path(self) {
                    TypePathDisambiguation::OntologyConstructor => {
                        // ad hoc, assume universe is 1
                        match items.len() {
                            0 => (
                                Ok(SemaExprData::ListFunctor {
                                    lbox_regional_token_idx,
                                    rbox_regional_token_idx,
                                }
                                .into()),
                                Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                            ),
                            1 => (
                                Ok(SemaExprData::ArrayFunctor {
                                    lbox_regional_token_idx,
                                    items: todo!(),
                                    rbox_regional_token_idx,
                                }
                                .into()),
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
                        (
                            Ok(SemaExprData::NewList {
                                lbox_regional_token_idx,
                                items: items
                                    .iter()
                                    .map(|&syn_comma_list_item| {
                                        self.build_sema_comma_list_item(
                                            syn_comma_list_item,
                                            ExpectCoersion::new_move(element_ty),
                                        )
                                    })
                                    .collect(),
                                rbox_regional_token_idx,
                            }),
                            FluffyTerm::new_application(
                                self,
                                self.term_menu.list_ty_ontology(),
                                element_ty,
                            )
                            .map_err(|_| todo!()),
                        )
                    }
                }
            }
            SynExprData::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                rbox_regional_token_idx,
                ref items,
            } => match items.len() {
                0 => (
                    Ok(SemaExprData::BoxColonList {
                        lbox_regional_token_idx,
                        colon_regional_token_idx,
                        items: items.iter().map(|item| todo!()).collect(),
                        rbox_regional_token_idx,
                    }),
                    Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                ),
                _ => todo!(),
            },
            SynExprData::Block { stmts } => {
                let (stmts, block_ty) =
                    self.build_sema_block_with_its_ty_returned(stmts, expr_ty_expectation.clone());
                (
                    Ok(SemaExprData::Block { stmts }),
                    block_ty.ok_or(DerivedSemaExprTypeError::BlockTypeError.into()),
                )
            }
            SynExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => (
                Ok(SemaExprData::EmptyHtmlTag {
                    empty_html_bra_idx,
                    function_ident,
                    arguments: unsafe {
                        VecMap::from_iter_assuming_no_repetitions_unchecked(
                            arguments
                                .iter()
                                .map(|&argument| self.build_sema_html_argument_expr(argument)),
                        )
                    },
                    empty_html_ket,
                }),
                Ok(self.term_menu.html_ty_ontology().into()),
            ),
            SynExprData::Ritchie {
                ritchie_kind,
                ritchie_kind_regional_token_idx,
                lpar_token,
                ref parameter_ty_items,
                rpar_regional_token_idx,
                light_arrow_token,
                return_ty_syn_expr_idx,
            } => {
                let parameter_ty_items = parameter_ty_items
                    .iter()
                    .map(|&syn_comma_list_item| {
                        self.build_sema_comma_list_item(
                            syn_comma_list_item,
                            self.expect_ty0_subtype(),
                        )
                    })
                    .collect();
                let return_ty_sema_expr_idx =
                    return_ty_syn_expr_idx.map(|return_ty_syn_expr_idx| {
                        self.build_sema_expr(return_ty_syn_expr_idx, self.expect_ty0_subtype())
                    });
                (
                    Ok(SemaExprData::Ritchie {
                        ritchie_kind_regional_token_idx,
                        ritchie_kind,
                        lpar_token,
                        parameter_ty_items,
                        rpar_regional_token_idx,
                        light_arrow_token,
                        return_ty_sema_expr_idx,
                    }),
                    Ok(self.term_menu.ty0().into()),
                )
            }
            SynExprData::Sorry {
                regional_token_idx: token_idx,
            } => todo!(),
            SynExprData::Todo { regional_token_idx } => (
                Ok(SemaExprData::Todo { regional_token_idx }),
                Ok(self.term_menu.never().into()),
            ),
            SynExprData::Unreachable { regional_token_idx } => todo!(),
            SynExprData::Err(_) => (
                Err(DerivedSemaExprDataError::SynExpr.into()),
                Err(DerivedSemaExprTypeError::SynExprError.into()),
            ),
        }
    }

    fn build_explicit_application_sema_expr(
        &mut self,
        expr_idx: SynExprIdx,
        function_expr_idx: SynExprIdx,
        argument_expr_idx: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        self.calc_function_application_expr_ty(
            expr_idx,
            function_expr_idx,
            argument_expr_idx,
            final_destination,
        )
    }
}
