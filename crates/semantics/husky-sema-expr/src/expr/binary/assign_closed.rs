use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_binary_assign_closed_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        lopd: SynExprIdx,
        opr: BinaryClosedOpr,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        // let expr_eval_lifetime = self
        //     .fluffy_term_region
        //     .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        let (lopd_sema_expr_idx, lopd_ty) = self.build_sema_expr_with_ty(lopd, ExpectAnyOriginal);
        let ropd_sema_expr_idx = match lopd_ty {
            Some(lopd_ty) => {
                let lopd_base_ty = lopd_ty.base_ty_data(self);
                match lopd_ty.place() {
                    Some(lopd_place) => match lopd_place {
                        FlyPlace::Const => {
                            // ad hoc
                            // should return err
                            ()
                        }
                        FlyPlace::StackPure { location } => todo!(),
                        FlyPlace::ImmutableStackOwned { location } => todo!(),
                        FlyPlace::MutableStackOwned { .. } => (),
                        FlyPlace::Transient => {
                            // ad hoc
                            // should return err
                            ()
                        }
                        FlyPlace::Ref { guard } => todo!(),
                        FlyPlace::RefMut { guard } => todo!(),
                        FlyPlace::Leashed => todo!(),
                        FlyPlace::Todo => todo!(),
                        FlyPlace::EtherealSymbol(_) => todo!(),
                    },
                    // ad hoc
                    None => (), // todo!(),
                };
                let ropd_ty_expected: FlyTerm = match lopd_base_ty {
                    FlyBaseTypeData::TypeOntology {
                        ty_path,
                        refined_ty_path,
                        ty_arguments,
                        ty_ethereal_term,
                    } => match ty_ethereal_term {
                        Some(ty_ethereal_term) => ty_ethereal_term.into(),
                        None => todo!(),
                    },
                    FlyBaseTypeData::Curry {
                        curry_kind,
                        variance,
                        parameter_rune,
                        parameter_ty,
                        return_ty,
                        ty_ethereal_term,
                    } => todo!(),
                    FlyBaseTypeData::Hole(_, _) => lopd_ty, // ad hoc
                    FlyBaseTypeData::Category(_) => todo!(),
                    FlyBaseTypeData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys,
                        return_ty,
                    } => todo!(),
                    FlyBaseTypeData::Symbol { symbol } => todo!(),
                    FlyBaseTypeData::Rune { rune } => todo!(),
                };
                self.build_sema_expr(
                    ropd,
                    ExpectCoersion::new(TermContract::Move, ropd_ty_expected),
                )
            }
            None => self.build_sema_expr(ropd, ExpectAnyDerived),
        };
        (
            lopd_sema_expr_idx,
            SemaBinaryOpr::AssignClosed(opr),
            ropd_sema_expr_idx,
            Ok(SemaBinaryOprDynamicDispatch::builtin()),
            Ok(self.term_menu().unit_ty_ontology().into()),
        )
    }
}
