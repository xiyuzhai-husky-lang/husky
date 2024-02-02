use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_binary_closed_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
        opr: BinaryClosedOpr,
        menu: &EthTermMenu,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        let (lopd, lopd_ty) = self.build_sema_expr_with_ty(lopd, ExpectAnyOriginal);
        let Some(lopd_ty) = lopd_ty else {
            let ropd = self.build_sema_expr(ropd, ExpectAnyDerived);
            return (
                lopd,
                SemaBinaryOpr::Closed(opr),
                ropd,
                todo!(),
                Err(DerivedSemaExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into()),
            );
        };
        let ropd = self.build_sema_expr(ropd, ExpectCoersion::new_pure(self, lopd_ty));
        let ty_result = match lopd_ty.data(self) {
            FlyTermData::Literal(_) => todo!(),
            FlyTermData::TypeOntology {
                ty_path,
                refined_ty_path: Left(PreludeTypePath::Num(_)),
                ..
            } => Ok(ItemPathTerm::TypeOntology(ty_path).into()),
            FlyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ..
            } => todo!(),
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => Ok(lopd_ty),
                HoleKind::ImplicitType => todo!(),
                HoleKind::Any => todo!(),
            },
            FlyTermData::Category(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FlyTermData::Symbol { .. } => todo!(),
            FlyTermData::Rune { .. } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
        };
        (
            lopd,
            SemaBinaryOpr::Closed(opr),
            ropd,
            Ok(SemaBinaryOprDynamicDispatch::builtin()),
            ty_result,
        )
    }
}
