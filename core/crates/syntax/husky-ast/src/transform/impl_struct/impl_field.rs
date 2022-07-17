use word::LiasonKeyword;

use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_eager_field(
        &mut self,
        token_group: &[HuskyToken],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        let mut token_stream: TokenStream = token_group.into();
        let opt_this_ty = self.opt_this_ty();
        let mut parser = AtomParser::new(self, &mut token_stream);
        let field_liason = MemberLiason::from_opt_keyword(try_get!(parser, liason));
        let ident = get!(parser, sema_custom_ident, SemanticTokenKind::Field);
        eat!(parser, ":");
        let opt_field_ty = try_get!(parser, ranged_ty?);
        match self.push_new_symbol(Symbol {
            init_ident: ident,
            kind: SymbolKind::ThisField {
                opt_this_ty,
                opt_field_ty,
                field_liason,
            },
        }) {
            Some(old) => match old.kind {
                SymbolKind::ThisMethod => {
                    return err!(
                        format!("a method with the same name already exists"),
                        ident.range
                    )
                }
                SymbolKind::ThisField { .. } => {
                    return err!(
                        format!("a field with the same name already exists"),
                        ident.range
                    )
                }
                _ => (),
            },
            _ => (),
        };
        let mut parser = AtomParser::new(self, &mut token_stream);
        let ty = if let Some(ty) = opt_field_ty {
            ty
        } else {
            return err!(format!("expect type"), parser.token_stream.next_range());
        };
        let field_kind = if try_eat!(
            parser,
            token_kind,
            HuskyTokenKind::Special(SpecialToken::Assign)
        ) {
            self.update_struct_item_context(StructItemContext::DefaultField, token_group)?;
            enter_block(self);
            self.context.set(AstContext::Stmt {
                paradigm: Paradigm::EagerFunctional,
                return_kind: ReturnKind::Normal,
            });
            self.opt_this_liason.set(Some(ParameterLiason::Pure));
            if token_stream.empty() {
                return err!(
                    format!("expect expr but got nothing"),
                    token_stream.next_range()
                );
            }
            let mut parser = AtomParser::new(self, &mut token_stream);
            let atoms = parser.parse_all()?;
            FieldAstKind::StructDefault {
                default: self.parse_expr_from_atoms(atoms)?,
            }
        } else if try_eat!(
            parser,
            token_kind,
            HuskyTokenKind::Special(SpecialToken::DeriveAssign)
        ) {
            self.update_struct_item_context(StructItemContext::DerivedEagerField, token_group)?;
            enter_block(self);
            self.context.set(AstContext::Stmt {
                paradigm: Paradigm::EagerFunctional,
                return_kind: ReturnKind::Normal,
            });
            self.opt_this_liason.set(Some(ParameterLiason::Pure));
            if token_stream.empty() {
                return err!(
                    format!("expect expr but got nothing"),
                    token_stream.next_range()
                );
            }
            let mut parser = AtomParser::new(self, &mut token_stream);
            let atoms = parser.parse_all()?;
            FieldAstKind::StructDerivedEager {
                derivation: self.parse_expr_from_atoms(atoms)?,
            }
        } else {
            end!(parser);
            self.update_struct_item_context(StructItemContext::OriginalField, token_group)?;
            FieldAstKind::StructOriginal
        };
        Ok(AstVariant::FieldDefnHead {
            liason: field_liason,
            ranged_ident: ident,
            field_ty: ty,
            field_ast_kind: field_kind,
        })
    }

    pub(super) fn parse_struct_derived_lazy_field(
        &mut self,
        token_group: &[HuskyToken],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        let context_update_result =
            self.update_struct_item_context(StructItemContext::DerivedLazyField, token_group);
        enter_block(self);
        let paradigm = match token_group[0].kind {
            HuskyTokenKind::Keyword(Keyword::Paradigm(paradigm)) => paradigm,
            _ => todo!(),
        };
        self.context.set(AstContext::Stmt {
            paradigm,
            return_kind: ReturnKind::LazyField,
        });
        self.opt_this_liason.set(Some(ParameterLiason::EvalRef));
        let ident = identify_token!(self, token_group[1], SemanticTokenKind::Field);
        match token_group[2].kind {
            HuskyTokenKind::Special(SpecialToken::LightArrow) => (),
            _ => todo!(),
        }
        let ty_result = husky_atom::parse_route(self, &token_group[3..]);
        self.symbols.push(Symbol {
            init_ident: ident,
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
            field_ty: ty,
            field_ast_kind: FieldAstKind::StructDerivedLazy { paradigm },
        })
    }
}
