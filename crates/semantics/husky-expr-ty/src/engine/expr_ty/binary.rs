mod assign;
mod assign_closed;
mod assign_shift;
mod comparison;
mod pure_closed;
mod shift;

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
    ) -> ExprTypeResult<LocalTerm> {
        let menu = self.term_menu;
        match opr {
            BinaryOpr::Closed(opr) => self.calc_binary_closed_expr_ty(lopd, ropd, opr, menu),
            BinaryOpr::Shift(opr) => self.calc_binary_shift_expr_ty(lopd, ropd, opr, menu),
            BinaryOpr::Comparison(_) => self.calc_binary_comparison_expr_ty(lopd, ropd),
            BinaryOpr::ShortCircuitLogic(_) => {
                self.calc_binary_short_circuit_logic_expr_ty(lopd, ropd)
            }
            BinaryOpr::Assign => self.calc_binary_assign_expr_ty(expr_idx, lopd, ropd),
            BinaryOpr::AssignClosed(opr) => {
                self.calc_binary_assign_closed_expr_ty(expr_idx, lopd, opr, ropd)
            }
            BinaryOpr::AssignShift(opr) => {
                self.calc_binary_assign_shift_expr_ty(expr_idx, lopd, opr, ropd)
            }
            BinaryOpr::ScopeResolution => Err(OriginalExprTypeError::TodoScopeResolution.into()),
            BinaryOpr::Curry => self.calc_curry_expr_ty(lopd, ropd),
            BinaryOpr::As => self.calc_as_expr_ty(ropd, lopd),
            BinaryOpr::Ins => self.calc_ins_expr_ty(ropd),
            BinaryOpr::In => todo!(),
        }
    }

    fn calc_binary_short_circuit_logic_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        self.infer_new_expr_ty_discarded(lopd, self.expect_implicitly_convertible_to_bool());
        self.infer_new_expr_ty_discarded(ropd, self.expect_implicitly_convertible_to_bool());
        Ok(self.term_menu.bool_ty_ontology().into())
    }

    fn calc_ins_expr_ty(&mut self, ropd: ExprIdx) -> Result<LocalTerm, ExprTypeError> {
        let Some(ropd_ty) = self.infer_new_expr_ty(
            ropd,
            ExpectAnyOriginal,
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
    ) -> Result<LocalTerm, ExprTypeError> {
        self.infer_new_expr_ty_discarded(ropd, ExpectEqsCategory::new_any_sort());
        let Some(ropd_term) = self.infer_new_expr_term(ropd,  )
            else {
                return Err(DerivedExprTypeError::AsOperationRightOperandTermNotInferred.into())
            };
        self.infer_new_expr_ty_discarded(lopd, ExpectExplicitlyConvertible::new(ropd_term));
        Ok(ropd_term)
    }

    fn calc_curry_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let expect_any_sort = ExpectEqsCategory::new_any_sort();
        let Some(lopd_universe) = self.infer_new_expr_ty_for_outcome(lopd, expect_any_sort,  )
            else {
                return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
            };
        let Some(ropd_universe) = self.infer_new_expr_ty_for_outcome(ropd, expect_any_sort,  )
            else {
                return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
            };
        todo!()
        // Ok(Term::new_category(x_u.max(y_u)).into())
    }

    fn calc_binary_assign_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        lopd: ExprIdx,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let expr_eval_lifetime = self
            .local_term_region
            .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        match self.infer_new_expr_ty_for_outcome(lopd, ExpectAnyOriginal) {
            Some(_) => todo!(),
            None => {
                self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived);
            }
        };
        Ok(self.term_menu.unit().into())
    }

    fn calc_binary_assign_closed_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        lopd: ExprIdx,
        opr: BinaryClosedOpr,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let expr_eval_lifetime = self
            .local_term_region
            .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        match self.infer_new_expr_ty_for_outcome(lopd, ExpectAnyOriginal) {
            Some(_) => todo!(),
            None => {
                self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived);
            }
        };
        Ok(self.term_menu.unit().into())
    }

    fn calc_binary_assign_shift_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        lopd: ExprIdx,
        opr: BinaryShiftOpr,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let expr_eval_lifetime = self
            .local_term_region
            .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        match self.infer_new_expr_ty_for_outcome(lopd, ExpectAnyOriginal) {
            Some(_) => todo!(),
            None => {
                self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived);
            }
        };
        Ok(self.term_menu.unit().into())
    }

    fn infer_basic_assign_ropd_ty(&mut self, lopd_ty: LocalTerm, ropd: ExprIdx) {
        let ropd_ty = self.infer_new_expr_ty(ropd, ExpectAnyOriginal);
        let Some(ropd_ty) = ropd_ty else { return };
        let lopd_ty = match lopd_ty {
            LocalTerm::Resolved(lopd_ty) => match lopd_ty {
                Term::Application(lopd_ty) => todo!(),
                _ => todo!(),
            },
            LocalTerm::Unresolved(lopd_ty) => {
                match self.local_term_region[lopd_ty].unresolved_term() {
                    LocalTermData::ImplicitSymbol(_) => todo!(),
                    LocalTermData::TypeOntology(_) => {
                        todo!()
                    }
                    LocalTermData::Ritchie(_) => todo!(),
                    LocalTermData::PlaceType { .. } => todo!(),
                }
            }
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
        opr: BinaryClosedOpr,
        ropd: ExprIdx,
    ) {
        match opr {
            BinaryClosedOpr::Add => todo!(),
            BinaryClosedOpr::BitAnd => todo!(),
            BinaryClosedOpr::BitOr => todo!(),
            BinaryClosedOpr::BitXor => todo!(),
            BinaryClosedOpr::Div => todo!(),
            BinaryClosedOpr::Mul => todo!(),
            BinaryClosedOpr::RemEuclid => todo!(),
            BinaryClosedOpr::Power => todo!(),
            BinaryClosedOpr::Sub => todo!(),
        }
    }
}
