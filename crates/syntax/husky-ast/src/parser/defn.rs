use husky_entity_path::*;
use husky_entity_taxonomy::{
    AssociatedItemKind, FugitiveKind, MajorItemConnectionKind, MajorItemKind, TraitItemKind,
    TypeItemKind, TypeKind,
};
use husky_opr::{BinaryOpr, Bracket};
use husky_print_utils::p;
use husky_token::{EntityKindKeywordGroup, FugitiveKeyword, TokenStreamParser, TypeEntityKeyword};
use parsec::{StreamParser, TryParseOptionFromStream};
use salsa::DebugWithDb;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_defn<C: NormalAstChildren>(
        &mut self,
        token_group_idx: TokenGroupIdx,
        visibility_expr: VisibilityExpr,
        state: Option<TokenStreamState>,
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
        state: Option<TokenStreamState>,
    ) -> AstResult<Ast> {
        let mut aux_parser = BasicAuxAstParser::new(
            self.db,
            self.module_path,
            self.token_sheet
                .token_group_token_stream(token_group_idx, state),
        );
        let (item_kind, ident_token, is_generic, saved_stream_state) =
            aux_parser.parse_head::<C>()?;
        let ident = ident_token.ident();
        let block = match item_kind {
            EntityKind::Module => DefnBlock::Submodule {
                path: ModulePath::new_child(self.db, self.module_path, ident),
            },
            EntityKind::MajorItem {
                module_item_kind,
                connection,
            } => {
                let connection = self.new_connection(ident, connection);
                match module_item_kind {
                    MajorItemKind::Type(ty_kind) => {
                        let path =
                            TypePath::new(self.db, self.module_path, ident, connection, ty_kind)
                                .into();
                        DefnBlock::Type {
                            path,
                            variants: match ty_kind {
                                TypeKind::Enum | TypeKind::Inductive => {
                                    Some(self.parse_expected_with_context(
                                        path,
                                        OriginalAstError::ExpectedTypeVariants,
                                    )?)
                                }
                                _ => None,
                            },
                        }
                    }
                    MajorItemKind::Fugitive(form_kind) => DefnBlock::Fugitive {
                        path: FugitivePath::new(
                            self.db,
                            self.module_path,
                            ident,
                            connection,
                            form_kind,
                        )
                        .into(),
                        body: self.try_parse_option()?, // todo: check that this is coherent with decl
                    },
                    MajorItemKind::Trait => DefnBlock::Trait {
                        path: TraitPath::new(self.db, self.module_path, ident, connection).into(),
                        items: self.try_parse_option()?,
                    },
                }
            }
            EntityKind::AssociatedItem { .. } => DefnBlock::AssociatedItem {
                body: self.try_parse_option()?,
            },
            EntityKind::TypeVariant => todo!(),
            EntityKind::Trait => todo!(),
            EntityKind::ImplBlock => todo!(),
            EntityKind::Decr => todo!(),
        };
        Ok(Ast::Defn {
            visibility_expr,
            ident_token,
            is_generic,
            token_group_idx,
            block,
            item_kind,
            saved_stream_state,
        })
    }

    #[inline(always)]
    fn new_connection(
        &mut self,
        ident: Ident,
        kind: MajorItemConnectionKind,
    ) -> MajorItemConnection {
        match kind {
            MajorItemConnectionKind::Connected => MajorItemConnection::Connected,
            MajorItemConnectionKind::Disconnected => {
                MajorItemConnection::Disconnected(self.disambiguator_registry.issue_new(ident))
            }
        }
    }
    /// parse variants of enum or inductive types
    #[inline(always)]
    pub(crate) fn parse_ty_variants(&mut self, path: TypePath) -> AstIdxRange {
        let mut ty_variants = vec![];
        loop {
            let state = self.token_groups.state();
            let Some((token_group_idx, _)) = self.token_groups.next() else {
                break;
            };
            // todo: change the api of `self.token_groups.next()`
            // it should directly return a token stream
            let mut aux_parser = BasicAuxAstParser::new(
                self.db,
                self.module_path,
                self.token_sheet
                    .token_group_token_stream(token_group_idx, None),
            );
            let Ok(Some(vertical_token)) = aux_parser.try_parse_option::<VerticalToken>() else {
                self.token_groups.rollback(state);
                break;
            };
            ty_variants.push(
                match aux_parser.try_parse_expected::<IdentToken, _>(
                    OriginalAstError::ExpectedIdentForTypeVariant,
                ) {
                    Ok(ident_token) => Ast::TypeVariant {
                        token_group_idx,
                        variant_path: TypeVariantPath::new(self.db, path, ident_token.ident()),
                        vertical_token,
                        ident_token,
                        state_after: aux_parser.save_state(),
                    },
                    Err(error) => Ast::Err {
                        token_group_idx,
                        error,
                    },
                },
            )
        }
        self.alloc_asts(ty_variants)
    }
}

impl<'a> BasicAuxAstParser<'a> {
    fn parse_head<C: NormalAstChildren>(
        mut self,
    ) -> AstResult<(EntityKind, IdentToken, bool, TokenStreamState)> {
        let item_keyword_group =
            self.try_parse_expected(OriginalAstError::ExpectedEntityKeywordGroup)?;
        let ident: IdentToken = self.try_parse_expected(OriginalAstError::ExpectedIdent)?;
        let is_generic = self.parse_is_generic();
        let item_kind = C::determine_item_kind(item_keyword_group)?;
        Ok((
            item_kind,
            ident,
            is_generic,
            self.finish_with_saved_stream_state(),
        ))
    }
}
