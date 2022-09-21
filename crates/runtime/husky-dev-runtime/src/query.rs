use crate::HuskyDevRuntime;
use husky_comptime::{AskCompileTime, HuskyComptime};

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyDevRuntime;

    fn comptime(&self) -> &HuskyComptime {
        self.runtime().comptime()
    }
}
