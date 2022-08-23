use crate::HuskyRuntime;
use husky_comptime::{AskCompileTime, HuskyComptime};
use husky_feature_eval::EvalFeature;

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyRuntime;

    fn comptime(&self) -> &HuskyComptime {
        self.runtime().comptime()
    }
}
