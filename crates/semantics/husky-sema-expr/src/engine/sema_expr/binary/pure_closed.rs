use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_binary_closed_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
        opr: BinaryClosedOpr,
        menu: &EtherealTermMenu,
    ) -> (
        SemaExprIdx,
        SemaExprIdx,
        SemaExprDataResult<BinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (lopd, lopd_ty) = self.build_sema_expr_with_its_ty_returned(lopd, ExpectAnyOriginal);
        let Some(lopd_ty) = lopd_ty else {
            let ropd = self.build_sema_expr(ropd, ExpectAnyDerived);
            return (
                lopd,
                ropd,
                todo!(),
                Err(DerivedSemaExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into()),
            );
        };
        let ropd = self.build_sema_expr(ropd, ExpectCoersion::new_pure(self, lopd_ty));
        let ty_result = match lopd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path: Left(PreludeTypePath::Num(_)),
                ..
            } => Ok(TermEntityPath::TypeOntology(ty_path).into()),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ..
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => Ok(lopd_ty),
                HoleKind::ImplicitType => todo!(),
                HoleKind::Any => todo!(),
            },
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        };
        (
            lopd,
            ropd,
            Ok(BinaryOprDynamicDispatch::builtin()),
            ty_result,
        )
    }
}
