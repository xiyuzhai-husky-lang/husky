use crate::*;

use vm::{InitKind, Instruction, InstructionSource, StackIdx};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn push_instruction(&mut self, instr: Instruction) {
        self.sheet.instructions.push(instr);
    }

    pub(super) fn def_variable(&mut self, varname: CustomIdentifier) {
        self.sheet.variable_stack.push(Some(varname));
    }

    pub(super) fn varidx(&self, varname: CustomIdentifier) -> StackIdx {
        self.sheet.variable_stack.stack_idx(varname)
    }

    pub(super) fn def_for_frame_variable(&mut self, frame_varname: CustomIdentifier) {
        self.sheet.variable_stack.push(None); // initial boundary
        self.sheet.variable_stack.push(None); // final boundary
        self.sheet.variable_stack.push(Some(frame_varname));
    }

    pub(super) fn def_forext_frame(&mut self) {
        self.sheet.variable_stack.push(None); // boundary
    }
}
