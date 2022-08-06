use super::*;

impl<'a> EntityRouteSheetBuilder<'a> {
    pub(super) fn infer_function(
        &mut self,
        inputs: &[Parameter],
        opt_output_ty: Option<EntityRoutePtr>,
        ast_iter: AstIter,
    ) {
        self.add_inputs(inputs);
        self.infer_stmts(ast_iter.clone(), opt_output_ty)
    }
}
