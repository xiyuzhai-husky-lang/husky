use crate::HuskyDevRuntime;
use husky_comptime::{AskCompileTime, Comptime};

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyDevRuntime;

    fn comptime(&self) -> &Comptime {
        self.runtime().comptime()
    }
}
