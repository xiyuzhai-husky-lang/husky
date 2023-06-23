use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_shift_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryShiftOpr,
        menu: &EtherealTermMenu,
    ) -> ExprTypeResult<FluffyTerm> {
        // todo: don't use resolved
        let Some(lopd_ty) = self.infer_new_expr_ty(
            lopd,
            ExpectAnyOriginal,
        ) else {
            Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred)?
        };
        match lopd_ty.data(self) {
            FluffyTermData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Num(_)),
                ..
            }
            | FluffyTermData::Hole(
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType,
                _,
            ) => {
                self.calc_num_ty_binary_shift_ropd_ty(ropd)?;
                Ok(lopd_ty)
            }
            FluffyTermData::PlaceTypeOntology {
                place,
                ty_path: path,
                refined_ty_path: Left(PreludeTypePath::Num(_)),
                ..
            } => {
                self.calc_num_ty_binary_shift_ropd_ty(ropd)?;
                Ok(TermEntityPath::TypeOntology(path).into())
            }
            FluffyTermData::TypeOntology { .. }
            | FluffyTermData::Hole(_, _)
            | FluffyTermData::Literal(_)
            | FluffyTermData::Curry { .. }
            | FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }

    pub(super) fn calc_num_ty_binary_shift_ropd_ty(&mut self, ropd: ExprIdx) -> ExprTypeResult<()> {
        let Some(ropd_ty) = self.infer_new_expr_ty(ropd, ExpectAnyOriginal) else {
            Err(DerivedExprTypeError::BinaryShiftRightOperandTypeNotInferred)?
        };
        match ropd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
                ..
            }
            | FluffyTermData::PlaceTypeOntology {
                refined_ty_path: Left(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
                ..
            }
            | FluffyTermData::Hole(HoleKind::UnspecifiedIntegerType, _) => Ok(()),
            FluffyTermData::TypeOntology { .. } => todo!(),
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}
