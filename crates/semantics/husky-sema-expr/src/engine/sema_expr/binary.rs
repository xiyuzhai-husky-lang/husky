mod assign;
mod assign_closed;
mod assign_shift;
mod comparison;
mod pure_closed;
mod shift;

use husky_fluffy_term::{
    dispatch::dynamic_dispatch::binary_opr::SemaBinaryOprDynamicDispatch,
    signature::binary_opr::SemaBinaryOprFluffySignature,
};
use husky_sema_opr::binary::SemaBinaryOpr;
use husky_syn_opr::SynBinaryOpr;

use super::*;

impl<'a> SemaExprEngine<'a> {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub(super) fn calc_binary_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        lopd: SynExprIdx,
        opr: SynBinaryOpr,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let menu = self.term_menu;
        match opr {
            SynBinaryOpr::Closed(opr) => self.calc_binary_closed_expr_ty(lopd, ropd, opr, menu),
            SynBinaryOpr::Shift(opr) => self.calc_binary_shift_expr_ty(lopd, ropd, opr, menu),
            SynBinaryOpr::Comparison(opr) => self.calc_binary_comparison_expr_ty(lopd, opr, ropd),
            SynBinaryOpr::ShortCircuitLogic(opr) => {
                self.calc_binary_short_circuit_logic_expr_ty(lopd, opr, ropd)
            }
            SynBinaryOpr::Assign => self.calc_binary_assign_expr_ty(expr_idx, lopd, ropd),
            SynBinaryOpr::AssignClosed(opr) => {
                self.calc_binary_assign_closed_expr_ty(expr_idx, lopd, opr, ropd)
            }
            SynBinaryOpr::AssignShift(opr) => {
                self.calc_binary_assign_shift_expr_ty(expr_idx, lopd, opr, ropd)
            }
            SynBinaryOpr::ScopeResolution => {
                todo!()
                // Err(OriginalSemaExprTypeError::TodoScopeResolution.into())
            }
            SynBinaryOpr::CurryType => self.calc_curry_expr_ty(lopd, ropd),
            SynBinaryOpr::As => self.calc_as_expr_ty(lopd, ropd),
            SynBinaryOpr::Ins => self.calc_ins_sema_expr(lopd, ropd),
            SynBinaryOpr::In => todo!(),
        }
    }

    fn calc_binary_short_circuit_logic_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        opr: BinaryShortcuitLogicOpr,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        // todo: indirections
        let lopd_sema_expr_idx = self.build_sema_expr(lopd, self.expect_argument_ty_bool());
        let ropd_sema_expr_idx = self.build_sema_expr(ropd, self.expect_argument_ty_bool());
        (
            lopd_sema_expr_idx,
            SemaBinaryOpr::ShortCircuitLogic(opr),
            ropd_sema_expr_idx,
            Ok(SemaBinaryOprDynamicDispatch::builtin()),
            Ok(self.term_menu.bool_ty_ontology().into()),
        )
    }

    fn calc_ins_sema_expr(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (ropd_sema_expr_idx, ropd_ty) = self.build_sema_expr_with_ty(ropd, ExpectAnyOriginal);
        let Some(ropd_ty) = ropd_ty else {
            let lopd_sema_expr_idx = self.build_sema_expr(lopd, ExpectAnyDerived);
            return (
                lopd_sema_expr_idx,
                SemaBinaryOpr::Ins,
                ropd_sema_expr_idx,
                todo!(),
                Err(DerivedSemaExprTypeError::BinaryOperationRightOperandTypeNotInferred.into()),
            );
        };
        let lopd_sema_expr_idx = todo!();
        // todo
        // match ropd_ty {
        //     EtherealTerm::Entity(path) if path == self.item_path_menu.trai_ty().into() => {
        //         todo!()
        //     }
        //     EtherealTerm::Category(_) => {
        //         todo!()
        //         // if let Some(ropd_term) = self.infer_new_expr_term(ropd) {
        //         //     ropd_expectation = ExpectImplicitConversion {
        //         //         destination: ropd_term,
        //         //     }
        //         // }
        //     }
        //     _ => todo!(),
        // }
        (
            lopd_sema_expr_idx,
            SemaBinaryOpr::Ins,
            ropd_sema_expr_idx,
            todo!(),
            Ok(self.term_menu.prop().into()),
        )
    }

    fn calc_as_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let ropd_sema_expr_idx = self.build_sema_expr(ropd, ExpectEqsCategory::new_any_sort());
        let Some(ropd_term) = self.infer_expr_term(ropd_sema_expr_idx) else {
            let lopd_sema_expr_idx = self.build_sema_expr(lopd, ExpectAnyDerived);
            return (
                lopd_sema_expr_idx,
                SemaBinaryOpr::As,
                ropd_sema_expr_idx,
                todo!(),
                Err(DerivedSemaExprTypeError::AsOperationRightOperandTermNotInferred.into()),
            );
        };
        let lopd_sema_expr_idx = self.build_sema_expr(lopd, ExpectCasting::new(ropd_term));
        (
            lopd_sema_expr_idx,
            SemaBinaryOpr::As,
            ropd_sema_expr_idx,
            Ok(SemaBinaryOprDynamicDispatch::builtin()),
            Ok(ropd_term),
        )
    }

    fn calc_curry_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let expect_any_sort = ExpectEqsCategory::new_any_sort();
        let (lopd_sema_expr_idx, lopd_universe) =
            self.build_sema_expr_with_outcome(lopd, expect_any_sort);
        let Some(lopd_universe) = lopd_universe else {
            return (
                lopd_sema_expr_idx,
                SemaBinaryOpr::CurryType,
                todo!(),
                todo!(),
                Err(DerivedSemaExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into()),
            );
        };
        let (ropd_sema_expr_idx, ropd_universe) =
            self.build_sema_expr_with_outcome(ropd, expect_any_sort);
        let Some(ropd_universe) = ropd_universe else {
            return (
                lopd_sema_expr_idx,
                SemaBinaryOpr::CurryType,
                ropd_sema_expr_idx,
                todo!(),
                Err(DerivedSemaExprTypeError::BinaryOperationRightOperandTypeNotInferred.into()),
            );
        };
        todo!()
        // Ok(EtherealTerm::new_category(x_u.max(y_u)).into())
    }

    fn calc_binary_assign_shift_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        lopd: SynExprIdx,
        opr: BinaryShiftOpr,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        todo!()
        // let expr_eval_lifetime = self
        //     .fluffy_term_region
        //     .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        // match self.infer_new_expr_ty_for_outcome(lopd, ExpectAnyOriginal) {
        //     Some(_) => todo!(),
        //     None => {
        //         self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived);
        //     }
        // };
        // Ok(self.term_menu.unit().into())
    }

    fn infer_basic_assign_ropd_ty(&mut self, lopd_ty: FluffyTerm, ropd: SynExprIdx) {
        let (ropd_sema_expr_idx, ropd_ty) = self.build_sema_expr_with_ty(ropd, ExpectAnyOriginal);
        let Some(ropd_ty) = ropd_ty else { return };
        todo!()
        // let lopd_ty = match lopd_ty {
        //     FluffyTerm::EtherealTerm(lopd_ty) => match lopd_ty {
        //         EtherealTerm::Application(lopd_ty) => todo!(),
        //         _ => todo!(),
        //     },
        //     FluffyTerm::Unresolved(lopd_ty) => {
        //         match self.fluffy_term_region[lopd_ty].unresolved_term() {
        //             FluffyTermData::ImplicitSymbol(_) => todo!(),
        //             FluffyTermData::TypeOntology(_) => {
        //                 todo!()
        //             }
        //             FluffyTermData::Ritchie(_) => todo!(),
        //             FluffyTermData::PlaceType { .. } => todo!(),
        //         }
        //     }
        //     _ => todo!(),
        // };
        // let ropd_ty = match ropd_ty {
        //     FluffyTerm::EtherealTerm(ropd_ty) => todo!(),
        //     // self.db.intrinsic_ty(ropd_ty).reduced_term(),
        //     FluffyTerm::Unresolved(_) => todo!(),
        //     _ => todo!(),
        // };
    }

    fn infer_composite_assign_ropd_ty(
        &mut self,
        lopd_ty: FluffyTerm,
        opr: BinaryClosedOpr,
        ropd: SynExprIdx,
    ) {
        match opr {
            BinaryClosedOpr::Add => todo!(),
            BinaryClosedOpr::BitAnd => todo!(),
            BinaryClosedOpr::BitOr => todo!(),
            BinaryClosedOpr::BitXor => todo!(),
            BinaryClosedOpr::Div => todo!(),
            BinaryClosedOpr::Mul => todo!(),
            BinaryClosedOpr::RemEuclid => todo!(),
            BinaryClosedOpr::Power => todo!(),
            BinaryClosedOpr::Sub => todo!(),
        }
    }
}
