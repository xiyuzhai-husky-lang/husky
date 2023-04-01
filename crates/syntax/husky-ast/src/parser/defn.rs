use husky_entity_path::*;
use husky_entity_taxonomy::{
    AssociatedItemKind, FormKind, ModuleItemConnectionKind, ModuleItemKind, TraitItemKind,
    TypeItemKind, TypeKind,
};
use husky_opn_syntax::{BinaryOpr, Bracket};
use husky_print_utils::p;
use husky_token::{EntityKeywordGroup, FormKeyword, TokenParseContext, TypeEntityKeyword};
use parsec::{ParseContext, ParseFrom};
use salsa::DebugWithDb;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_defn(
        &mut self,
        context: &Context,
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        state: Option<TokenIdx>,
    ) -> Ast {
        self.parse_defn_aux(context, token_group_idx, visibility_expr, state)
            .unwrap_or_else(|error| Ast::Err {
                token_group_idx,
                error,
            })
    }

    fn parse_defn_aux(
        &mut self,
        ctx: &Context,
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        state: Option<TokenIdx>,
    ) -> AstResult<Ast> {
        let mut aux_parser = BasicAuxAstParser::new(
            self.db,
            ctx,
            self.module_path,
            self.token_sheet
                .token_group_token_stream(token_group_idx, state),
        );
        let (entity_kind, ident_token, is_generic, saved_stream_state) = aux_parser.parse_head()?;
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
                            Token::Punctuation(Punctuation::VERTICAL) => (
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
            visibility_expr,
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
                Token::Punctuation(Punctuation::VERTICAL) => {
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
    fn parse_head(mut self) -> AstResult<(EntityKind, IdentToken, bool, TokenIdx)> {
        let entity_keyword_group =
            self.parse_expected(OriginalAstError::ExpectedEntityKeywordGroup)?;
        let ident: IdentToken = self.parse_expected(OriginalAstError::ExpectedIdent)?;
        let is_generic = self.parse_is_generic();
        let entity_kind = match self.ast_context_kind() {
            AstContextKind::InsideTrait { module_item_path } => {
                self.determine_entity_kind_inside_trai(entity_keyword_group)?
            }
            AstContextKind::InsideEnumLikeType { module_item_path } => todo!(),
            AstContextKind::InsideForm => {
                self.determine_form_item_entity_kind(entity_keyword_group)?
            }
            AstContextKind::InsideTypeImplBlock => {
                self.determine_entity_kind_inside_ty_impl_block(entity_keyword_group)?
            }
            AstContextKind::InsideTraitForTypeImplBlock => match entity_keyword_group {
                EntityKeywordGroup::Mod(_) => todo!(),
                EntityKeywordGroup::Fn(_) => EntityKind::AssociatedItem {
                    associated_item_kind: AssociatedItemKind::TraitForTypeItem(
                        TraitItemKind::MethodFn,
                    ),
                },
                EntityKeywordGroup::ConstFn(_, _) => todo!(),
                EntityKeywordGroup::StaticFn(_, _) => todo!(),
                EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
                EntityKeywordGroup::Gn(_) => todo!(),
                EntityKeywordGroup::GeneralDef(_) => todo!(),
                EntityKeywordGroup::TypeEntity(_) => todo!(),
                EntityKeywordGroup::Type(_) => todo!(),
                EntityKeywordGroup::Trait(_) => todo!(),
                EntityKeywordGroup::Visual(_) => todo!(),
                EntityKeywordGroup::Var(_) => todo!(),
            },
            AstContextKind::InsideModule => {
                self.determine_module_item_entity_kind(entity_keyword_group)?
            }
            AstContextKind::InsideMatchStmt => todo!(),
            AstContextKind::InsideNoChild => return Err(OriginalAstError::ExpectNothing.into()),
        };
        Ok((
            entity_kind,
            ident,
            is_generic,
            self.finish_with_saved_stream_state(),
        ))
    }

    fn determine_module_item_entity_kind(
        &self,
        entity_keyword_group: EntityKeywordGroup,
    ) -> AstResult<EntityKind> {
        let module_item_kind: ModuleItemKind = match entity_keyword_group {
            EntityKeywordGroup::Mod(_) => return Ok(EntityKind::Module),
            EntityKeywordGroup::Fn(_) => FormKind::Fn.into(),
            EntityKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKeywordGroup::Var(_) => FormKind::Var.into(),
            EntityKeywordGroup::Gn(_) => FormKind::Gn.into(),
            EntityKeywordGroup::GeneralDef(_) => todo!(),
            EntityKeywordGroup::TypeEntity(token) => token.type_kind().into(),
            EntityKeywordGroup::Type(_) => FormKind::TypeAlias.into(),
            EntityKeywordGroup::Trait(_) => ModuleItemKind::Trait,
            EntityKeywordGroup::Visual(_) => todo!(),
        };
        Ok(EntityKind::ModuleItem {
            module_item_kind,
            connection: ModuleItemConnectionKind::Connected,
        })
    }

    fn determine_entity_kind_inside_ty_impl_block(
        &self,
        entity_keyword_group: EntityKeywordGroup,
    ) -> AstResult<EntityKind> {
        let ty_item_kind = match entity_keyword_group {
            EntityKeywordGroup::Mod(_) => todo!(),
            EntityKeywordGroup::Fn(_) => TypeItemKind::MethodFn,
            EntityKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKeywordGroup::StaticFn(_, _) => TypeItemKind::AssociatedFn,
            EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKeywordGroup::Gn(_) => todo!(),
            EntityKeywordGroup::GeneralDef(_) => todo!(),
            EntityKeywordGroup::TypeEntity(_) => {
                Err(OriginalAstError::UnexpectedTypeDefnInsideTypeImplBlock)?
            }
            EntityKeywordGroup::Type(_) => todo!(),
            EntityKeywordGroup::Trait(_) => todo!(),
            EntityKeywordGroup::Visual(_) => todo!(),
            EntityKeywordGroup::Var(_) => TypeItemKind::Memo,
        };
        Ok(EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TypeItem(ty_item_kind),
        })
    }

    fn determine_entity_kind_inside_trai(
        &self,
        entity_keyword_group: EntityKeywordGroup,
    ) -> AstResult<EntityKind> {
        let trai_item_kind = match entity_keyword_group {
            EntityKeywordGroup::Mod(_) => todo!(),
            EntityKeywordGroup::Fn(_) => TraitItemKind::MethodFn,
            EntityKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKeywordGroup::Gn(_) => todo!(),
            EntityKeywordGroup::GeneralDef(_) => todo!(),
            EntityKeywordGroup::TypeEntity(_) => todo!(),
            EntityKeywordGroup::Type(_) => TraitItemKind::AssociatedType,
            EntityKeywordGroup::Trait(_) => Err(OriginalAstError::UnexpectedTraitInsideTrait)?,
            EntityKeywordGroup::Visual(_) => todo!(),
            EntityKeywordGroup::Var(_) => todo!(),
        };
        Ok(EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TraitItem(trai_item_kind),
        })
    }

    fn determine_form_item_entity_kind(
        &self,
        entity_keyword_group: EntityKeywordGroup,
    ) -> AstResult<EntityKind> {
        let module_item_kind = match entity_keyword_group {
            EntityKeywordGroup::Mod(_) => Err(OriginalAstError::UnexpectedModInsideForm)?,
            EntityKeywordGroup::Fn(_) => FormKind::Fn.into(),
            EntityKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKeywordGroup::Gn(_) => todo!(),
            EntityKeywordGroup::GeneralDef(_) => todo!(),
            EntityKeywordGroup::TypeEntity(token) => token.type_kind().into(),
            EntityKeywordGroup::Type(_) => todo!(),
            EntityKeywordGroup::Trait(_) => todo!(),
            EntityKeywordGroup::Visual(_) => todo!(),
            EntityKeywordGroup::Var(_) => FormKind::Var.into(),
        };
        Ok(EntityKind::ModuleItem {
            module_item_kind,
            connection: ModuleItemConnectionKind::Disconnected,
        })
    }
}
