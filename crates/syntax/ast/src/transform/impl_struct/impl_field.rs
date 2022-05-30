use word::LiasonKeyword;

use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_eager_field(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
    ) -> AstResult<AstVariant> {
        let mut token_stream: TokenStream = token_group.into();
        let mut parser = AtomParser::new(self, &mut token_stream);
        let field_liason = MemberLiason::from_opt_keyword(try_get!(parser, liason));
        let ident = get!(parser, sema_custom_ident, SemanticTokenKind::Field);
        eat!(parser, ":");
        let opt_field_ty = try_get!(parser, ranged_ty?);
        parser.push_symbol(|atom_context| Symbol {
            ident: ident.ident,
            kind: SymbolKind::ThisField {
                opt_this_ty: atom_context.opt_this_ty(),
                opt_field_ty,
                field_liason,
            },
        });
        let ty = if let Some(ty) = opt_field_ty {
            ty
        } else {
            return err!(format!("expect type"), parser.token_stream.next_range());
        };
        let field_kind = if try_eat!(parser, token_kind, TokenKind::Special(Special::Assign)) {
            let atoms = parser.parse_all()?;
            self.update_struct_item_context(
                struct_item_context,
                StructItemContext::DefaultField,
                token_group,
            )?;
            FieldKind::StructDefault {
                default: self.parse_expr_from_atoms(atoms)?,
            }
        } else if try_eat!(
            parser,
            token_kind,
            TokenKind::Special(Special::DeriveAssign)
        ) {
            let atoms = parser.parse_all()?;
            self.update_struct_item_context(
                struct_item_context,
                StructItemContext::DerivedEagerField,
                token_group,
            )?;
            FieldKind::StructDerivedEager {
                derivation: self.parse_expr_from_atoms(atoms)?,
            }
        } else {
            end!(parser);
            self.update_struct_item_context(
                struct_item_context,
                StructItemContext::OriginalField,
                token_group,
            )?;
            FieldKind::StructOriginal
        };
        Ok(AstVariant::FieldDefnHead {
            liason: field_liason,
            ranged_ident: ident,
            ty,
            field_kind,
        })
    }

    // fn parse_struct_derived_eager_field(
    //     &mut self,
    //     token_group: &[Token],
    //     struct_item_context: StructItemContext,
    //     paradigm: Paradigm,
    // ) -> AstResult<AstKind> {
    //     let context_update_result = self.update_struct_item_context(
    //         struct_item_context,
    //         StructItemContext::DerivedEagerField,
    //         token_group,
    //     );
    //     self.context.set(AstContext::Stmt(paradigm));
    //     context_update_result?;
    //     todo!()
    // }

    pub(super) fn parse_struct_derived_lazy_field(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        let context_update_result = self.update_struct_item_context(
            struct_item_context,
            StructItemContext::DerivedLazyField,
            token_group,
        );
        enter_block(self);
        let paradigm = match token_group[0].kind {
            TokenKind::Keyword(Keyword::Paradigm(paradigm)) => paradigm,
            _ => todo!(),
        };
        self.context.set(AstContext::Stmt(paradigm));
        self.opt_this_liason.set(Some(InputLiason::GlobalRef));
        let ident = identify_token!(self, token_group[1], SemanticTokenKind::Field);
        match token_group[2].kind {
            TokenKind::Special(Special::LightArrow) => (),
            _ => todo!(),
        }
        let ty_result = atom::parse_route(self, &token_group[3..]);
        self.symbols.push(Symbol {
            ident: ident.ident,
            kind: SymbolKind::ThisField {
                opt_this_ty: self.opt_this_ty(),
                opt_field_ty: ty_result.clone().ok(),
                field_liason: MemberLiason::Derived,
            },
        });
        let ty = ty_result?;
        context_update_result?;
        Ok(AstVariant::FieldDefnHead {
            liason: MemberLiason::Derived,
            ranged_ident: ident,
            ty,
            field_kind: FieldKind::StructDerivedLazy { paradigm },
        })
    }
}
