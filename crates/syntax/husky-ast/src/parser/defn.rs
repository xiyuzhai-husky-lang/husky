use husky_entity_path::*;
use husky_entity_taxonomy::{
    AssociatedItemKind, FormKind, ModuleItemConnectionKind, ModuleItemKind, TraitItemKind,
    TypeItemKind, TypeKind,
};
use husky_opn_syntax::{BinaryOpr, Bracket};
use husky_print_utils::p;
use husky_token::{FormKeyword, TokenParseContext, TypeKeyword};
use parsec::{ParseContext, ParseFrom};
use salsa::DebugWithDb;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_defn(&mut self, context: &Context, token_group_idx: TokenGroupIdx) -> Ast {
        self.parse_defn_aux(context, token_group_idx)
            .unwrap_or_else(|error| Ast::Err {
                token_group_idx,
                error,
            })
    }

    fn parse_defn_aux(&mut self, ctx: &Context, token_group_idx: TokenGroupIdx) -> AstResult<Ast> {
        let mut aux_parser = BasicAuxAstParser::new(
            self.db,
            ctx,
            self.module_path,
            self.token_sheet
                .token_group_token_stream(token_group_idx, None),
        );
        let (accessibility, entity_kind, ident_token, is_generic, saved_stream_state) =
            aux_parser.parse_head()?;
        let ident = ident_token.ident();
        let entity_path: Option<husky_entity_path::EntityPath> = match entity_kind {
            EntityKind::Module => {
                Some(ModulePath::new_child(self.db, self.module_path, ident).into())
            }
            EntityKind::ModuleItem {
                module_item_kind,
                connection,
            } => {
                let connection = self.new_connection(ident, connection);
                Some(match module_item_kind {
                    ModuleItemKind::Type(ty_kind) => {
                        TypePath::new(self.db, self.module_path, ident, connection, ty_kind).into()
                    }
                    ModuleItemKind::Form(form_kind) => {
                        FormPath::new(self.db, self.module_path, ident, connection, form_kind)
                            .into()
                    }
                    ModuleItemKind::Trait => {
                        TraitPath::new(self.db, self.module_path, ident, connection).into()
                    }
                })
            }
            EntityKind::AssociatedItem { .. } => None,
            EntityKind::Variant => todo!(),
        };
        let ast_ctx_kind = AstContextKind::inside_defn(entity_kind, entity_path);
        let (body, body_kind) = {
            let body = self.parse_asts(ctx.subcontext(ast_ctx_kind));
            match body.last() {
                Some(_) => (body, DefnBodyKind::Block),
                None => match self
                    .token_groups
                    .peek_token_group_of_exact_indent_with_its_first_token(ctx.indent())
                {
                    Some((_token_group_idx, token_group, first_noncomment_token)) => {
                        match first_noncomment_token {
                            Token::Punctuation(Punctuation::Vertical) => (
                                self.parse_enum_variants(ctx.subcontext(ast_ctx_kind)),
                                DefnBodyKind::EnumVariants,
                            ),
                            _ => (Default::default(), DefnBodyKind::None),
                        }
                    }
                    None => (Default::default(), DefnBodyKind::None),
                },
            }
        };
        Ok(Ast::Defn {
            // order matters!
            accessibility,
            ident_token,
            is_generic,
            token_group_idx,
            body,
            body_kind,
            entity_kind,
            entity_path,
            saved_stream_state,
        })
    }

    #[inline(always)]
    fn new_connection(
        &mut self,
        ident: Ident,
        kind: ModuleItemConnectionKind,
    ) -> ModuleItemConnection {
        match kind {
            ModuleItemConnectionKind::Connected => ModuleItemConnection::Connected,
            ModuleItemConnectionKind::Disconnected => {
                ModuleItemConnection::Disconnected(self.disambiguator_registry.issue_new(ident))
            }
        }
    }

    fn parse_enum_variants(&mut self, context: Context) -> AstIdxRange {
        let mut verticals = vec![];
        while let Some((idx, token_group, first_noncomment_token)) = self
            .token_groups
            .peek_token_group_of_exact_indent_with_its_first_token(context.indent())
        {
            match first_noncomment_token {
                Token::Punctuation(Punctuation::Vertical) => {
                    self.token_groups.next();
                    verticals.push(self.parse_enum_variant(idx, &context))
                }
                _ => break,
            }
        }
        self.alloc_asts(verticals)
    }

    fn parse_enum_variant(&mut self, token_group_idx: TokenGroupIdx, context: &Context) -> Ast {
        let ident = match self.token_sheet[token_group_idx].get(1) {
            Some(token) => match token {
                Token::Keyword(_) => todo!(),
                Token::Ident(_) => todo!(),
                Token::Label(_) => todo!(),
                Token::Punctuation(_) => todo!(),
                Token::WordOpr(_) => todo!(),
                Token::Literal(_) => todo!(),
                Token::Error(_) => todo!(),
            },
            None => todo!(),
        };
        let module_item_path = todo!();
        Ast::ModuleItemVariant {
            token_group_idx,
            module_item_variant_path: VariantPath::new(self.db, module_item_path, ident),
            ident: todo!(),
        }
    }
}

impl<'a> BasicAuxAstParser<'a> {
    fn parse_head(mut self) -> AstResult<(Accessibility, EntityKind, IdentToken, bool, TokenIdx)> {
        let accessibility = self.parse_accessibility()?;
        let kw = self.take_entity_kind_keyword()?;
        let ident: IdentToken = self.parse_expected(OriginalAstError::ExpectIdent)?;
        let is_generic = self.parse_is_generic();
        let entity_kind = match self.ast_context_kind() {
            AstContextKind::InsideTrait { module_item_path } => {
                self.parse_entity_kind_inside_trai(kw)?
            }
            AstContextKind::InsideEnumLikeType { module_item_path } => todo!(),
            AstContextKind::InsideForm => self.parse_form_item_entity_kind(kw)?,
            AstContextKind::InsideTypeImplBlock => {
                self.parse_entity_kind_inside_ty_impl_block(kw)?
            }
            AstContextKind::InsideTypeAsTraitImplBlock => match kw {
                Keyword::Config(_) => todo!(),
                Keyword::Form(kw) => {
                    let Some(trait_item_kind_token) = self.token_stream_mut().peek() else {
                       Err(OriginalAstError::UnexpectedEndAfterFormKeywordInsideTypeAsTraitImplBlock)?
                    };
                    let ty_as_trai_impl_block_item_kind: TraitItemKind = match trait_item_kind_token
                    {
                        Token::Punctuation(special_token) => match special_token {
                            Punctuation::Bra(Bracket::Par) | Punctuation::LaOrLt => match kw {
                                FormKeyword::Def => todo!(),
                                FormKeyword::Func
                                | FormKeyword::Proc
                                | FormKeyword::Fn
                                | FormKeyword::Function => TraitItemKind::Method,
                                FormKeyword::Theorem => todo!(),
                                FormKeyword::Lemma => todo!(),
                                FormKeyword::Proposition => todo!(),
                                FormKeyword::Type => todo!(),
                                FormKeyword::Const => todo!(),
                                FormKeyword::Mm => todo!(),
                            },
                            Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                                todo!()
                                // TraitItemKind::Memo
                            }
                            unexpected_special_token => {
                                return Err(
                                    OriginalAstError::UnexpectedTokenForTypeAsTraitImplItem(
                                        self.token_stream().state(),
                                    )
                                    .into(),
                                )
                            }
                        },
                        ref unexpected_token => {
                            return Err(OriginalAstError::UnexpectedTokenForTypeAsTraitImplItem(
                                self.token_stream().state(),
                            )
                            .into())
                        }
                    };
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TypeAsTraitItem(
                            ty_as_trai_impl_block_item_kind,
                        ),
                    }
                }
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Pattern(_) => todo!(),
                Keyword::Main => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Visual => todo!(),
                Keyword::Impl => todo!(),
                Keyword::Trait => todo!(),
                Keyword::End(_) => todo!(),
                Keyword::Connection(_) => todo!(),
                Keyword::Pronoun(_) => todo!(),
                Keyword::Pub => todo!(),
                Keyword::Static => todo!(),
                Keyword::Async => todo!(),
            },
            AstContextKind::InsideModule => self.parse_module_item_entity_kind(kw)?,
            AstContextKind::InsideMatchStmt => todo!(),
            AstContextKind::InsideNoChild => return Err(OriginalAstError::ExpectNothing.into()),
        };
        Ok((
            accessibility,
            entity_kind,
            ident,
            is_generic,
            self.finish_with_saved_stream_state(),
        ))
    }

    fn parse_module_item_entity_kind(&mut self, kw: Keyword) -> AstResult<EntityKind> {
        Ok(match kw {
            Keyword::Config(_) => todo!(),
            Keyword::Form(kw) => {
                let Some(form_kind_token) = self.token_stream_mut().peek() else {
                    Err(OriginalAstError::UnexpectedEndAfterFormKeywordInsideModule)?
                };
                let form_kind = match *form_kind_token {
                    Token::Punctuation(punctuation) => match punctuation {
                        Punctuation::Bra(Bracket::Par) | Punctuation::LaOrLt => match kw {
                            FormKeyword::Def => todo!(),
                            FormKeyword::Func
                            | FormKeyword::Proc
                            | FormKeyword::Fn
                            | FormKeyword::Function => FormKind::Function,
                            FormKeyword::Theorem => todo!(),
                            FormKeyword::Lemma => todo!(),
                            FormKeyword::Proposition => todo!(),
                            FormKeyword::Type => FormKind::TypeAlias,
                            FormKeyword::Const => FormKind::Value,
                            FormKeyword::Mm => todo!(),
                        },
                        Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                            FormKind::Feature
                        }
                        unexpected_punctuation => Err(
                            OriginalAstError::UnexpectedPunctuationForConnectedModuleItem(
                                self.token_stream().state(),
                                unexpected_punctuation,
                            ),
                        )?,
                    },
                    unexpected_token => {
                        Err(OriginalAstError::UnexpectedTokenForConnectedModuleItem(
                            self.token_stream().state(),
                        ))?
                    }
                };
                EntityKind::ModuleItem {
                    module_item_kind: ModuleItemKind::Form(form_kind).into(),
                    connection: ModuleItemConnectionKind::Connected,
                }
            }
            Keyword::Type(kw) => {
                let type_kind = match kw {
                    TypeKeyword::Extern => TypeKind::Extern,
                    TypeKeyword::Struct => TypeKind::Struct,
                    TypeKeyword::Enum => TypeKind::Enum,
                    TypeKeyword::Record => TypeKind::Record,
                    TypeKeyword::Structure => TypeKind::Structure,
                    TypeKeyword::Inductive => TypeKind::Inductive,
                };
                EntityKind::ModuleItem {
                    module_item_kind: ModuleItemKind::Type(type_kind).into(),
                    connection: ModuleItemConnectionKind::Connected,
                }
            }
            Keyword::Stmt(_) => todo!(),
            Keyword::Pattern(_) => todo!(),
            Keyword::Main => todo!(),
            Keyword::Use => todo!(),
            Keyword::Mod => EntityKind::Module,
            Keyword::Visual => todo!(),
            Keyword::Impl => todo!(),
            Keyword::Trait => EntityKind::ModuleItem {
                module_item_kind: ModuleItemKind::Trait.into(),
                connection: ModuleItemConnectionKind::Connected,
            },
            Keyword::End(_) => todo!(),
            Keyword::Connection(_) => todo!(),
            Keyword::Pronoun(_) => todo!(),
            Keyword::Pub => todo!(),
            Keyword::Static => todo!(),
            Keyword::Async => todo!(),
        })
    }

    fn parse_entity_kind_inside_ty_impl_block(&mut self, kw: Keyword) -> AstResult<EntityKind> {
        Ok(match kw {
            Keyword::Config(_) => todo!(),
            Keyword::Form(kw) => {
                let Some(type_item_kind_token) = self.token_stream().peek() else {
                    Err(OriginalAstError::UnexpectedEndAfterFormKeywordInsideTypeImplBlock)?
                };
                let type_item_kind: TypeItemKind = match *type_item_kind_token {
                    Token::Punctuation(punctuation) => match punctuation {
                        Punctuation::Bra(Bracket::Par) | Punctuation::LaOrLt => match kw {
                            FormKeyword::Def => todo!(),
                            FormKeyword::Func
                            | FormKeyword::Proc
                            | FormKeyword::Fn
                            | FormKeyword::Function => TypeItemKind::Method,
                            FormKeyword::Theorem => todo!(),
                            FormKeyword::Lemma => todo!(),
                            FormKeyword::Proposition => todo!(),
                            FormKeyword::Type => todo!(),
                            FormKeyword::Const => todo!(),
                            FormKeyword::Mm => todo!(),
                        },
                        Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                            TypeItemKind::Memo
                        }
                        unexpected_punctuation => {
                            return Err(OriginalAstError::UnexpectedPunctuationForTypeImplItem(
                                self.token_stream().state(),
                                unexpected_punctuation,
                            ))?
                        }
                    },
                    ref unexpected_token => Err(OriginalAstError::UnexpectedTokenForTypeImplItem(
                        self.token_stream().state(),
                    ))?,
                };
                EntityKind::AssociatedItem {
                    associated_item_kind: AssociatedItemKind::TypeItem(type_item_kind),
                }
            }
            Keyword::Type(_) => Err(OriginalAstError::UnexpectedTypeDefnInsideTypeImplBlock)?,
            Keyword::Stmt(_) => todo!(),
            Keyword::Pattern(_) => todo!(),
            Keyword::Main => todo!(),
            Keyword::Use => todo!(),
            Keyword::Mod => todo!(),
            Keyword::Visual => todo!(),
            Keyword::Impl => todo!(),
            Keyword::Trait => todo!(),
            Keyword::End(_) => todo!(),
            Keyword::Connection(_) => todo!(),
            Keyword::Pronoun(_) => todo!(),
            Keyword::Pub => todo!(),
            Keyword::Static => todo!(),
            Keyword::Async => todo!(),
        })
    }

    fn parse_entity_kind_inside_trai(&mut self, kw: Keyword) -> AstResult<EntityKind> {
        Ok(match kw {
            Keyword::Config(_) => todo!(),
            Keyword::Form(kw) => {
                let Some(trai_item_kind_token) = self.token_stream().peek() else {
                    return  Err(OriginalAstError::UnexpectedEndAfterFormKeywordInsideTrait.into());
                };
                let trai_item_kind: TraitItemKind = match trai_item_kind_token {
                    Token::Punctuation(punctuation) => match punctuation {
                        Punctuation::Bra(Bracket::Par) | Punctuation::LaOrLt => match kw {
                            FormKeyword::Def => todo!(),
                            FormKeyword::Func
                            | FormKeyword::Proc
                            | FormKeyword::Fn
                            | FormKeyword::Function => TraitItemKind::Method,
                            FormKeyword::Theorem => todo!(),
                            FormKeyword::Lemma => todo!(),
                            FormKeyword::Proposition => todo!(),
                            FormKeyword::Type => todo!(),
                            FormKeyword::Const => todo!(),
                            FormKeyword::Mm => todo!(),
                        },
                        Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                            todo!()
                        }
                        unexpected_punctuation => {
                            Err(OriginalAstError::UnexpectedPunctuationForTraitItem(
                                self.token_stream().state(),
                                *unexpected_punctuation,
                            ))?
                        }
                    },
                    unexpected_token => Err(OriginalAstError::UnexpectedTokenForTraitItem(
                        self.token_stream().state(),
                    ))?,
                };
                EntityKind::AssociatedItem {
                    associated_item_kind: AssociatedItemKind::TraitItem(trai_item_kind),
                }
            }
            Keyword::Type(kw) => {
                todo!()
            }
            Keyword::Stmt(_) | Keyword::Pattern(_) => {
                Err(OriginalAstError::UnexpectedStmtInsideTrait)?
            }
            Keyword::Main => Err(OriginalAstError::UnexpectedMainInsideTrait)?,
            Keyword::Use => Err(OriginalAstError::UnexpectedUseInsideTrait)?,
            Keyword::Mod => Err(OriginalAstError::UnexpectedModInsideTrait)?,
            Keyword::Visual => Err(OriginalAstError::UnexpectedVisualInsideTrait)?,
            Keyword::Impl => Err(OriginalAstError::UnexpectedImplInsideTrait)?,
            Keyword::Trait => Err(OriginalAstError::UnexpectedTraitInsideTrait)?,
            Keyword::End(_) => todo!(),
            Keyword::Connection(_) => todo!(),
            Keyword::Pronoun(_) => todo!(),
            Keyword::Pub => todo!(),
            Keyword::Static => todo!(),
            Keyword::Async => todo!(),
        })
    }

    fn parse_form_item_entity_kind(&mut self, kw: Keyword) -> AstResult<EntityKind> {
        Ok(match kw {
            Keyword::Config(_) => todo!(),
            Keyword::Form(kw) => {
                let Some(form_kind_token) = self.token_stream_mut().peek() else {
                    Err(OriginalAstError::UnexpectedEndAfterFormKeywordInsideModule)?
                };
                let form_kind = match form_kind_token {
                    Token::Punctuation(punctuation) => match punctuation {
                        Punctuation::Bra(Bracket::Par) | Punctuation::LaOrLt => match kw {
                            FormKeyword::Def => todo!(),
                            FormKeyword::Func
                            | FormKeyword::Proc
                            | FormKeyword::Fn
                            | FormKeyword::Function => FormKind::Function,
                            FormKeyword::Theorem => todo!(),
                            FormKeyword::Lemma => todo!(),
                            FormKeyword::Proposition => todo!(),
                            FormKeyword::Type => todo!(),
                            FormKeyword::Const => todo!(),
                            FormKeyword::Mm => todo!(),
                        },
                        Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                            FormKind::Feature
                        }
                        unexpected_punctuation => Err(
                            OriginalAstError::UnexpectedPunctuationForDisconnectedModuleItem(
                                self.token_stream().state(),
                                *unexpected_punctuation,
                            ),
                        )?,
                    },
                    ref unexpected_token => {
                        Err(OriginalAstError::UnexpectedTokenForDisconnectedModuleItem(
                            self.token_stream().state(),
                        ))?
                    }
                };
                EntityKind::ModuleItem {
                    module_item_kind: ModuleItemKind::Form(form_kind).into(),
                    connection: ModuleItemConnectionKind::Disconnected,
                }
            }
            Keyword::Type(kw) => {
                let type_kind = match kw {
                    TypeKeyword::Extern => TypeKind::Extern,
                    TypeKeyword::Struct => TypeKind::Struct,
                    TypeKeyword::Enum => TypeKind::Enum,
                    TypeKeyword::Record => TypeKind::Record,
                    TypeKeyword::Structure => TypeKind::Structure,
                    TypeKeyword::Inductive => TypeKind::Inductive,
                };
                EntityKind::ModuleItem {
                    module_item_kind: ModuleItemKind::Type(type_kind).into(),
                    connection: ModuleItemConnectionKind::Disconnected,
                }
            }
            Keyword::Stmt(_) => todo!(),
            Keyword::Pattern(_) => todo!(),
            Keyword::Main => todo!(),
            Keyword::Use => todo!(),
            Keyword::Mod => Err(OriginalAstError::UnexpectedModInsideForm)?,
            Keyword::Visual => Err(OriginalAstError::UnexpectedVisualInsideForm)?,
            Keyword::Impl => Err(OriginalAstError::UnexpectedImplInsideForm)?,
            Keyword::Trait => Err(OriginalAstError::UnexpectedTraitInsideForm)?,
            Keyword::End(_) => todo!(),
            Keyword::Connection(_) => todo!(),
            Keyword::Pronoun(_) => todo!(),
            Keyword::Pub => todo!(),
            Keyword::Static => todo!(),
            Keyword::Async => todo!(),
        })
    }
}
