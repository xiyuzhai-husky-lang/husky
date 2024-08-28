use crate::*;
use ml_task::IsMlTask;

#[husky_standard_value::value_conversion]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MnistTask;

impl MnistTask {
    pub fn new() -> Self {
        Self
    }
}

impl IsMlTask<__VarId> for MnistTask {
    type Input = Leash<BinaryImage28>;

    type INPUT = INPUT;
}
