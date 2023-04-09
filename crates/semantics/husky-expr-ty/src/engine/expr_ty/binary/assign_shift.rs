use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_shift_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryShiftOpr,
        menu: &TermMenu,
    ) -> Result<FluffyTerm, ExprTypeError> {
        // todo: don't use resolved
        let Some(lopd_ty) = self.infer_new_expr_ty(
            lopd,
            ExpectAnyOriginal,
        ) else {
            Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred)?
        };
        let lopd_ty_unravelled = lopd_ty.unravel_borrow(self);
        match lopd_ty_unravelled.data(self) {
            FluffyTermData::TypeOntology {
                refined_path: Right(PreludeTypePath::Num(_)),
                ..
            }
            | FluffyTermData::Hole(
                ImplicitSymbolKind::UnspecifiedIntegerType
                | ImplicitSymbolKind::UnspecifiedFloatType,
                _,
            ) => {
                if let Some(ropd_ty) = self.infer_new_expr_ty(ropd, ExpectAnyOriginal) {
                    match ropd_ty.data(self) {
                        FluffyTermData::Literal(_) => todo!(),
                        FluffyTermData::TypeOntology {
                            refined_path: Right(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
                            ..
                        }
                        | FluffyTermData::Hole(ImplicitSymbolKind::UnspecifiedIntegerType, _) => {}
                        FluffyTermData::TypeOntology { .. } => todo!(),
                        FluffyTermData::Curry { .. } => todo!(),
                        FluffyTermData::Hole(_, _) => todo!(),
                        FluffyTermData::Category(_) => todo!(),
                        FluffyTermData::Ritchie { .. } => todo!(),
                    }
                }
                Ok(lopd_ty_unravelled)
            }
            FluffyTermData::TypeOntology { .. }
            | FluffyTermData::Hole(_, _)
            | FluffyTermData::Literal(_)
            | FluffyTermData::Curry { .. }
            | FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
        }
    }
}
