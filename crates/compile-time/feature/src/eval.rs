mod id;
mod impl_block;
mod impl_expr;
mod impl_repr;
mod impl_stmt;
mod indicator;

pub(crate) use id::FeatureEvalId;
pub use indicator::FeatureEvalIndicator;

use vm::{AnyValueDyn, EvalValue};

use crate::{sheet::FeatureSheet, *};
use vm::EvalResult;

pub fn eval_feature_block<'eval>(
    db: &dyn FeatureQueryGroup,
    block: &FeatureBlock,
    input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &mut FeatureSheet<'eval>,
) -> EvalResult<'eval> {
    let mut evaluator = FeatureEvaluator { db, input, sheet };
    evaluator.eval_feature_block(block)
}

pub fn eval_feature_stmt<'eval>(
    db: &dyn FeatureQueryGroup,
    stmt: &FeatureStmt,
    input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &mut FeatureSheet<'eval>,
) -> EvalResult<'eval> {
    let mut evaluator = FeatureEvaluator { db, input, sheet };
    evaluator.eval_feature_stmt(stmt)
}

pub fn eval_feature_expr<'eval>(
    db: &dyn FeatureQueryGroup,
    expr: &FeatureExpr,
    input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &mut FeatureSheet<'eval>,
) -> EvalResult<'eval> {
    let mut evaluator = FeatureEvaluator { db, input, sheet };
    evaluator.eval_feature_expr(expr)
}

pub fn eval_feature_repr<'eval>(
    db: &dyn FeatureQueryGroup,
    repr: &FeatureRepr,
    input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &mut FeatureSheet<'eval>,
) {
    todo!()
}

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &'a mut FeatureSheet<'eval>,
    db: &'a dyn FeatureQueryGroup,
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    fn cache(
        &mut self,
        feature: FeaturePtr,
        compute_value: impl FnOnce(&mut Self) -> EvalResult<'eval>,
    ) -> EvalResult<'eval> {
        if let Some(value) = self.sheet.cached_value(feature) {
            value
        } else {
            let value = compute_value(self);
            self.sheet.cache(feature, value)
        }
    }
}
