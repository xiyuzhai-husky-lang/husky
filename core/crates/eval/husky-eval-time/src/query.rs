use crate::HuskyEvalTime;
use eval_feature::EvalFeature;
use husky_compile_time::{AskCompileTime, HuskyCompileTime};

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyEvalTime;

    fn compile_time(&self) -> &HuskyCompileTime {
        self.runtime().compile_time()
    }
}
