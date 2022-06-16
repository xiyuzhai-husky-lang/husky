mod impl_block;
mod impl_expr;
mod impl_repr;
mod impl_stmt;
mod indicator;
mod sheet;

pub use indicator::FeatureEvalIndicator;
pub use sheet::*;

use crate::*;
use feature_gen::FeatureEvalId;
use vm::EvalResult;
use vm::{AnyValueDyn, EvalValue};

pub fn eval_feature_lazy_block<'eval>(
    db: &dyn FeatureEvalQueryGroup,
    block: &FeatureLazyBlock,
    eval_input: EvalValue<'eval>,
    sheet: &EvalSheet<'eval>,
    verbose: bool,
) -> EvalResult<'eval> {
    let mut evaluator = FeatureEvaluator {
        db,
        eval_input,
        sheet,
        verbose,
    };
    evaluator.eval_feature_lazy_block(block)
}

pub fn eval_feature_stmt<'eval>(
    db: &dyn FeatureEvalQueryGroup,
    stmt: &FeatureStmt,
    eval_input: EvalValue<'eval>,
    sheet: &EvalSheet<'eval>,
    verbose: bool,
) -> EvalResult<'eval> {
    if let Some(value) = sheet.cached_value(EvalKey::Feature(stmt.opt_feature.unwrap())) {
        value
    } else {
        let mut evaluator = FeatureEvaluator {
            db,
            eval_input,
            sheet,
            verbose,
        };
        evaluator.eval_feature_stmt(stmt)
    }
}

pub fn eval_feature_expr<'eval>(
    db: &dyn FeatureEvalQueryGroup,
    expr: &FeatureExpr,
    eval_input: EvalValue<'eval>,
    sheet: &EvalSheet<'eval>,
    verbose: bool,
) -> EvalResult<'eval> {
    if let Some(value) = sheet.cached_value(EvalKey::Feature(expr.feature)) {
        value
    } else {
        let mut evaluator = FeatureEvaluator {
            db,
            eval_input,
            sheet,
            verbose,
        };
        evaluator.eval_feature_expr(expr)
    }
}

pub fn eval_feature_repr<'eval>(
    db: &dyn FeatureEvalQueryGroup,
    repr: &FeatureRepr,
    eval_input: EvalValue<'eval>,
    sheet: &EvalSheet<'eval>,
    verbose: bool,
) -> EvalResult<'eval> {
    let mut evaluator = FeatureEvaluator {
        db,
        eval_input,
        sheet,
        verbose,
    };
    evaluator.eval_feature_repr(repr)
}

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    eval_input: EvalValue<'eval>,
    sheet: &'a EvalSheet<'eval>,
    db: &'a dyn FeatureEvalQueryGroup,
    verbose: bool,
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
