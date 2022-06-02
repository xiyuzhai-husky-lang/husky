use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_method(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        // even if context update is not successful, must set context correctly for children
        let context_update_result = self.update_struct_item_context(
            struct_item_context,
            StructItemContext::Method,
            token_group,
        );
        enter_block(self);
        let routine_keyword = match token_group[0].kind {
            TokenKind::Keyword(Keyword::Paradigm(routine_keyword)) => routine_keyword,
            _ => todo!(),
        };
        self.context.set(AstContext::Stmt(routine_keyword));
        context_update_result?;
        expect_at_least!(token_group, token_group.text_range(), 5);
        expect_block_head!(token_group);
        const funcname_idx: usize = 1;
        let head = self.parse_atoms(&token_group[funcname_idx..], |parser| {
            parser.method_defn_head(ParameterLiason::Pure, Paradigm::EagerFunctional)
        })?;
        self.opt_this_liason
            .set(Some(head.opt_this_contract.unwrap()));
        self.symbols.extend(
            head.parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        Ok(AstVariant::CallFormDefnHead(head))
    }
}
