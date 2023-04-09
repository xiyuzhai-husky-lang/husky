use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_closed_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryClosedOpr,
        menu: &TermMenu,
    ) -> Result<FluffyTerm, ExprTypeError> {
        // todo: don't use resolved
        let Some(lopd_ty) = self.infer_new_expr_ty(
            lopd,
            ExpectAnyOriginal,
        ) else {
            Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred)?
        };
        todo!()
        // let lopd_ty_unravelled = lopd_ty.unravel_borrow(self);
        // match lopd_ty_unravelled.pattern(self) {
        //     FluffyTermData::TypeOntology {
        //         refined_path: Right(PreludeTypePath::Num(_)),
        //         ..
        //     }
        //     | FluffyTermData::Hole(
        //         HoleKind::UnspecifiedIntegerType
        //         | HoleKind::UnspecifiedFloatType,
        //         _,
        //     ) => {
        //         self.infer_new_expr_ty(
        //             ropd,
        //             ExpectImplicitlyConvertible::new_ad_hoc(lopd_ty_unravelled),
        //         );
        //         Ok(lopd_ty_unravelled)
        //     }
        //     FluffyTermData::TypeOntology { .. }
        //     | FluffyTermData::Hole(_, _)
        //     | FluffyTermData::Literal(_)
        //     | FluffyTermData::Curry { .. }
        //     | FluffyTermData::Category(_) => todo!(),
        //     FluffyTermData::Ritchie { .. } => todo!(),
        // }
    }
}
