use crate::*;

#[husky_standard_value::value_conversion]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MnistTask;

impl MnistTask {
    pub fn new() -> Self {
        Self
    }
}
