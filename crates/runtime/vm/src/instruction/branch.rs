use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VMBranch {
    pub opt_condition_sheet: Option<Arc<InstructionSheet>>,
    pub body: Arc<InstructionSheet>,
}
