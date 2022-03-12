use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InstructionSheet {
    pub instructions: Vec<Instruction>,
    pub variable_stack: VariableStack,
}

impl InstructionSheet {
    pub fn init_subsheet(&self) -> Self {
        Self {
            instructions: vec![],
            variable_stack: self.variable_stack.clone(),
        }
    }
}
