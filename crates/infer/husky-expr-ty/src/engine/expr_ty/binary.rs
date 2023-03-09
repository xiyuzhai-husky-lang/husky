mod pure_closed;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub(super) fn calc_binary_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        lopd: ExprIdx,
        opr: BinaryOpr,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<LocalTerm> {
        let menu = self.term_menu;
        match opr {
            BinaryOpr::PureClosed(opr) => {
                self.calc_pure_closed_expr_ty(lopd, ropd, opr, menu, local_term_region)
            }
            BinaryOpr::Comparison(_) => self.calc_comparison_expr_ty(lopd, ropd, local_term_region),
            BinaryOpr::ShortCircuitLogic(_) => {
                self.calc_short_circuit_logic_expr_ty(lopd, ropd, local_term_region)
            }
            BinaryOpr::Assign(opr) => {
                self.calc_assign_expr_ty(expr_idx, lopd, opr, ropd, local_term_region)
            }
            BinaryOpr::ScopeResolution => Err(OriginalExprTypeError::TodoScopeResolution.into()),
            BinaryOpr::Curry => self.calc_curry_expr_ty(lopd, ropd, local_term_region),
            BinaryOpr::As => self.calc_as_expr_ty(ropd, lopd, local_term_region),
            BinaryOpr::Ins => self.calc_ins_expr_ty(ropd, local_term_region),
            BinaryOpr::In => todo!(),
        }
    }

    fn calc_comparison_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let lopd_ty = self.infer_new_expr_ty(lopd, ExpectAnyOriginal, local_term_region);
        match lopd_ty {
            Some(destination) => self.infer_new_expr_ty_discarded(
                ropd,
                ExpectImplicitlyConvertible { dst: destination },
                local_term_region,
            ),
            None => self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived, local_term_region),
        };
        Ok(self.term_menu.bool().into())
    }

    fn calc_short_circuit_logic_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        self.infer_new_expr_ty_discarded(
            lopd,
            self.expect_implicitly_convertible_to_bool(),
            local_term_region,
        );
        self.infer_new_expr_ty_discarded(
            ropd,
            self.expect_implicitly_convertible_to_bool(),
            local_term_region,
        );
        Ok(self.term_menu.bool().into())
    }

    fn calc_ins_expr_ty(
        &mut self,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let Some(ropd_ty) = self.infer_new_expr_ty(
            ropd,
            ExpectAnyOriginal,
            local_term_region,
        ) else {
                return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
            };
        // todo
        // match ropd_ty {
        //     Term::Entity(path) if path == self.entity_path_menu.trai_ty().into() => {
        //         todo!()
        //     }
        //     Term::Category(_) => {
        //         todo!()
        //         // if let Some(ropd_term) = self.infer_new_expr_term(ropd) {
        //         //     ropd_expectation = ExpectImplicitConversion {
        //         //         destination: ropd_term,
        //         //     }
        //         // }
        //     }
        //     _ => todo!(),
        // }
        Ok(self.term_menu.prop().into())
    }

    fn calc_as_expr_ty(
        &mut self,
        ropd: ExprIdx,
        lopd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        self.infer_new_expr_ty_discarded(
            ropd,
            ExpectEqsCategory {
                smallest_universe: 0.into(),
            },
            local_term_region,
        );
        let Some(ropd_term) = self.infer_new_expr_term(ropd)
            else {
                return Err(DerivedExprTypeError::AsOperationRightOperandTermNotInferred.into())
            };
        self.infer_new_expr_ty_discarded(
            lopd,
            ExpectExplicitlyConvertible {
                destination: ropd_term,
            },
            local_term_region,
        );
        print_debug_expr!(self, ropd);
        p!(self.debug(&ropd_term));
        Ok(ropd_term)
    }

    fn calc_curry_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let expect_any_sort = ExpectEqsCategory {
            smallest_universe: 0.into(),
        };
        let Some(lopd_universe) = self.infer_new_expr_ty_for_outcome(lopd, expect_any_sort, local_term_region)
            else {
                return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
            };
        let Some(ropd_universe) = self.infer_new_expr_ty_for_outcome(ropd, expect_any_sort, local_term_region)
            else {
                return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
            };
        todo!()
        // Ok(Term::new_category(x_u.max(y_u)).into())
    }

    fn calc_assign_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        lopd: ExprIdx,
        opr: Option<BinaryPureClosedOpr>,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let expr_eval_lifetime = local_term_region
            .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        match self.infer_new_expr_ty_for_outcome(
            lopd,
            ExpectEqsRefMutApplication {
                lifetime: expr_eval_lifetime,
            },
            local_term_region,
        ) {
            Some(_) => todo!(),
            None => {
                self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived, local_term_region);
            }
        };
        Ok(self.term_menu.unit().into())
    }

    fn infer_basic_assign_ropd_ty(
        &mut self,
        lopd_ty: LocalTerm,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) {
        let ropd_ty = self.infer_new_expr_ty(ropd, ExpectAnyOriginal, local_term_region);
        let Some(ropd_ty) = ropd_ty else { return };
        let lopd_ty = match lopd_ty {
            LocalTerm::Resolved(lopd_ty) => match lopd_ty {
                Term::Application(lopd_ty) => todo!(),
                _ => todo!(),
            },
            LocalTerm::Unresolved(lopd_ty) => match local_term_region[lopd_ty].unresolved_term() {
                UnresolvedTerm::ImplicitSymbol(_) => todo!(),
                UnresolvedTerm::TypeApplication {
                    ty_path: ty,
                    arguments,
                } => {
                    todo!()
                }
                UnresolvedTerm::Ritchie {
                    ritchie_kind,
                    parameter_tys,
                    return_ty,
                } => todo!(),
            },
        };
        let ropd_ty = match ropd_ty {
            LocalTerm::Resolved(ropd_ty) => todo!(),
            // self.db.intrinsic_ty(ropd_ty).reduced_term(),
            LocalTerm::Unresolved(_) => todo!(),
        };
    }

    fn infer_composite_assign_ropd_ty(
        &mut self,
        lopd_ty: LocalTerm,
        opr: BinaryPureClosedOpr,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) {
        match opr {
            BinaryPureClosedOpr::Add => todo!(),
            BinaryPureClosedOpr::BitAnd => todo!(),
            BinaryPureClosedOpr::BitOr => todo!(),
            BinaryPureClosedOpr::BitXor => todo!(),
            BinaryPureClosedOpr::Div => todo!(),
            BinaryPureClosedOpr::Mul => todo!(),
            BinaryPureClosedOpr::RemEuclid => todo!(),
            BinaryPureClosedOpr::Power => todo!(),
            BinaryPureClosedOpr::Shl => todo!(),
            BinaryPureClosedOpr::Shr => todo!(),
            BinaryPureClosedOpr::Sub => todo!(),
        }
    }
}
