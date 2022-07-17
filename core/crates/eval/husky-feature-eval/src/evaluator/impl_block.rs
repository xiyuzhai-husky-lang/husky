use super::*;
use crate::*;
use print_utils::msg_once;
use vm::eval_fast;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_feature_lazy_block(
        &mut self,
        block: &FeatureLazyBlock,
    ) -> __EvalValueResult<'eval> {
        self.cache(EvalKey::Feature(block.feature), |this: &mut Self| {
            for stmt in block.stmts.iter() {
                let value = this.eval_stmt(stmt)?;
                match value {
                    __EvalValue::Unreturned => (),
                    _ => return Ok(value),
                }
            }
            Ok(__EvalValue::Unreturned)
        })
    }

    pub(crate) fn eval_feature_func_block(
        &mut self,
        block: &FeatureFuncBlock,
    ) -> __EvalValueResult<'eval> {
        let arguments = match block.opt_this {
            Some(ref this_repr) => {
                vec![self.eval_feature_repr(this_repr)?.into_stack()]
            }
            None => vec![],
        };
        msg_once!("kwargs");
        eval_fast(
            self.db.upcast(),
            unsafe { self.some_ctx() },
            Some(&block.instruction_sheet),
            None,
            block.ty.route,
            arguments.into_iter(),
            [].into_iter(),
            &self.evaluator_config.vm,
        )
    }
}
