use vm::{eval_fast, InstructionSheet, StackValueSnapshot};

use super::*;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_memb_routine_call(
        &mut self,
        instrns: &InstructionSheet,
        maybe_compiled: Option<()>,
        opds: &[Arc<FeatureExpr>],
    ) -> EvalResult<'eval> {
        let values = opds
            .iter()
            .map(|expr| StackValue::from_eval(self.eval_feature_expr(expr)?));
        eval_fast(values, instrns, maybe_compiled)
    }
}
