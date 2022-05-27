use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_field(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
    ) -> AstResult<AstKind> {
        msg_once!("derived field");
        self.update_struct_item_context(
            struct_item_context,
            StructItemContext::OriginalField,
            token_group,
        )?;
        if token_group.len() > 2 && token_group[1].kind == TokenKind::Special(Special::Colon) {
            let ident = identify_token!(self, token_group[0], SemanticTokenKind::Field);
            let ty = atom::parse_route(&self.symbol_context(), &token_group[2..])?;
            Ok(AstKind::FieldDefnHead(FieldDefnHead {
                ident,
                contract: FieldLiason::Own,
                ty,
                kind: FieldKind::StructOriginal,
            }))
        } else {
            err!("expect <identifier>: <type>", token_group.text_range())
        }
    }
}
