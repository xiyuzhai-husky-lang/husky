use crate::instruction::{Instruction, VMStackIdx};
use husky_coword::Ident;
use husky_task_interface::IsLinkageImpl;

pub(crate) struct InstructionBlockBuilder<'a, LinkageImpl: IsLinkageImpl> {
    db: &'a ::salsa::Db,
    instructions: Vec<Instruction<LinkageImpl>>,
    variables: Vec<Ident>,
}

impl<'a, LinkageImpl: IsLinkageImpl> InstructionBlockBuilder<'a, LinkageImpl> {
    pub(super) fn push_instruction(&mut self, instr: Instruction<LinkageImpl>) {
        self.instructions.push(instr);
    }

    pub(super) fn def_variable(&mut self, varname: Ident) {
        self.variables.push(varname);
    }

    pub(super) fn varidx(&self, varname: Ident) -> VMStackIdx {
        // self.variables.stack_idx(varname)
        todo!()
    }
}
