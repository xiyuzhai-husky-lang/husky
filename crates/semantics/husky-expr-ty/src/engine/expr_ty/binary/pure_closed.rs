use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_closed_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryClosedOpr,
        menu: &EtherealTermMenu,
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
                refined_path: Left(PreludeTypePath::Num(_)),
                ..
            }
            | FluffyTermData::PlaceTypeOntology {
                ty_path: path,
                refined_ty_path: Left(PreludeTypePath::Num(_)),
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
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(hole_kind, _) | FluffyTermData::PlaceHole { hole_kind, .. } => {
                match hole_kind {
                    HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => {
                        Ok(lopd_ty)
                    }
                    HoleKind::ImplicitType => todo!(),
                }
            }
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}
