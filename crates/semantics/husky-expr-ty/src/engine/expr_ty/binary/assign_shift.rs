use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_shift_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryShiftOpr,
        menu: &TermMenu,
    ) -> Result<LocalTerm, ExprTypeError> {
        // todo: don't use resolved
        let Some(lopd_ty) = self.infer_new_expr_ty(
            lopd, ExpectAnyOriginal,  
        ) else {
            return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
        };
        let lopd_ty_unravelled =
            lopd_ty.unravel_borrow(self.db,self. local_term_region.unresolved_terms());
        match lopd_ty_unravelled.pattern(self) {
            LocalTermPattern::TypeOntology {
                refined_path: Right(PreludeTypePath::Num(_)),
                ..
            }
            | LocalTermPattern::ImplicitSymbol(
                ImplicitSymbolKind::UnspecifiedIntegerType
                | ImplicitSymbolKind::UnspecifiedFloatType,
                _,
            ) => {
                if let Some(ropd_ty) =
                    self.infer_new_expr_ty(ropd, ExpectAnyOriginal,  )
                {
                    match ropd_ty.pattern(self) {
                        LocalTermPattern::Literal(_) => todo!(),
                        LocalTermPattern::TypeOntology {
                            refined_path: Right(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
                            ..
                        }
                        | LocalTermPattern::ImplicitSymbol(
                            ImplicitSymbolKind::UnspecifiedIntegerType,
                            _,
                        ) => {}
                        LocalTermPattern::TypeOntology { .. } => todo!(),
                        LocalTermPattern::Curry { .. } => todo!(),
                        LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
                        LocalTermPattern::Category(_) => todo!(),
                        LocalTermPattern::Ritchie { .. } => todo!(),
                    }
                }
                Ok(lopd_ty_unravelled)
            }
            LocalTermPattern::TypeOntology { .. }
            | LocalTermPattern::ImplicitSymbol(_, _)
            | LocalTermPattern::Literal(_)
            | LocalTermPattern::Curry { .. }
            | LocalTermPattern::Category(_) => todo!(),
            LocalTermPattern::Ritchie { .. } => todo!(),
        }
    }
}
