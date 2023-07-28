use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VMConditionBranch {
    pub opt_condition_sheet: Option<Instructions>,
    pub body: Instructions,
}

pub type VMConditionBranchs = Vec<VMConditionBranch>;
