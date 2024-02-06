use crate::instruction::{Instruction, InstructionArena, InstructionIdxRange, VMStackIdx};
use husky_coword::Ident;

pub(crate) struct InstructionBlockBuilder<'a, 'b> {
    db: &'a ::salsa::Db,
    arena: &'b mut InstructionArena,
    buffer: Vec<Instruction>,
    variables: Vec<Ident>,
}

impl<'a, 'b> InstructionBlockBuilder<'a, 'b> {
    pub(super) fn push_instruction(&mut self, instruction: Instruction) {
        self.buffer.push(instruction)
    }

    pub(super) fn def_variable(&mut self, varname: Ident) {
        self.variables.push(varname);
    }

    pub(super) fn varidx(&self, varname: Ident) -> VMStackIdx {
        // self.variables.stack_idx(varname)
        todo!()
    }

    pub(crate) fn finish(self) -> InstructionIdxRange {
        self.arena.alloc_batch(self.buffer)
    }
}
