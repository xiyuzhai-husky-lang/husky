use crate::vmir::{VMStackIdx, VmirArena, VmirData, VmirIdxRange};
use husky_coword::Ident;
use husky_hir_eager_expr::HirEagerExprArena;

pub(crate) struct VmirBlockBuilder<'db, 'arena> {
    db: &'db ::salsa::Db,
    expr_arena: &'db HirEagerExprArena,
    instruction_arena: &'arena mut VmirArena,
    buffer: Vec<VmirData>,
    variables: Vec<Ident>,
}

impl<'db, 'arena> VmirBlockBuilder<'db, 'arena> {
    pub(super) fn push_instruction(&mut self, instruction: VmirData) {
        self.buffer.push(instruction)
    }

    pub(super) fn def_variable(&mut self, varname: Ident) {
        self.variables.push(varname);
    }

    pub(super) fn varidx(&self, varname: Ident) -> VMStackIdx {
        // self.variables.stack_idx(varname)
        todo!()
    }

    pub(crate) fn finish(self) -> VmirIdxRange {
        self.instruction_arena.alloc_batch(self.buffer)
    }
}
