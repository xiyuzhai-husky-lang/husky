use husky_fly_term::{dispatch::HasFlyMethodDispatch, quary::FlyQuary};

use super::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_binary_assign_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
    ) -> (
        SemExprIdx,
        SemaBinaryOpr,
        SemExprIdx,
        SemExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemExprTypeResult<FlyTerm>,
    ) {
        let (lopd_sem_expr_idx, lopd_ty) = self.build_sem_expr_with_ty(lopd, ExpectAnyOriginal);
        let ropd_sem_expr_idx = match lopd_ty {
            Some(lopd_ty) => {
                match lopd_ty.quary() {
                    Some(quary) => match quary {
                        FlyQuary::Compterm => todo!(),
                        FlyQuary::StackPure { place } => todo!(),
                        FlyQuary::ImmutableOnStack { place } => todo!(),
                        FlyQuary::MutableOnStack { .. } => (),
                        FlyQuary::Transient => todo!(),
                        FlyQuary::Ref { guard } => todo!(),
                        FlyQuary::RefMut { place, lifetime } => todo!(),
                        FlyQuary::Leashed { .. } => todo!(),
                        FlyQuary::Todo => todo!(),
                        FlyQuary::EtherealSymbol(_) => todo!(),
                    },
                    None => todo!(),
                }
                self.build_sem_expr(
                    ropd,
                    ExpectCoercion::new_move(lopd_ty.with_quary(FlyQuary::Transient)),
                )
            }
            None => self.build_sem_expr(ropd, ExpectAnyDerived),
        };
        (
            lopd_sem_expr_idx,
            SemaBinaryOpr::Assign,
            ropd_sem_expr_idx,
            Ok(SemaBinaryOprDynamicDispatch::builtin()),
            Ok(self.term_menu().unit_ty_ontology().into()),
        )
    }
}
