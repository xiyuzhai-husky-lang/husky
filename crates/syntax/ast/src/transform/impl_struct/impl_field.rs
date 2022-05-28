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
        let mut iter = token_group.iter().enumerate().peekable();
        let liason = match iter.peek().unwrap().1.kind {
            TokenKind::Keyword(Keyword::Liason(liason_keyword)) => {
                iter.next();
                match liason_keyword {
                    LiasonKeyword::Mut => FieldLiason::Mutable,
                }
            }
            _ => FieldLiason::Immutable,
        };
        if token_group.len() <= 2 {
            return err!(
                r#"expect
      <original field>( = <identifier>: <type>)
    | <default field>( = <identifier>[: <type>] ?= <expr>)
    | <derived eager field>( = <identifier> = <expr>)"#,
                token_group.text_range()
            );
        }
        let ident = {
            let (i, token) = iter.next().unwrap();
            p!(i);
            identify_token!(self, token, SemanticTokenKind::Field)
        };
        match iter.next().unwrap().1.kind {
            TokenKind::Special(Special::MaybeEq) => todo!(),
            TokenKind::Special(Special::Colon) => todo!(),
            _ => todo!(),
        }
        todo!()
        //  && token_group[1].kind == TokenKind::Special(Special::Colon) {
        //     let ident = identify_token!(self, token_group[0], SemanticTokenKind::Field);
        //     let ty = atom::parse_route(self, &token_group[2..])?;
        //     Ok(AstKind::FieldDefnHead {
        //         head: FieldDefnHead {
        //             liason,
        //             ident,
        //             ty,
        //             kind: FieldKind::StructOriginal,
        //         },
        //         opt_expr: todo!(),
        //     })
        // }
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
