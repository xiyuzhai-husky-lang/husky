use crate::HuskyRuntime;
use compile_time_db::{AskCompileTime, HuskyCompileTime};
use eval_feature::EvalFeature;

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyRuntime;

    fn compile_time(&self) -> &HuskyCompileTime {
        self.runtime().compile_time()
    }
}
