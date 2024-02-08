use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VMConditionBranch {
    pub opt_condition_sheet: Option<Vmirs>,
    pub body: Vmirs,
}

pub type VMConditionBranchs = Vec<VMConditionBranch>;
