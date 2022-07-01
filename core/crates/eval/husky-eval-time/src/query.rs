use crate::HuskyEvalTime;
use husky_compile_time::{AskCompileTime, HuskyCompileTime};
use husky_feature_eval::EvalFeature;

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyEvalTime;

    fn compile_time(&self) -> &HuskyCompileTime {
        self.runtime().compile_time()
    }
}
