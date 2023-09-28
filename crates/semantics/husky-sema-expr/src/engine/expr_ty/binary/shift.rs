use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_binary_shift_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
        opr: BinaryShiftOpr,
        menu: &EtherealTermMenu,
    ) -> (SemaExprIdx, SemaExprIdx, SemaExprTypeResult<FluffyTerm>) {
        // todo: don't use resolved
        let (lopd_sema_expr_idx, lopd_ty) =
            self.build_sema_expr_with_its_ty_returned(lopd, ExpectAnyOriginal);
        let Some(lopd_ty) = lopd_ty else {
            p!(self.path());
            p!(self.syn_expr_region_data[lopd].debug(self.db));
            p!(self.symbol_tys.debug(self.db));
            todo!();
            match self.syn_expr_region_data[lopd] {
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
            let ropd_sema_expr_idx = self.build_sema_expr(ropd, ExpectAnyDerived);
            return (
                lopd_sema_expr_idx,
                ropd_sema_expr_idx,
                Err(DerivedSemaExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into()),
            );
        };
        match lopd_ty.data(self) {
            FluffyTermData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Num(_)),
                ..
            }
            | FluffyTermData::Hole(HoleKind::UnspecifiedIntegerType, _) => {
                let ropd_sema_expr_idx = self.build_sema_expr(ropd, ExpectIntType);
                (lopd_sema_expr_idx, ropd_sema_expr_idx, Ok(lopd_ty))
            }
            FluffyTermData::Hole(HoleKind::UnspecifiedFloatType, _) => todo!(),
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

    // pub(super) fn calc_num_ty_binary_shift_ropd_ty(
    //     &mut self,
    //     ropd: SynExprIdx,
    // ) -> SemaExprTypeResult<()> {
    //     match ropd_ty.data(self) {
    //         FluffyTermData::Literal(_) => todo!(),
    //         FluffyTermData::TypeOntology {
    //             refined_ty_path: Left(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
    //             ..
    //         }
    //         | FluffyTermData::Hole(HoleKind::UnspecifiedIntegerType, _) => Ok(()),
    //         FluffyTermData::TypeOntology { .. } => todo!(),
    //         FluffyTermData::Curry { .. } => todo!(),
    //         FluffyTermData::Hole(_, _) => todo!(),
    //         FluffyTermData::Category(_) => todo!(),
    //         FluffyTermData::Ritchie { .. } => todo!(),
    //         FluffyTermData::Symbol { .. } => todo!(),
    //         FluffyTermData::Variable { ty } => todo!(),
    //         FluffyTermData::TypeVariant { path } => todo!(),
    //     }
    // }
}
