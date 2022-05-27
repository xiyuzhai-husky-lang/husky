use super::*;
use crate::*;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_block(
        &mut self,
        block: &FeatureBlock,
        eval_key: EvalKey<'eval>,
    ) -> EvalResult<'eval> {
        self.cache(eval_key, |this: &mut Self| {
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
}
