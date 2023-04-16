use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_closed_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryClosedOpr,
        menu: &TermMenu,
    ) -> Result<FluffyTerm, ExprTypeError> {
        let Some(lopd_ty) = self.infer_new_expr_ty(
            lopd,
            ExpectAnyOriginal,
        ) else {
            self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived);
            Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred)?
        };
        self.infer_new_expr_ty_discarded(
            ropd,
            ExpectImplicitlyConvertible::new_pure(self, lopd_ty),
        );
        match lopd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path: Right(PreludeTypePath::Num(_)),
                ..
            }
            | FluffyTermData::PlaceTypeOntology {
                path,
                refined_path: Right(PreludeTypePath::Num(_)),
                ..
            } => Ok(TermEntityPath::TypeOntology(path).into()),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
                ..
            } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}
