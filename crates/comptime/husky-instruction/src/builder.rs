mod expr;

use crate::instruction::{InstructionArena, InstructionData, InstructionIdxRange, VMStackIdx};
use husky_coword::Ident;
use husky_hir_eager_expr::HirEagerExprArena;

pub(crate) struct InstructionBlockBuilder<'db, 'arena> {
    db: &'db ::salsa::Db,
    expr_arena: &'db HirEagerExprArena,
    instruction_arena: &'arena mut InstructionArena,
    buffer: Vec<InstructionData>,
    variables: Vec<Ident>,
}

impl<'db, 'arena> InstructionBlockBuilder<'db, 'arena> {
    pub(super) fn push_instruction(&mut self, instruction: InstructionData) {
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
        self.instruction_arena.alloc_batch(self.buffer)
    }
}
