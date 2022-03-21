use scope::RangedScope;

use super::*;

impl<'a> TySheetBuilder<'a> {
    pub(super) fn infer_routine(
        &mut self,
        line_group_idx: usize,
        inputs: &[InputPlaceholder],
        output_ty: ScopePtr,
        ast_iter: AstIter,
        arena: &RawExprArena,
    ) {
        self.add_inputs(inputs);
        self.infer_stmts(ast_iter.clone(), output_ty, arena);
    }
}
