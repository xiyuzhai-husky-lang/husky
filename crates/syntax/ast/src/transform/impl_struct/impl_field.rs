use word::LiasonKeyword;

use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_eager_field(
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
        let mut token_stream: TokenStream = token_group.into();
        let mut parser = AtomParser::new(self, &mut token_stream);
        let liason = FieldLiason::from_opt_keyword(try_get!(parser, liason));
        let ident = get!(parser, sema_custom_ident, SemanticTokenKind::Field);
        eat!(parser, ":");
        let ty = get!(parser, ranged_ty?);
        let opt_expr = if try_eat!(
            parser,
            token_kind,
            TokenKind::Special(Special::DeriveAssign)
        ) {
            todo!()
        } else if try_eat!(parser, token_kind, TokenKind::Special(Special::Assign)) {
            todo!()
        } else {
            end!(parser);
            None
        };
        Ok(AstKind::FieldDefnHead {
            head: FieldDefnHead {
                liason,
                ident,
                ty,
                kind: FieldKind::StructOriginal,
            },
            opt_expr,
        })
    }

    fn parse_struct_original_field(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
    ) -> AstResult<AstKind> {
        self.update_struct_item_context(
            struct_item_context,
            StructItemContext::OriginalField,
            token_group,
        )?;
        todo!()
    }

    fn parse_struct_default_field(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
    ) -> AstResult<AstKind> {
        self.update_struct_item_context(
            struct_item_context,
            StructItemContext::DefaultField,
            token_group,
        )?;
        todo!()
    }

    fn parse_struct_derived_eager_field(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
    ) -> AstResult<AstKind> {
        self.update_struct_item_context(
            struct_item_context,
            StructItemContext::DerivedEagerField,
            token_group,
        )?;
        todo!()
    }

    pub(super) fn parse_struct_derived_lazy_field(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        self.update_struct_item_context(
            struct_item_context,
            StructItemContext::DerivedLazyField,
            token_group,
        )?;
        let paradigm = match token_group[0].kind {
            TokenKind::Keyword(Keyword::Paradigm(paradigm)) => paradigm,
            _ => todo!(),
        };
        let ident = identify_token!(self, token_group[1], SemanticTokenKind::Field);
        match token_group[2].kind {
            TokenKind::Special(Special::LightArrow) => (),
            _ => todo!(),
        }
        let ty = atom::parse_route(self, &token_group[3..])?;
        Ok(AstKind::FieldDefnHead {
            head: FieldDefnHead {
                liason: FieldLiason::Derived,
                ident,
                ty,
                kind: FieldKind::StructDerivedLazy { paradigm },
            },
            opt_expr: None,
        })
    }
}
