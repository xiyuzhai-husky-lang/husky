use husky_entity_path::*;
use husky_entity_taxonomy::{
    AssociatedItemKind, FormKind, ModuleItemConnectionKind, ModuleItemKind, TraitItemKind,
    TypeItemKind, TypeKind,
};
use husky_opn_syntax::{BinaryOpr, Bracket};
use husky_print_utils::p;
use husky_token::{EntityKindKeywordGroup, FormKeyword, TokenParseContext, TypeEntityKeyword};
use parsec::{ParseFromStream, StreamParser};
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
                    ModuleItemKind::Type(ty_kind) => {
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
                    ModuleItemKind::Form(form_kind) => DefnBlock::Form {
                        path: FormPath::new(
                            self.db,
                            self.module_path,
                            ident,
                            connection,
                            form_kind,
                        )
                        .into(),
                        body: self.parse()?, // todo: check that this is coherent with decl
                    },
                    ModuleItemKind::Trait => DefnBlock::Trait {
                        path: TraitPath::new(self.db, self.module_path, ident, connection).into(),
                        items: self.parse()?,
                    },
                }
            }
            EntityKind::AssociatedItem { .. } => DefnBlock::AssociatedItem {
                body: self.parse()?,
            },
            EntityKind::TypeVariant => todo!(),
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
    #[inline(always)]
    pub(crate) fn parse_ty_variants(&mut self, path: TypePath) -> AstIdxRange {
        let mut ty_variants = vec![];
        loop {
            let state = self.token_groups.state();
            let Some((token_group_idx, _)) = self.token_groups.next() else{
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
            let Ok(Some(vertical_token)) = aux_parser.parse::<VerticalToken>() else {
                self.token_groups.rollback(state);
                break
            };
            ty_variants.push(
                match aux_parser
                    .parse_expected::<IdentToken, _>(OriginalAstError::ExpectedIdentForTypeVariant)
                {
                    Ok(ident_token) => {
                        for ty_variant in &ty_variants {
                            match ty_variant {
                                Ast::TypeVariant {
                                    ident_token: prev_ident_token,
                                    ..
                                } if prev_ident_token.ident() == ident_token.ident() => {
                                    todo!()
                                }
                                _ => (),
                            }
                        }
                        Ast::TypeVariant {
                            token_group_idx,
                            path: TypeVariantPath::new(self.db, path, ident_token.ident()),
                            vertical_token,
                            ident_token,
                            state_after: aux_parser.save_state(),
                        }
                    }
                    Err(_) => todo!(),
                },
            )
        }
        self.alloc_asts(ty_variants)
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
