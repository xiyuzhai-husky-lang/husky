use crate::Runtime;
use husky_comptime::{AskCompileTime, HuskyComptime};

pub trait AskRuntime {
    fn runtime(&self) -> &Runtime;

    fn comptime(&self) -> &HuskyComptime {
        self.runtime().comptime()
    }
}
