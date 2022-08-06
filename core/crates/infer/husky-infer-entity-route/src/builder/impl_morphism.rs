use super::*;

impl<'a> EntityRouteSheetBuilder<'a> {
    pub(super) fn infer_Model(
        &mut self,
        inputs: &[InputParameter],
        opt_return_ty: Option<EntityRoutePtr>,
        ast_iter: AstIter,
        arena: &RawExprArena,
    ) {
        self.add_inputs(inputs);
        self.infer_stmts(ast_iter.clone(), opt_return_ty);
    }
}
