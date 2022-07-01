use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_method(
        &mut self,
        token_group: &[HuskyToken],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        // even if context update is not successful, must set context correctly for children
        let context_update_result =
            self.update_struct_item_context(StructItemContext::Method, token_group);
        let ast_kind =
            self.call_defn_head(token_group, Some(ParameterLiason::Pure), enter_block)?;
        context_update_result?;
        Ok(ast_kind)
    }
}
