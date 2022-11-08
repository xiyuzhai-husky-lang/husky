use crate::DevRuntime;
use husky_comptime::HuskyComptime;

pub trait AskRuntime {
    fn runtime(&self) -> &DevRuntime;
}
