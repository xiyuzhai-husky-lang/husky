use husky_fly_term::quary::FlyQuary;

use super::*;

impl<'a> SemaExprBuilder<'a> {
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
        //     .fly_term_region
        //     .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        let (lopd_sem_expr_idx, lopd_ty) = self.build_sem_expr_with_ty(lopd, ExpectAnyOriginal);
        let ropd_sem_expr_idx = match lopd_ty {
            Some(lopd_ty) => {
                let lopd_base_ty = lopd_ty.base_ty_data(self);
                match lopd_ty.quary() {
                    Some(lopd_place) => match lopd_place {
                        FlyQuary::Const => {
                            // ad hoc
                            // should return err
                            ()
                        }
                        FlyQuary::StackPure { place } => todo!(),
                        FlyQuary::ImmutableOnStack { place } => todo!(),
                        FlyQuary::MutableOnStack { .. } => (),
                        FlyQuary::Transient => {
                            // ad hoc
                            // should return err
                            ()
                        }
                        FlyQuary::Ref { guard } => todo!(),
                        FlyQuary::RefMut { .. } => todo!(),
                        FlyQuary::Leashed { .. } => todo!(),
                        FlyQuary::Todo => todo!(),
                        FlyQuary::EtherealSymbol(_) => todo!(),
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
                        parameter_hvar,
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
                    FlyBaseTypeData::SymbolicVariable {
                        symbolic_variable: symbol,
                    } => todo!(),
                    FlyBaseTypeData::LambdaVariable {
                        lambda_variable: hvar,
                    } => todo!(),
                };
                self.build_sem_expr(ropd, ExpectCoersion::new(Contract::Move, ropd_ty_expected))
            }
            None => self.build_sem_expr(ropd, ExpectAnyDerived),
        };
        (
            lopd_sem_expr_idx,
            SemaBinaryOpr::AssignClosed(opr),
            ropd_sem_expr_idx,
            Ok(SemaBinaryOprDynamicDispatch::builtin()),
            Ok(self.term_menu().unit_ty_ontology().into()),
        )
    }
}
