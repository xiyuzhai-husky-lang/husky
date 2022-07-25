use super::*;
use crate::*;
use husky_print_utils::msg_once;
use vm::{__RegisterDataKind, eval_fast};

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_feature_lazy_block(
        &mut self,
        block: &FeatureLazyBlock,
    ) -> __VMResult<__Register<'eval>> {
        self.cache(EvalKey::Feature(block.feature), |this: &mut Self| {
            for stmt in block.stmts.iter() {
                let value = this.eval_stmt(stmt)?;
                match value.data_kind {
                    __RegisterDataKind::Unreturned => (),
                    _ => return Ok(value),
                }
            }
            Ok(__Register::new_undefined())
        })
    }

    pub(crate) fn eval_feature_func_block(
        &mut self,
        block: &FeatureFuncBlock,
    ) -> __VMResult<__Register<'eval>> {
        let arguments = match block.opt_this {
            Some(ref this_repr) => {
                vec![self.eval_feature_repr(this_repr)?.stack()]
            }
            None => vec![],
        };
        let nargs: u8 = arguments.len().try_into().unwrap();
        msg_once!("kwargs");
        todo!()
        // eval_fast(
        //     self.db.upcast(),
        //     unsafe { self.some_ctx() },
        //     Some(&block.instruction_sheet),
        //     block.opt_linkage,
        //     block.ty.route,
        //     arguments.into_iter(),
        //     [].into_iter(),
        //     nargs,
        //     &self.evaluator_config.vm,
        // )
    }
}
