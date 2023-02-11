use super::*;

impl<'a> ExprTypeEngine<'a> {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub(super) fn calc_binary_ty(
        &mut self,
        lopd: ExprIdx,
        opr: BinaryOpr,
        ropd: ExprIdx,
    ) -> ExprTypeResult<LocalTerm> {
        let menu = self.reduced_term_menu;
        match opr {
            BinaryOpr::PureClosed(opr) => {
                let lopd_ty = self.infer_new_expr_ty_resolved(lopd, ExprTypeExpectation::None);
                let ropd_ty = self.infer_new_expr_ty_resolved(ropd, ExprTypeExpectation::None);
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
                    BinaryPureClosedOpr::Div => todo!(),
                    BinaryPureClosedOpr::Mul => todo!(),
                    BinaryPureClosedOpr::RemEuclid => todo!(),
                    BinaryPureClosedOpr::Power => todo!(),
                    BinaryPureClosedOpr::Shl => todo!(),
                    BinaryPureClosedOpr::Shr => todo!(),
                    BinaryPureClosedOpr::Sub => todo!(),
                }
            }
            BinaryOpr::Comparison(_) => {
                let lopd_ty = self.infer_new_expr_ty_resolved(lopd, ExprTypeExpectation::None);
                let ropd_ty_expectation = match lopd_ty {
                    Some(_) => todo!(),
                    None => ExprTypeExpectation::None,
                };
                let ropd_ty = self.infer_new_expr_ty_resolved(ropd, ropd_ty_expectation);
                Ok(self.reduced_term_menu.bool().into())
            }
            BinaryOpr::ShortcuitLogic(_) => {
                let expectation = ExprTypeExpectation::ImplicitlyConvertibleTo {
                    ty: self.reduced_term_menu.bool().into(),
                };
                self.infer_new_expr_ty_resolved(lopd, expectation);
                self.infer_new_expr_ty_resolved(ropd, expectation);
                Ok(self.reduced_term_menu.bool().into())
            }
            BinaryOpr::Assign(opr) => {
                let lopd_ty = self.infer_new_expr_ty(lopd, ExprTypeExpectation::None);
                let ropd_ty = self.infer_new_expr_ty(ropd, ExprTypeExpectation::None);
                let Some(lopd_ty) = lopd_ty
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
                    };
                let Some(ropd_ty) = ropd_ty
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
                    };
                let lopd_ty = match lopd_ty {
                    LocalTerm::Resolved(lopd_ty) => match lopd_ty.term() {
                        Term::Application(lopd_ty) => todo!(),
                        _ => todo!(),
                    },
                    LocalTerm::Unresolved(_) => todo!(),
                };
                let ropd_ty = match ropd_ty {
                    LocalTerm::Resolved(ropd_ty) => self.db.intrinsic_ty(ropd_ty).reduced_term(),
                    LocalTerm::Unresolved(_) => todo!(),
                };
                match opr {
                    Some(opr) => match opr {
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
                    },
                    None => todo!(),
                }
                Ok(self.reduced_term_menu.unit().into())
            }
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => {
                let Some(lopd_ty) = self.infer_new_expr_ty_resolved(lopd, ExprTypeExpectation::TypeType)
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
                    };
                let Some(ropd_ty) = self.infer_new_expr_ty_resolved(ropd, ExprTypeExpectation::TypeType)
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
                    .reduced_term(
                        TermCurry::new(self.db, /* ad hoc */ Variance::Invariant, x, y).into(),
                    )
                    .into())
            }
            BinaryOpr::As => {
                self.infer_new_expr_ty_resolved(ropd, ExprTypeExpectation::TypeType);
                let Some(ropd_term) = self.infer_new_expr_term(ropd)
                    else {
                        return Err(DerivedExprTypeError::AsOperationRightOperandTermNotInferred.into())
                    };
                let Some(lopd_ty) = self.infer_new_expr_ty_resolved(lopd, ExprTypeExpectation::ImplicitlyConvertibleTo{
                    ty: todo!()
                })
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
                    };
                todo!()
            }
            BinaryOpr::Is => {
                let Some(ropd_ty) = self.infer_new_expr_ty_resolved(ropd, ExprTypeExpectation::None)
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
                    };
                let mut ropd_expectation = ExprTypeExpectation::None;
                match ropd_ty.term() {
                    Term::Entity(path) if path == self.entity_path_menu.trai_ty().into() => {
                        todo!()
                    }
                    Term::Category(_) => {
                        if let Some(ropd_term) = self.infer_new_expr_term(ropd) {
                            ropd_expectation =
                                ExprTypeExpectation::ImplicitlyConvertibleTo { ty: ropd_term }
                        }
                    }
                    _ => todo!(),
                }
                Ok(self.reduced_term_menu.prop().into())
            }
            BinaryOpr::In => todo!(),
        }
    }
}
