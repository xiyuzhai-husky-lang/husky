use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_associated_routine(
        &mut self,
        token_group: &[HuskyToken],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        let context_update_result =
            self.update_struct_item_context(StructItemContext::AssociatedCall, token_group)?;
        self.call_defn_head(&token_group[1..], None, enter_block)
    }
}
