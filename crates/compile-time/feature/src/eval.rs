mod id;
mod impl_block;
mod impl_expr;
mod impl_repr;
mod impl_stmt;
mod indicator;
mod sheet;

pub(crate) use id::FeatureEvalId;
pub use indicator::FeatureEvalIndicator;
pub use sheet::*;

use vm::{AnyValueDyn, EvalValue};

use crate::*;
use vm::EvalResult;

pub fn eval_feature_lazy_block<'eval>(
    db: &dyn FeatureQueryGroup,
    block: &FeatureLazyBlock,
    eval_input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &mut EvalSheet<'eval>,
    eval_key: EvalKey<'eval>,
) -> EvalResult<'eval> {
    let mut evaluator = FeatureEvaluator {
        db,
        eval_input,
        sheet,
    };
    evaluator.eval_feature_block(block, eval_key)
}

pub fn eval_feature_stmt<'eval>(
    db: &dyn FeatureQueryGroup,
    stmt: &FeatureStmt,
    eval_input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &mut EvalSheet<'eval>,
) -> EvalResult<'eval> {
    if let Some(value) = sheet.cached_value(EvalKey::Feature(stmt.opt_feature.unwrap())) {
        value
    } else {
        let mut evaluator = FeatureEvaluator {
            db,
            eval_input,
            sheet,
        };
        evaluator.eval_feature_stmt(stmt)
    }
}

pub fn eval_feature_expr<'eval>(
    db: &dyn FeatureQueryGroup,
    expr: &FeatureExpr,
    input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &mut EvalSheet<'eval>,
) -> EvalResult<'eval> {
    if let Some(value) = sheet.cached_value(EvalKey::Feature(expr.feature)) {
        value
    } else {
        let mut evaluator = FeatureEvaluator {
            db,
            eval_input: input,
            sheet,
        };
        evaluator.eval_feature_expr(expr)
    }
}

pub fn eval_feature_repr<'eval>(
    db: &dyn FeatureQueryGroup,
    repr: &FeatureRepr,
    input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &mut EvalSheet<'eval>,
) {
    todo!()
}

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    eval_input: Arc<dyn AnyValueDyn<'eval>>,
    sheet: &'a mut EvalSheet<'eval>,
    db: &'a dyn FeatureQueryGroup,
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    fn cache(
        &mut self,
        eval_key: EvalKey<'eval>,
        compute_value: impl FnOnce(&mut Self) -> EvalResult<'eval>,
    ) -> EvalResult<'eval> {
        if let Some(value) = self.sheet.cached_value(eval_key) {
            value
        } else {
            let value = compute_value(self);
            self.sheet.cache(eval_key, value)
        }
    }
}
