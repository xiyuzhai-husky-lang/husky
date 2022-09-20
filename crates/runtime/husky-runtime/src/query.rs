use crate::Runtime;
use husky_comptime::{AskCompileTime, Comptime};

pub trait AskRuntime {
    fn runtime(&self) -> &Runtime;

    fn comptime(&self) -> &Comptime {
        self.runtime().comptime()
    }
}
