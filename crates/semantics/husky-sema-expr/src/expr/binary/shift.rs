use super::*;

impl<'a> SemaExprBuilder<'a> {
    pub(super) fn calc_binary_shift_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
        opr: BinaryShiftOpr,
        menu: &EthTermMenu,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        // todo: don't use resolved
        let (lopd_sema_expr_idx, lopd_ty) = self.build_sema_expr_with_ty(lopd, ExpectAnyOriginal);
        let Some(lopd_ty) = lopd_ty else {
            match self.syn_expr_region_data()[lopd] {
                SynExprData::CurrentSynSymbol {
                    ident,
                    regional_token_idx,
                    current_syn_symbol_idx,
                    current_syn_symbol_kind,
                } => {
                    todo!()
                }
                _ => todo!(),
            }
            let ropd_sema_expr_idx = self.build_sema_expr(ropd, ExpectAnyDerived);
            return (
                lopd_sema_expr_idx,
                SemaBinaryOpr::Shift(opr),
                ropd_sema_expr_idx,
                todo!(),
                Err(DerivedSemaExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into()),
            );
        };
        match lopd_ty.data(self) {
            FlyTermData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Num(_)),
                ..
            }
            | FlyTermData::Hole(HoleKind::UnspecifiedIntegerType, _) => {
                let ropd_sema_expr_idx = self.build_sema_expr(ropd, ExpectIntType);
                (
                    lopd_sema_expr_idx,
                    SemaBinaryOpr::Shift(opr),
                    ropd_sema_expr_idx,
                    Ok(SemaBinaryOprDynamicDispatch::builtin()),
                    Ok(lopd_ty),
                )
            }
            FlyTermData::Hole(HoleKind::UnspecifiedFloatType, _) => todo!(),
            // FlyTermData::TypeOntologyAtPlace {
            //     place,
            //     ty_path: path,
            //     refined_ty_path: Left(PreludeTypePath::Num(_)),
            //     ..
            // } => {
            //     self.calc_num_ty_binary_shift_ropd_ty(ropd)?;
            //     Ok(TermEntityPath::TypeOntology(path).into())
            // }
            FlyTermData::TypeOntology { .. }
            | FlyTermData::Hole(_, _)
            | FlyTermData::Literal(_)
            | FlyTermData::Curry { .. }
            | FlyTermData::Sort(_) => todo!(),
            FlyTermData::Ritchie { .. } => todo!(),
            FlyTermData::Symbol { .. } => todo!(),
            FlyTermData::Hvar { .. } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
        }
    }

    // pub(super) fn calc_num_ty_binary_shift_ropd_ty(
    //     &mut self,
    //     ropd: SynExprIdx,
    // ) -> SemaExprTypeResult<()> {
    //     match ropd_ty.data(self) {
    //         FlyTermData::Literal(_) => todo!(),
    //         FlyTermData::TypeOntology {
    //             refined_ty_path: Left(PreludeTypePath::Num(PreludeNumTypePath::Int(_))),
    //             ..
    //         }
    //         | FlyTermData::Hole(HoleKind::UnspecifiedIntegerType, _) => Ok(()),
    //         FlyTermData::TypeOntology { .. } => todo!(),
    //         FlyTermData::Curry { .. } => todo!(),
    //         FlyTermData::Hole(_, _) => todo!(),
    //         FlyTermData::Category(_) => todo!(),
    //         FlyTermData::Ritchie { .. } => todo!(),
    //         FlyTermData::Symbol { .. } => todo!(),
    //         FlyTermData::Variable { ty } => todo!(),
    //         FlyTermData::TypeVariant { path } => todo!(),
    //     }
    // }
}
