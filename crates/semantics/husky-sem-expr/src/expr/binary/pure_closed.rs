use super::*;

impl<'a> SemaExprBuilder<'a> {
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
        let lopd_syn_expr_idx = lopd;
        let (lopd, lopd_ty) = self.build_sem_expr_with_ty(lopd, ExpectAnyOriginal);
        let Some(lopd_ty) = lopd_ty else {
            use husky_print_utils::p;

            p!(self.syn_expr_region_data()[lopd_syn_expr_idx].debug(self.db()));
            p!(self.sem_expr_arena()[lopd].debug(self.db()));
            todo!();
            let ropd = self.build_sem_expr(ropd, ExpectAnyDerived);
            return (
                lopd,
                SemaBinaryOpr::Closed(opr),
                ropd,
                todo!(),
                Err(DerivedSemaExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into()),
            );
        };
        let ropd = self.build_sem_expr(ropd, ExpectCoercion::new_pure(self, lopd_ty));
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
                parameter_hvar,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => Ok(lopd_ty),
                HoleKind::ImplicitType => todo!(),
                HoleKind::AnyOriginal => todo!(),
                HoleKind::AnyDerived => todo!(),
            },
            FlyTermData::Sort(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FlyTermData::SymbolicVariable { .. } => todo!(),
            FlyTermData::LambdaVariable { .. } => todo!(),
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
