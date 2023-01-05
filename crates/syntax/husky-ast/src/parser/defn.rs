use husky_entity_path::*;
use husky_entity_taxonomy::{
    AssociatedItemKind, FormKind, ModuleItemConnection, ModuleItemKind, TraitItemKind,
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
            } => Some(match module_item_kind {
                ModuleItemKind::Type(ty_kind) => {
                    TypePath::new(self.db, self.module_path, ident, connection, ty_kind).into()
                }
                ModuleItemKind::Form(form_kind) => {
                    FormPath::new(self.db, self.module_path, ident, connection, form_kind).into()
                }
                ModuleItemKind::Trait => {
                    TraitPath::new(self.db, self.module_path, ident, connection).into()
                }
            }),
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
                Token::Attr(_) => todo!(),
                Token::Keyword(_) => todo!(),
                Token::Identifier(_) => todo!(),
                Token::Punctuation(_) => todo!(),
                Token::WordOpr(_) => todo!(),
                Token::Literal(_) => todo!(),
                Token::Err(_) => todo!(),
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
    fn parse_head(
        mut self,
    ) -> Result<(Accessibility, EntityKind, IdentifierToken, bool, TokenIdx), AstError> {
        let accessibility = self.parse_accessibility()?;
        let kw = self.take_entity_kind_keyword()?;
        let ident: IdentifierToken = self.parse_expected()?;
        let is_generic = self.parse_is_generic();
        let entity_kind = match self.ast_context_kind() {
            AstContextKind::InsideTrait { module_item_path } => match kw {
                Keyword::Config(_) => todo!(),
                Keyword::Paradigm(kw) => {
                    let trai_item_kind: TraitItemKind = if let Some(token) =
                        self.token_stream_mut().peek()
                    {
                        match token {
                            Token::Punctuation(special_token) => match special_token {
                                Punctuation::Bra(Bracket::Par) | Punctuation::LAngle => match kw {
                                    FormKeyword::Def => todo!(),
                                    FormKeyword::Func
                                    | FormKeyword::Proc
                                    | FormKeyword::Fn
                                    | FormKeyword::Function => TraitItemKind::Method,
                                    FormKeyword::Theorem => todo!(),
                                    FormKeyword::Lemma => todo!(),
                                    FormKeyword::Proposition => todo!(),
                                },
                                Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                                    todo!()
                                }
                                unexpected_special_token => {
                                    todo!("unexpected_special_token = {unexpected_special_token:?}")
                                }
                            },
                            ref unexpected_token => todo!(),
                        }
                    } else {
                        todo!()
                    };
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TraitItem(trai_item_kind),
                    }
                }
                Keyword::Type(kw) => {
                    let trai_item_kind: TraitItemKind = match kw {
                        TypeKeyword::Type => TraitItemKind::AssociatedType,
                        TypeKeyword::Struct => todo!(),
                        TypeKeyword::Enum => todo!(),
                        TypeKeyword::Record => todo!(),
                        TypeKeyword::Structure => todo!(),
                        TypeKeyword::Inductive => todo!(),
                    };
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TraitItem(trai_item_kind),
                    }
                }
                Keyword::Stmt(_) => todo!(),
                Keyword::Liason(_) => todo!(),
                Keyword::Main => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Visual => todo!(),
                Keyword::Impl => todo!(),
                Keyword::Trait => todo!(),
                Keyword::End(_) => todo!(),
                Keyword::Connection(_) => todo!(),
            },
            AstContextKind::InsideEnumLikeType { module_item_path } => todo!(),
            AstContextKind::InsideForm => match kw {
                Keyword::Config(_) => todo!(),
                Keyword::Paradigm(_) => todo!(),
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Liason(_) => todo!(),
                Keyword::Main => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Visual => todo!(),
                Keyword::Impl => todo!(),
                Keyword::Trait => todo!(),
                Keyword::End(_) => todo!(),
                Keyword::Connection(_) => todo!(),
            },
            AstContextKind::InsideTypeImpl => match kw {
                Keyword::Config(_) => todo!(),
                Keyword::Paradigm(kw) => {
                    let type_item_kind: TypeItemKind = if let Some(token) =
                        self.token_stream_mut().peek()
                    {
                        match token {
                            Token::Punctuation(special_token) => match special_token {
                                Punctuation::Bra(Bracket::Par) | Punctuation::LAngle => match kw {
                                    FormKeyword::Def => todo!(),
                                    FormKeyword::Func
                                    | FormKeyword::Proc
                                    | FormKeyword::Fn
                                    | FormKeyword::Function => TypeItemKind::Method,
                                    FormKeyword::Theorem => todo!(),
                                    FormKeyword::Lemma => todo!(),
                                    FormKeyword::Proposition => todo!(),
                                },
                                Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                                    TypeItemKind::Memo
                                }
                                unexpected_special_token => {
                                    todo!("unexpected_special_token = {unexpected_special_token:?}")
                                }
                            },
                            ref unexpected_token => todo!(),
                        }
                    } else {
                        todo!()
                    };
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TypeImplItem(type_item_kind),
                    }
                }
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Liason(_) => todo!(),
                Keyword::Main => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Visual => todo!(),
                Keyword::Impl => todo!(),
                Keyword::Trait => todo!(),
                Keyword::End(_) => todo!(),
                Keyword::Connection(_) => todo!(),
            },
            AstContextKind::InsideTraitImpl => match kw {
                Keyword::Config(_) => todo!(),
                Keyword::Paradigm(kw) => {
                    let form_kind = if let Some(token) = self.token_stream_mut().peek() {
                        match token {
                            Token::Punctuation(special_token) => match special_token {
                                Punctuation::Bra(Bracket::Par) | Punctuation::LAngle => match kw {
                                    FormKeyword::Def => todo!(),
                                    FormKeyword::Func
                                    | FormKeyword::Proc
                                    | FormKeyword::Fn
                                    | FormKeyword::Function => FormKind::Function,
                                    FormKeyword::Theorem => todo!(),
                                    FormKeyword::Lemma => todo!(),
                                    FormKeyword::Proposition => todo!(),
                                },
                                Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                                    FormKind::Feature
                                }
                                unexpected_special_token => {
                                    todo!("unexpected_special_token = {unexpected_special_token:?}")
                                }
                            },
                            ref unexpected_token => todo!(),
                        }
                    } else {
                        todo!()
                    };
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TraitImplItem(todo!()),
                    }
                }
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Liason(_) => todo!(),
                Keyword::Main => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Visual => todo!(),
                Keyword::Impl => todo!(),
                Keyword::Trait => todo!(),
                Keyword::End(_) => todo!(),
                Keyword::Connection(_) => todo!(),
            },
            AstContextKind::InsideModule => match kw {
                Keyword::Config(_) => todo!(),
                Keyword::Paradigm(kw) => {
                    let form_kind = if let Some(token) = self.token_stream_mut().peek() {
                        match token {
                            Token::Punctuation(special_token) => match special_token {
                                Punctuation::Bra(Bracket::Par) | Punctuation::LAngle => match kw {
                                    FormKeyword::Def => todo!(),
                                    FormKeyword::Func
                                    | FormKeyword::Proc
                                    | FormKeyword::Fn
                                    | FormKeyword::Function => FormKind::Function,
                                    FormKeyword::Theorem => todo!(),
                                    FormKeyword::Lemma => todo!(),
                                    FormKeyword::Proposition => todo!(),
                                },
                                Punctuation::Binary(BinaryOpr::Curry) | Punctuation::Colon => {
                                    FormKind::Feature
                                }
                                unexpected_special_token => {
                                    todo!("unexpected_special_token = {unexpected_special_token:?}")
                                }
                            },
                            ref unexpected_token => todo!(),
                        }
                    } else {
                        todo!()
                    };
                    EntityKind::ModuleItem {
                        module_item_kind: ModuleItemKind::Form(form_kind).into(),
                        connection: ModuleItemConnection::Connected,
                    }
                }
                Keyword::Type(kw) => {
                    let type_kind = match kw {
                        TypeKeyword::Type => TypeKind::Foreign,
                        TypeKeyword::Struct => TypeKind::Struct,
                        TypeKeyword::Enum => TypeKind::Enum,
                        TypeKeyword::Record => TypeKind::Record,
                        TypeKeyword::Structure => TypeKind::Structure,
                        TypeKeyword::Inductive => TypeKind::Inductive,
                    };
                    EntityKind::ModuleItem {
                        module_item_kind: ModuleItemKind::Type(type_kind).into(),
                        connection: ModuleItemConnection::Connected,
                    }
                }
                Keyword::Stmt(_) => todo!(),
                Keyword::Liason(_) => todo!(),
                Keyword::Main => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => EntityKind::Module,
                Keyword::Visual => todo!(),
                Keyword::Impl => todo!(),
                Keyword::Trait => EntityKind::ModuleItem {
                    module_item_kind: ModuleItemKind::Trait.into(),
                    connection: ModuleItemConnection::Connected,
                },
                Keyword::End(_) => todo!(),
                Keyword::Connection(_) => todo!(),
            },
            AstContextKind::InsideMatchStmt => todo!(),
            AstContextKind::InsideNoChild => return Err(AstError::ExpectNothing),
        };
        Ok((
            accessibility,
            entity_kind,
            ident,
            is_generic,
            self.finish_with_saved_stream_state(),
        ))
    }
}
