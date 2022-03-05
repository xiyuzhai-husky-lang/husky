use vm::{InitKind, Instruction, InstructionKind};

use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InstructionSheet {
    instructions: Vec<Instruction>,
    variables: Vec<Variable>,
    correspondence: (),
}

impl InstructionSheet {
    pub fn push_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn def_variable(&mut self, varname: CustomIdentifier, init_kind: InitKind) {
        self.push_instruction(Instruction {
            kind: InstructionKind::Init(init_kind),
        })
    }
}
