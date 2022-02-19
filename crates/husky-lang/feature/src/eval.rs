mod id;
mod impl_expr;
mod impl_stmt;
mod indicator;

pub(crate) use id::FeatureEvalId;
pub use indicator::FeatureEvalIndicator;

use vm::{AnyValueDyn, Instruction, VMResult, VMStack};

use crate::{sheet::FeatureSheet, *};
use vm::{Conditional, EvalValue, StackValue};

pub fn eval_feature_block<'eval>(
    block: &FeatureBlock,
    input: Arc<dyn AnyValueDyn>,
    sheet: &mut FeatureSheet<'eval>,
    indicator: &mut FeatureEvalIndicator,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator {
        input,
        sheet,
        indicator,
    };
    evaluator.eval_feature_block(block)
}

pub fn eval_feature_stmt<'eval>(
    stmt: &FeatureStmt,
    input: Arc<dyn AnyValueDyn>,
    sheet: &mut FeatureSheet<'eval>,
    indicator: &mut FeatureEvalIndicator,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator {
        input,
        sheet,
        indicator,
    };
    evaluator.eval_feature_stmt(stmt)
}

pub fn eval_feature_expr<'eval>(
    expr: &FeatureExpr,
    input: Arc<dyn AnyValueDyn>,
    sheet: &mut FeatureSheet<'eval>,
    indicator: &mut FeatureEvalIndicator,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator {
        input,
        sheet,
        indicator,
    };
    evaluator.eval_feature_expr(expr)
}

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    input: Arc<dyn AnyValueDyn>,
    sheet: &'a mut FeatureSheet<'eval>,
    indicator: &'a mut FeatureEvalIndicator,
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    fn eval_feature_block(&mut self, block: &FeatureBlock) -> EvalValue<'eval, 'eval> {
        self.cache(block.feature, |this: &mut Self| {
            this.indicator.set(block.eval_id);
            for stmt in block.stmts.iter() {
                let value = this.eval_feature_stmt(stmt)?;
                match value {
                    Conditional::Defined(_) => return Ok(value),
                    Conditional::Undefined => (),
                }
            }
            Ok(Conditional::Undefined)
        })
    }

    fn eval_func_call(
        &mut self,
        instrns: &[Instruction],
        maybe_compiled: Option<()>,
        inputs: &[Arc<FeatureExpr>],
    ) -> EvalValue<'eval, 'eval> {
        let values: Vec<StackValue<'eval, 'eval>> = inputs
            .iter()
            .map(|expr| self.eval_feature_expr(expr)?.defined())
            .collect::<VMResult<_>>()?;
        let mut stack = VMStack::new(values);
        if let Some(compiled) = maybe_compiled {
            todo!()
        } else {
            match stack.exec_all(instrns)? {
                vm::ControlSignal::Normal | vm::ControlSignal::Break => panic!(),
                vm::ControlSignal::Return(value) => Ok(Conditional::Defined(value)),
            }
        }
    }

    fn cache(
        &mut self,
        feature: FeaturePtr,
        compute_value: impl FnOnce(&mut Self) -> EvalValue<'eval, 'eval>,
    ) -> EvalValue<'eval, 'eval> {
        if let Some(value) = self.sheet.cached_value(feature) {
            value
        } else {
            let value = compute_value(self);
            self.sheet.cache(feature, value)
        }
    }
}
