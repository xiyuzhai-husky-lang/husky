use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_associated_routine(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        let context_update_result = self.update_struct_item_context(
            struct_item_context,
            StructItemContext::AssociatedCall,
            token_group,
        )?;
        enter_block(self);
        let routine_keyword = match token_group[1].kind {
            TokenKind::Keyword(Keyword::Paradigm(routine_keyword)) => routine_keyword,
            _ => todo!(),
        };
        self.context.set(AstContext::Stmt(routine_keyword));
        expect_at_least!(token_group, token_group.text_range(), 5);
        expect_block_head!(token_group);
        const funcname_idx: usize = 2;
        let head = self.parse_atoms(&token_group[funcname_idx..], |parser| {
            parser.routine_defn_head(routine_keyword)
        })?;
        self.opt_this_liason.set(None);
        self.opt_this_ty.set(None);
        self.symbols.extend(
            head.parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        Ok(AstVariant::CallFormDefnHead(head))
    }
}
