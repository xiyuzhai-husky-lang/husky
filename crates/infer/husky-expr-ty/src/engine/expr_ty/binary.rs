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
        let menu = self.reduced_term_menu;
        match opr {
            BinaryOpr::PureClosed(opr) => self.calc_pure_closed_expr_ty(lopd, ropd, opr, menu),
            BinaryOpr::Comparison(_) => self.calc_comparison_expr_ty(lopd, ropd),
            BinaryOpr::ShortCircuitLogic(_) => self.calc_short_circuit_logic_expr_ty(lopd, ropd),
            BinaryOpr::Assign(opr) => self.calc_assign_expr_ty(expr_idx, lopd, opr, ropd),
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => self.calc_curry_expr_ty(lopd, ropd),
            BinaryOpr::As => self.calc_as_expr_ty(ropd, lopd),
            BinaryOpr::Is => self.calc_is_expr_ty(ropd),
            BinaryOpr::In => todo!(),
        }
    }

    fn calc_pure_closed_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryPureClosedOpr,
        menu: ReducedTermMenu,
    ) -> Result<LocalTerm, ExprTypeError> {
        let lopd_ty = self.infer_new_expr_ty_resolved(lopd, ExpectType);
        let ropd_ty = self.infer_new_expr_ty_resolved(ropd, ExpectType);
        let Some(lopd_ty) = lopd_ty
            else {
                return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
            };
        let Some(ropd_ty) = ropd_ty
            else {
                return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
            };
        let lopd_ty = self.db.intrinsic_ty(lopd_ty).reduced_term();
        let ropd_ty = self.db.intrinsic_ty(ropd_ty).reduced_term();
        match opr {
            BinaryPureClosedOpr::Add => match lopd_ty {
                lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
                    _ => Err(todo!()),
                },
                lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
                    _ => Err(todo!()),
                },
                _ => Err(todo!()),
            },
            BinaryPureClosedOpr::BitAnd => todo!(),
            BinaryPureClosedOpr::BitOr => todo!(),
            BinaryPureClosedOpr::BitXor => todo!(),
            BinaryPureClosedOpr::Div => match lopd_ty {
                lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
                    _ => Err(todo!()),
                },
                lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
                    _ => Err(todo!()),
                },
                _ => Err(todo!()),
            },
            BinaryPureClosedOpr::Mul => match lopd_ty {
                lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
                    _ => Err(todo!()),
                },
                lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
                    _ => Err(todo!()),
                },
                _ => Err(todo!()),
            },
            BinaryPureClosedOpr::RemEuclid => match lopd_ty {
                lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
                    _ => Err(todo!()),
                },
                lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
                    _ => Err(todo!()),
                },
                _ => Err(todo!()),
            },
            BinaryPureClosedOpr::Power => match lopd_ty {
                lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
                    _ => Err(todo!()),
                },
                lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
                    _ => Err(todo!()),
                },
                _ => Err(todo!()),
            },
            BinaryPureClosedOpr::Shl => todo!(),
            BinaryPureClosedOpr::Shr => todo!(),
            BinaryPureClosedOpr::Sub => match lopd_ty {
                lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
                    _ => Err(todo!()),
                },
                lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
                    ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
                    _ => Err(todo!()),
                },
                _ => Err(todo!()),
            },
        }
    }

    fn calc_comparison_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let lopd_ty = self.infer_new_expr_ty_resolved(lopd, ExpectType);
        let ropd_ty_expectation = match lopd_ty {
            Some(_) => todo!(),
            None => ExpectType,
        };
        let ropd_ty = self.infer_new_expr_ty_resolved(ropd, ropd_ty_expectation);
        Ok(self.reduced_term_menu.bool().into())
    }

    fn calc_short_circuit_logic_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        self.infer_new_expr_ty_resolved(lopd, self.expect_bool());
        self.infer_new_expr_ty_resolved(ropd, self.expect_bool());
        Ok(self.reduced_term_menu.bool().into())
    }

    fn calc_is_expr_ty(&mut self, ropd: ExprIdx) -> Result<LocalTerm, ExprTypeError> {
        let Some(ropd_ty) = self.infer_new_expr_ty_resolved(ropd, ExpectType)
            else {
                return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
            };
        match ropd_ty.term() {
            Term::Entity(path) if path == self.entity_path_menu.trai_ty().into() => {
                todo!()
            }
            Term::Category(_) => {
                todo!()
                // if let Some(ropd_term) = self.infer_new_expr_term(ropd) {
                //     ropd_expectation = ExpectImplicitConversion {
                //         destination: ropd_term,
                //     }
                // }
            }
            _ => todo!(),
        }
        Ok(self.reduced_term_menu.prop().into())
    }

    fn calc_as_expr_ty(
        &mut self,
        ropd: ExprIdx,
        lopd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        self.infer_new_expr_ty_resolved(ropd, ExpectSort);
        let Some(ropd_term) = self.infer_new_expr_term(ropd)
            else {
                return Err(DerivedExprTypeError::AsOperationRightOperandTermNotInferred.into())
            };
        let Some(lopd_ty) = self.infer_new_expr_ty_resolved(lopd, ExpectImplicitConversion{
            destination: todo!()
        })
            else {
                return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
            };
        todo!()
    }

    fn calc_curry_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let Some(lopd_ty) = self.infer_new_expr_ty_resolved(lopd, ExpectSort)
            else {
                return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
            };
        let Some(ropd_ty) = self.infer_new_expr_ty_resolved(ropd, ExpectSort)
            else {
                return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
            };
        let x = lopd_ty.term();
        match x {
            Term::Category(_) => (),
            _ => return Err(todo!()),
        }
        let y = ropd_ty.term();
        match y {
            Term::Category(_) => (),
            _ => return Err(todo!()),
        }
        Ok(self
            .db
            .reduced_term(TermCurry::new(self.db, /* ad hoc */ Variance::Invariant, x, y).into())
            .into())
    }

    fn calc_assign_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        lopd: ExprIdx,
        opr: Option<BinaryPureClosedOpr>,
        ropd: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let expr_eval_lifetime =
            self.new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        let (_, lopd_expectation_rule_idx) = self.infer_new_expr_ty_with_expectation_rule(
            lopd,
            ExpectRefMut {
                lifetime: expr_eval_lifetime,
            },
        );
        if let Some(lopd_expectation_rule_idx) = lopd_expectation_rule_idx.into_option() {
            match self.local_term_table[lopd_expectation_rule_idx].resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => unreachable!(),
                LocalTermExpectationResolveProgress::ResolvedOk(resolved_expectation) => {
                    match resolved_expectation.local_term {
                        LocalTerm::Resolved(_) => todo!(),
                        LocalTerm::Unresolved(_) => todo!(),
                    }
                }
                LocalTermExpectationResolveProgress::ResolvedErr(_) => (),
            }
        }
        // match lopd_ty {
        //     Some(lopd_ty) => match opr {
        //         Some(opr) => {
        //             self.infer_composite_assign_ropd_ty(lopd_expectation_rule_idx, opr, ropd)
        //         }
        //         None => self.infer_basic_assign_ropd_ty(lopd_expectation_rule_idx, ropd),
        //     },
        //     None => {
        //         self.infer_new_expr_ty(ropd, LocalTermExpectation::None);
        //     }
        // };
        Ok(self.reduced_term_menu.unit().into())
    }

    fn infer_basic_assign_ropd_ty(&mut self, lopd_ty: LocalTerm, ropd: ExprIdx) {
        let ropd_ty = self.infer_new_expr_ty(ropd, ExpectType);
        let Some(ropd_ty) = ropd_ty else { return };
        let lopd_ty = match lopd_ty {
            LocalTerm::Resolved(lopd_ty) => match lopd_ty.term() {
                Term::Application(lopd_ty) => todo!(),
                _ => todo!(),
            },
            LocalTerm::Unresolved(lopd_ty) => {
                match self.local_term_table[lopd_ty].unresolved_term() {
                    UnresolvedTerm::ImplicitSymbol(_) => todo!(),
                    UnresolvedTerm::TypeApplication { ty, arguments } => {
                        todo!()
                    }
                }
            }
        };
        let ropd_ty = match ropd_ty {
            LocalTerm::Resolved(ropd_ty) => self.db.intrinsic_ty(ropd_ty).reduced_term(),
            LocalTerm::Unresolved(_) => todo!(),
        };
    }

    fn infer_composite_assign_ropd_ty(
        &mut self,
        lopd_ty: LocalTerm,
        opr: BinaryPureClosedOpr,
        ropd: ExprIdx,
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
