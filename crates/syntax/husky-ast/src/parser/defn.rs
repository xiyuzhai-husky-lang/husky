use husky_entity_path::*;
use husky_entity_taxonomy::{
    AssociatedItemKind, FormKind, ModuleItemConnectionKind, ModuleItemKind, TraitItemKind,
    TypeItemKind, TypeKind,
};
use husky_opn_syntax::{BinaryOpr, Bracket};
use husky_print_utils::p;
use husky_token::{EntityKeywordGroup, FormKeyword, TokenParseContext, TypeEntityKeyword};
use parsec::{ParseFromStreamWithError, StreamParser};
use salsa::DebugWithDb;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_defn<C: NormalAstChildren>(
        &mut self,
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        state: Option<TokenIdx>,
    ) -> Ast {
        self.parse_defn_aux::<C>(token_group_idx, visibility_expr, state)
            .unwrap_or_else(|error| Ast::Err {
                token_group_idx,
                error,
            })
    }

    fn parse_defn_aux<C: NormalAstChildren>(
        &mut self,
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        state: Option<TokenIdx>,
    ) -> AstResult<Ast> {
        let mut aux_parser = BasicAuxAstParser::new(
            self.db,
            self.module_path,
            self.token_sheet
                .token_group_token_stream(token_group_idx, state),
        );
        let (entity_kind, ident_token, is_generic, saved_stream_state) =
            aux_parser.parse_head::<C>()?;
        let ident = ident_token.ident();
        let block = match entity_kind {
            EntityKind::Module => DefnBlock::Submodule {
                path: ModulePath::new_child(self.db, self.module_path, ident).into(),
            },
            EntityKind::ModuleItem {
                module_item_kind,
                connection,
            } => {
                let connection = self.new_connection(ident, connection);
                match module_item_kind {
                    ModuleItemKind::Type(ty_kind) => DefnBlock::Type {
                        path: TypePath::new(self.db, self.module_path, ident, connection, ty_kind)
                            .into(),
                        variants: match ty_kind {
                            TypeKind::Enum | TypeKind::Inductive => todo!(),
                            _ => None,
                        },
                    },
                    ModuleItemKind::Form(form_kind) => DefnBlock::Form {
                        path: FormPath::new(
                            self.db,
                            self.module_path,
                            ident,
                            connection,
                            form_kind,
                        )
                        .into(),
                        body: todo!(),
                    },
                    ModuleItemKind::Trait => DefnBlock::Trait {
                        path: TraitPath::new(self.db, self.module_path, ident, connection).into(),
                        items: self.parse_expected(OriginalAstError::ExpectedTraitItems)?,
                    },
                }
            }
            EntityKind::AssociatedItem { .. } => DefnBlock::AssociatedItem {
                body: self.parse()?,
            },
            EntityKind::Variant => todo!(),
        };
        Ok(Ast::Defn {
            visibility_expr,
            ident_token,
            is_generic,
            token_group_idx,
            block,
            entity_kind,
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
    /// parse variants of enum or inductive types
    fn parse_ty_variants(&mut self) -> AstIdxRange {
        let mut ty_variants = vec![];
        while let Some((token_group_idx, token_group, first_noncomment_token)) = self
            .token_groups
            .peek_token_group_of_exact_indent_with_its_first_token(self.indent())
        {
            match first_noncomment_token {
                Token::Punctuation(Punctuation::VERTICAL) => {
                    self.token_groups.next();
                    ty_variants.push(self.parse_ty_variant(token_group_idx))
                }
                _ => break,
            }
        }
        self.alloc_asts(ty_variants)
    }

    fn parse_ty_variant(&mut self, token_group_idx: TokenGroupIdx) -> Ast {
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
        Ast::TypeVariant {
            token_group_idx,
            path: TypeVariantPath::new(self.db, module_item_path, ident),
            ident: todo!(),
        }
    }
}

impl<'a> BasicAuxAstParser<'a> {
    fn parse_head<C: NormalAstChildren>(
        mut self,
    ) -> AstResult<(EntityKind, IdentToken, bool, TokenIdx)> {
        let entity_keyword_group =
            self.parse_expected(OriginalAstError::ExpectedEntityKeywordGroup)?;
        let ident: IdentToken = self.parse_expected(OriginalAstError::ExpectedIdent)?;
        let is_generic = self.parse_is_generic();
        let entity_kind = C::determine_entity_kind(entity_keyword_group)?;
        Ok((
            entity_kind,
            ident,
            is_generic,
            self.finish_with_saved_stream_state(),
        ))
    }
}
