use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_shift_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
        opr: BinaryShiftOpr,
        menu: &EtherealTermMenu,
    ) -> SemaExprResult<FluffyTerm> {
        // todo: don't use resolved
        let Some(lopd_ty) = self.infer_new_expr_ty(lopd, ExpectAnyOriginal) else {
            p!(self.path());
            p!(self.expr_region_data[lopd].debug(self.db));
            p!(self.symbol_tys.debug(self.db));
            todo!();
            match self.expr_region_data[lopd] {
                SynExprData::CurrentSymbol {
                    ident,
                    regional_token_idx,
                    current_symbol_idx,
                    current_symbol_kind,
                } => {
                    todo!()
                }
                _ => todo!(),
            }
            self.build_new_expr_ty_discarded(ropd, ExpectAnyDerived);
            Err(DerivedSemaExprError::BinaryOperationLeftOperandTypeNotInferred)?
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
            // FluffyTermData::TypeOntologyAtPlace {
            //     place,
            //     ty_path: path,
            //     refined_ty_path: Left(PreludeTypePath::Num(_)),
            //     ..
            // } => {
            //     self.calc_num_ty_binary_shift_ropd_ty(ropd)?;
            //     Ok(TermEntityPath::TypeOntology(path).into())
            // }
            FluffyTermData::TypeOntology { .. }
            | FluffyTermData::Hole(_, _)
            | FluffyTermData::Literal(_)
            | FluffyTermData::Curry { .. }
            | FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }

    pub(super) fn calc_num_ty_binary_shift_ropd_ty(
        &mut self,
        ropd: SynExprIdx,
    ) -> SemaExprResult<()> {
        let Some(ropd_ty) = self.infer_new_expr_ty(ropd, ExpectAnyOriginal) else {
            Err(DerivedSemaExprError::BinaryShiftRightOperandTypeNotInferred)?
        };
        match ropd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
                ..
            }
            | FluffyTermData::Hole(HoleKind::UnspecifiedIntegerType, _) => Ok(()),
            FluffyTermData::TypeOntology { .. } => todo!(),
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
