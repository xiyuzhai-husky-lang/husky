use crate::*;

use vm::{Contract, InitKind, Instruction, InstructionKind, InstructionSource, StackIdx};

impl InstructionSheetBuilder {
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
}
