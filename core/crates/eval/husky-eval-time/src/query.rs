use crate::HuskyRuntime;
use husky_compile_time::{AskCompileTime, HuskyComptime};
use husky_feature_eval::EvalFeature;

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyRuntime;

    fn compile_time(&self) -> &HuskyComptime {
        self.runtime().compile_time()
    }
}
