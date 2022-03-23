use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionSheet {
    pub instructions: Vec<Instruction>,
    pub variable_stack: VariableStack,
}

impl InstructionSheet {
    pub fn new(inputs: Vec<CustomIdentifier>, has_this: bool) -> Self {
        Self {
            instructions: vec![],
            variable_stack: VariableStack::new(inputs, has_this),
        }
    }

    pub fn init_subsheet(&self) -> Self {
        Self {
            instructions: vec![],
            variable_stack: self.variable_stack.clone(),
        }
    }
}
