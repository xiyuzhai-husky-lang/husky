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
        let paradigm = match token_group[0].kind {
            TokenKind::Keyword(Keyword::Paradigm(paradigm)) => paradigm,
            _ => todo!(),
        };
        expect_at_least!(token_group, token_group.text_range(), 5);
        expect_block_head!(token_group);
        const funcname_idx: usize = 1;
        let head = self.parse_atoms(&token_group[funcname_idx..], |parser| {
            parser.call_defn_head(Some(ParameterLiason::Pure), paradigm)
        })?;
        match self.push_new_symbol(Symbol {
            ident: head.ident.ident,
            kind: SymbolKind::ThisMethod,
        }) {
            Some(old) => match old.kind {
                SymbolKind::ThisMethod => {
                    return err!(
                        format!("a method with the same name already exists"),
                        head.ident.range
                    )
                }
                SymbolKind::ThisField { .. } => {
                    return err!(
                        format!("a field with the same name already exists"),
                        head.ident.range
                    )
                }
                _ => (),
            },
            _ => (),
        };
        enter_block(self);
        self.opt_this_liason
            .set(Some(head.opt_this_liason.unwrap()));
        self.symbols.extend(
            head.parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        self.context.set(AstContext::Stmt(paradigm));
        context_update_result?;
        Ok(AstVariant::CallFormDefnHead(head))
    }
}
