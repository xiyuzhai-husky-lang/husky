use crate::HuskyRuntime;
use husky_comptime::{AskCompileTime, HuskyComptime};

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyRuntime;

    fn comptime(&self) -> &HuskyComptime {
        self.runtime().comptime()
    }
}
