use super::*;
use crate::*;
use print_utils::msg_once;
use vm::eval_fast;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_feature_lazy_block(
        &mut self,
        block: &FeatureLazyBlock,
    ) -> EvalValueResult<'eval> {
        self.cache(EvalKey::Feature(block.feature), |this: &mut Self| {
            for stmt in block.stmts.iter() {
                let value = this.eval_feature_stmt(stmt)?;
                match value {
                    EvalValue::Undefined => (),
                    _ => return Ok(value),
                }
            }
            Ok(EvalValue::Undefined)
        })
    }

    pub(crate) fn eval_feature_func_block(
        &mut self,
        block: &FeatureFuncBlock,
    ) -> EvalValueResult<'eval> {
        let arguments = match block.opt_this {
            Some(ref this_repr) => {
                vec![self.eval_feature_repr(this_repr)?.into_stack()]
            }
            None => vec![],
        };
        msg_once!("kwargs");
        eval_fast(
            self.db.upcast(),
            Some(&block.instruction_sheet),
            None,
            arguments.into_iter(),
            [].into_iter(),
            self.verbose,
        )
    }
}
