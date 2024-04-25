use husky_eth_term::term::{
    application::EthApplication, symbolic_variable::EthSymbolicVariable, EthTerm,
};
use husky_fly_term::{
    dispatch::StaticDispatch, instantiation::FlyInstantiation,
    signature::binary_opr::SemaBinaryOprFlySignature,
};
use husky_term_prelude::literal::{
    float::{F32Literal, F64Literal},
    int::{
        I128Literal, I64Literal, ISizeLiteral, R128Literal, R64Literal, RSizeLiteral, U128Literal,
        U64Literal, USizeLiteral,
    },
    Literal,
};
use husky_token_data::{BoolLiteralTokenData, FloatLiteralTokenData};

use super::*;

impl<'a> SemaExprBuilder<'a> {
    /// perform this during finish stage
    pub(crate) fn infer_expr_term(&mut self, expr: SemaExprIdx) -> Option<FlyTerm> {
        if let Some(term_result) = self.sem_expr_term_results.get_value(expr) {
            return term_result.as_ref().ok().copied();
        }
        let term_result = self.calc_expr_term(expr);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(expr, term_result);
        term
    }

    /// clear all holes before using this
    pub(super) fn infer_extra_expr_terms_in_preparation_for_hir(&mut self) {
        for sem_expr_idx in self.sem_expr_arena.index_iter() {
            self.infer_extra_expr_term_in_preparation_for_hir(sem_expr_idx)
        }
    }

    // helpful for hir stage
    fn infer_extra_expr_term_in_preparation_for_hir(&mut self, sem_expr_idx: SemaExprIdx) {
        if let Some(_) = self.sem_expr_term_results.get_value(sem_expr_idx) {
            return;
        }
        // ad hoc
        match sem_expr_idx.data_result(&self.sem_expr_arena) {
            Ok(SemaExprData::Literal(_, _)) => (),
            _ => return,
        }
        let term_result = self.calc_expr_term(sem_expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(sem_expr_idx, term_result)
    }

    fn save_new_expr_term(
        &mut self,
        expr_idx: SemaExprIdx,
        term_result: SemaExprTermResult<FlyTerm>,
    ) {
        self.sem_expr_term_results
            .insert_new((expr_idx, term_result))
            .expect("todo")
    }
}
