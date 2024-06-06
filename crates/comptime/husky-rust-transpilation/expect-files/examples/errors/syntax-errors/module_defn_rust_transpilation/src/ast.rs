use crate::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct A {
}

impl A {
    pub fn __constructor() -> Self {
        Self{
        }
    }
}

#[rustfmt::skip]
impl crate::ast::A {
}