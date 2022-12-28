use husky_entity_path::*;
use husky_entity_taxonomy::{FormKind, ModuleItemConnection, ModuleItemKind, TypeKind};
use husky_opn_syntax::{BinaryOpr, Bracket};
use husky_print_utils::p;
use husky_token::{FormKeyword, TypeKeyword};
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
                .token_group_token_iter(token_group_idx, None),
        );
        let (accessibility, entity_kind, ident, is_generic, saved_stream_state) =
            aux_parser.parse_head()?;
        let entity_path: Option<husky_entity_path::EntityPath> = match entity_kind {
            EntityKind::Module => {
                Some(ModulePath::new_child(self.db, self.module_path, ident).into())
            }
            EntityKind::ModuleItem {
                item_kind: module_item_kind,
                connection,
            } => match connection {
                ModuleItemConnection::Connected => {
                    Some(ConnectedModuleItemPath::new(self.db, self.module_path, ident).into())
                }
                ModuleItemConnection::Disconnected => todo!(),
            },
            EntityKind::AssociatedItem { .. } => {
                ctx.kind().module_item_path().map(|module_item_path| {
                    AssociatedItemPath::new(self.db, module_item_path, ident).into()
                })
            }
            EntityKind::Variant => todo!(),
        };
        let ast_ctx_kind = AstContextKind::inside_defn(entity_kind, entity_path);
        let (body, body_kind) = {
            let body = self.parse_asts(ctx.subcontext(ast_ctx_kind));
            match body.last() {
                Some(_) => (body, DefnBodyKind::Block),
                None => match self.token_groups.peek_with_exact_indent(ctx.indent()) {
                    Some((_token_group_idx, token_group)) => match token_group.first().kind {
                        TokenKind::Special(SpecialToken::Vertical) => (
                            self.parse_enum_variants(ctx.subcontext(ast_ctx_kind)),
                            DefnBodyKind::EnumVariants,
                        ),
                        _ => (Default::default(), DefnBodyKind::None),
                    },
                    None => (Default::default(), DefnBodyKind::None),
                },
            }
        };
        Ok(Ast::Defn {
            // order matters!
            accessibility,
            ident,
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
        while let Some((idx, token_group)) =
            self.token_groups.peek_with_exact_indent(context.indent())
        {
            match token_group.first().kind {
                TokenKind::Special(SpecialToken::Vertical) => {
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
            Some(token) => match token.kind {
                TokenKind::Attr(_) => todo!(),
                TokenKind::Keyword(_) => todo!(),
                TokenKind::Identifier(_) => todo!(),
                TokenKind::Special(_) => todo!(),
                TokenKind::WordOpr(_) => todo!(),
                TokenKind::Literal(_) => todo!(),
                TokenKind::Comment => todo!(),
                TokenKind::Err(_) => todo!(),
            },
            None => todo!(),
        };
        let module_item_path = todo!();
        Ast::ModuleItemVariant {
            token_group_idx,
            module_item_variant_path: ModuleItemVariantPath::new(self.db, module_item_path, ident),
            ident: todo!(),
        }
    }
}

impl<'a> BasicAuxAstParser<'a> {
    fn parse_head(
        mut self,
    ) -> Result<(Accessibility, EntityKind, Identifier, bool, TokenIterState), AstError> {
        let accessibility = self.parse_accessibility()?;
        let entity_kind_keyword = self.take_entity_kind_keyword()?;
        let ident = self.parse_ident()?;
        let is_generic = self.parse_is_generic();
        let coarse_item_kind: CoarseEntityKind = match entity_kind_keyword {
            Keyword::Config(_) => todo!(),
            Keyword::Paradigm(kw) => {
                let form_kind = if let Some(token) = self.token_iter().peek() {
                    match token.kind {
                        TokenKind::Special(special_token) => match special_token {
                            SpecialToken::Bra(Bracket::Par) | SpecialToken::LAngle => match kw {
                                FormKeyword::Def => todo!(),
                                FormKeyword::Func
                                | FormKeyword::Proc
                                | FormKeyword::Fn
                                | FormKeyword::Function => FormKind::Function,
                                FormKeyword::Theorem => todo!(),
                                FormKeyword::Lemma => todo!(),
                                FormKeyword::Proposition => todo!(),
                            },
                            SpecialToken::BinaryOpr(BinaryOpr::Curry) | SpecialToken::Colon => {
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
                ModuleItemKind::Form(form_kind).into()
            }
            Keyword::Type(kw) => {
                let type_kind = match kw {
                    TypeKeyword::Type => TypeKind::Alias,
                    TypeKeyword::Struct => TypeKind::Struct,
                    TypeKeyword::Enum => TypeKind::Enum,
                    TypeKeyword::Record => TypeKind::Record,
                    TypeKeyword::Structure => TypeKind::Structure,
                    TypeKeyword::Inductive => TypeKind::Inductive,
                };
                ModuleItemKind::Type(type_kind).into()
            }
            Keyword::Stmt(_) => todo!(),
            Keyword::Liason(_) => todo!(),
            Keyword::Main => todo!(),
            Keyword::Use => todo!(),
            Keyword::Mod => CoarseEntityKind::Module,
            Keyword::Visual => todo!(),
            Keyword::Impl => todo!(),
            Keyword::Trait => ModuleItemKind::Trait.into(),
            Keyword::End(_) => todo!(),
        };
        let entity_kind = match self.ast_context_kind() {
            AstContextKind::InsideTrait { module_item_path } => match coarse_item_kind {
                CoarseEntityKind::Module => todo!(),
                CoarseEntityKind::Item(item_kind) => EntityKind::AssociatedItem {
                    item_kind: item_kind,
                },
                CoarseEntityKind::Variant => todo!(),
            },
            AstContextKind::InsideEnumLikeType { module_item_path } => todo!(),
            AstContextKind::InsideForm => match coarse_item_kind {
                CoarseEntityKind::Module => todo!(),
                CoarseEntityKind::Item(item_kind) => {
                    match item_kind {
                        ModuleItemKind::Type(_) => todo!(),
                        ModuleItemKind::Trait => todo!(),
                        ModuleItemKind::Form(_) => todo!(),
                    }
                    p!(self.text_start(), self.module_path().debug(self.db()));
                    todo!();
                    EntityKind::ModuleItem {
                        item_kind,
                        connection: ModuleItemConnection::Disconnected,
                    }
                }
                CoarseEntityKind::Variant => todo!(),
            },
            AstContextKind::InsideImpl => match coarse_item_kind {
                CoarseEntityKind::Module => todo!(),
                CoarseEntityKind::Item(item_kind) => match item_kind {
                    ModuleItemKind::Type(_) => todo!(),
                    ModuleItemKind::Trait => todo!(),
                    ModuleItemKind::Form(_) => EntityKind::AssociatedItem { item_kind },
                },
                CoarseEntityKind::Variant => todo!(),
            },
            AstContextKind::InsideModule => match coarse_item_kind {
                CoarseEntityKind::Module => EntityKind::Module,
                CoarseEntityKind::Item(item_kind) => EntityKind::ModuleItem {
                    item_kind,
                    connection: ModuleItemConnection::Connected,
                },
                CoarseEntityKind::Variant => todo!(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CoarseEntityKind {
    Module,
    Item(ModuleItemKind),
    Variant,
}

impl From<ModuleItemKind> for CoarseEntityKind {
    fn from(v: ModuleItemKind) -> Self {
        Self::Item(v)
    }
}
