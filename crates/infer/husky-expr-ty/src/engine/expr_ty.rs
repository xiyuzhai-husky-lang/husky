mod application;
mod application_or_ritchie_call_ty;
mod binary;
mod box_list;
mod field;
mod index_or_compose_with_list;
mod literal;
mod method;
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
        let (ty_result, disambiguation) =
            self.calc_expr_ty(expr_idx, &expr_ty_expectation, local_term_region);
        let expectation_idx = match ty_result {
            Ok(ty) => {
                self.add_expectation_rule(expr_idx, ty, expr_ty_expectation, local_term_region)
            }
            Err(_) => Default::default(),
        };
        self.save_new_expr_ty(
            expr_idx,
            ExprTypeInfo::new(ty_result, disambiguation, expectation_idx),
        );
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
        let (ty_result, disambiguation) =
            self.calc_expr_ty(expr_idx, &expr_ty_expectation, local_term_region);
        let expectation_idx = match ty_result {
            Ok(ty) => {
                self.add_expectation_rule(expr_idx, ty, expr_ty_expectation, local_term_region)
            }
            Err(_) => Default::default(),
        };
        self.save_new_expr_ty(
            expr_idx,
            ExprTypeInfo::new(ty_result, disambiguation, expectation_idx),
        );
    }

    #[inline(always)]
    pub(super) fn infer_new_expr_ty_with_expectation_returned<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<E::Outcome>
    where
        E::Outcome: Clone,
    {
        let (ty_result, disambiguation) =
            self.calc_expr_ty(expr_idx, &expr_ty_expectation, local_term_region);
        let expectation_idx = match ty_result {
            Ok(ty) => {
                self.add_expectation_rule(expr_idx, ty, expr_ty_expectation, local_term_region)
            }
            Err(_) => Default::default(),
        };
        self.save_new_expr_ty(
            expr_idx,
            ExprTypeInfo::new(ty_result, disambiguation, expectation_idx),
        );
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak, local_term_region);
        let resolved_ok = match expectation_idx.into_option() {
            Some(expectation_idx) => local_term_region[expectation_idx]
                .resolve_progress()
                .resolved_ok()
                .cloned(),
            None => None,
        };
        resolved_ok
    }

    fn save_new_expr_ty(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: &impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
    ) -> (
        ExprTypeResult<LocalTerm>,
        ExprTypeResult<ExprDisambiguation>,
    ) {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx) => (
                self.calc_literal(
                    expr_idx,
                    literal_token_idx,
                    expr_ty_expectation,
                    local_term_region,
                ),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => (
                match entity_path {
                    Some(entity_path) => {
                        match husky_ty::entity_path_ty(
                            self.db,
                            expr_ty_expectation.entity_path_ty_expectation(
                                self.db,
                                local_term_region.unresolved_terms(),
                            ),
                            entity_path,
                        ) {
                            Ok(ty) => Ok(ty.into()),
                            Err(_) => Err(DerivedExprTypeError::EntityTypeError.into()),
                        }
                    }
                    None => Err(DerivedExprTypeError::EntityPathError.into()),
                },
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => (
                match self.inherited_symbol_tys.get(inherited_symbol_idx) {
                    Some(ty) => Ok((*ty).into()),
                    None => Err(DerivedExprTypeError::InheritedSymbolTypeError.into()),
                },
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::CurrentSymbol {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => (
                self.current_symbol_tys
                    .get(current_symbol_idx)
                    .copied()
                    .ok_or(DerivedExprTypeError::CurrentSymbolTypeError.into()),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::FrameVarDecl {
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => (
                match self.self_ty {
                    Some(_) => todo!(),
                    None => Err(DerivedExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::BinaryOpn {
                lopd,
                opr,
                ropd,
                opr_token_idx,
                ..
            } => (
                self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd, local_term_region),
                Ok(ExprDisambiguation::Trivial),
            ),
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
                (
                    Ok(self.reduced_term_menu.bool().into()),
                    Ok(ExprDisambiguation::Trivial),
                )
            }
            Expr::PrefixOpn { opr, opd, .. } => (
                self.calc_prefix_ty(opr, opd, local_term_region),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::SuffixOpn { opd, opr, .. } => (
                self.calc_suffix_ty(opd, opr, local_term_region),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::ApplicationOrRitchieCall {
                function,
                ref implicit_arguments,
                ref items,
                ..
            } => self.calc_application_or_ritchie_call_expr(
                function,
                expr_ty_expectation,
                local_term_region,
                implicit_arguments,
                items,
            ),
            Expr::Field {
                owner, ident_token, ..
            } => (
                self.calc_field_expr(owner, ident_token, local_term_region),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::MethodCall {
                self_argument,
                ident_token,
                ref implicit_arguments,
                nonself_arguments,
                ..
            } => (
                self.calc_method_expr(
                    self_argument,
                    ident_token,
                    implicit_arguments.as_ref(),
                    nonself_arguments,
                    local_term_region,
                ),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => (
                self.calc_application(function, argument, local_term_region),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::Bracketed { item, .. } => (
                self.infer_new_expr_ty(item, expr_ty_expectation.clone(), local_term_region)
                    .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::NewTuple { items, .. } => todo!(),
            Expr::IndexOrComposeWithList {
                owner,
                items: indices,
                ..
            } => (
                self.calc_index_or_compose_with_list_expr(
                    expr_idx,
                    owner,
                    indices,
                    local_term_region,
                ),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::BoxList { items, .. } => (
                self.calc_new_list_expr(expr_idx, items, local_term_region),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::BoxColonList { .. } => todo!(),
            Expr::Block { stmts } => (
                self.infer_new_block(stmts, expr_ty_expectation.clone(), local_term_region)
                    .ok_or(DerivedExprTypeError::BlockTypeError.into()),
                Ok(ExprDisambiguation::Trivial),
            ),
            Expr::Err(_) => (
                Err(DerivedExprTypeError::ExprError.into()),
                Ok(ExprDisambiguation::Trivial),
            ),
        }
    }
}
