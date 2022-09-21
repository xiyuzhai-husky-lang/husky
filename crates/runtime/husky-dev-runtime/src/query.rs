use crate::HuskyDevRuntime;
use husky_comptime::HuskyComptime;

pub trait AskRuntime {
    fn runtime(&self) -> &HuskyDevRuntime;
}
