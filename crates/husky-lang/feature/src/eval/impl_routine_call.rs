use vm::{eval_fast, InstructionSheet, StackValueSnapshot};

use super::*;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_routine_call(
        &mut self,
        instrns: &InstructionSheet,
        maybe_compiled: Option<()>,
        inputs: &[Arc<LazyExpr>],
    ) -> EvalResult<'eval> {
        let values = inputs
            .iter()
            .map(|expr| StackValue::from_eval(self.eval_lazy_expr(expr)?));
        eval_fast(values, instrns, maybe_compiled)
    }
}
