use vm::Instruction;

use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InstructionSheet {
    instructions: Vec<Instruction>,
    correspondence: (),
}

impl InstructionSheet {
    pub fn push_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}
