use crate::*;

use vm::{InitKind, Instruction, InstructionSource};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn push_instruction(&mut self, instr: Instruction) {
        self.sheet.instructions.push(instr);
    }

    pub(super) fn def_variable(&mut self, varname: CustomIdentifier) {
        self.sheet.variable_stack.push(varname);
    }

    pub(super) fn def_frame_variable(&mut self, frame_varname: CustomIdentifier) {
        self.sheet.variable_stack.push(frame_varname);
    }
}
