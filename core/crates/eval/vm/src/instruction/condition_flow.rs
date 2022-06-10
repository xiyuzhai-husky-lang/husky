use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VMConditionBranch {
    pub opt_condition_sheet: Option<Arc<InstructionSheet>>,
    pub body: Arc<InstructionSheet>,
}
