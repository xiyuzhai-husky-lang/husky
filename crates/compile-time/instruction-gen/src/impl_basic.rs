use crate::*;

use vm::{InitKind, Instruction, InstructionSource, StackIdx};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn push_instruction(&mut self, instr: Instruction) {
        self.sheet.instructions.push(instr);
    }

    pub(super) fn def_variable(&mut self, varname: CustomIdentifier) {
        self.sheet.variable_stack.push(varname);
    }

    pub(super) fn varidx(&self, varname: CustomIdentifier) -> StackIdx {
        self.sheet.variable_stack.stack_idx(varname)
    }
}
