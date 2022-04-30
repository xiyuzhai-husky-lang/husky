use crate::*;

use vm::{EagerContract, InitKind, Instruction, InstructionKind, InstructionSource, StackIdx};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn push_instruction(&mut self, instr: Instruction) {
        self.sheet.instructions.push(instr);
    }

    pub(super) fn def_variable(
        &mut self,
        varname: CustomIdentifier,
        init_kind: InitKind,
        src: Arc<dyn InstructionSource>,
    ) {
        self.push_instruction(Instruction::new(InstructionKind::Init(init_kind), src));
        self.sheet.variable_stack.push(varname);
    }

    pub(super) fn def_frame_variable(&mut self, frame_varname: CustomIdentifier) {
        self.sheet.variable_stack.push(frame_varname);
    }
}
