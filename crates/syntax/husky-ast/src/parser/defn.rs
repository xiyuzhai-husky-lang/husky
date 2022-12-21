use husky_entity_kind::{ModuleItemKind, TypeKind};
use husky_entity_path::{ModuleItemPath, ModuleItemVariantPath};
use husky_print_utils::p;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_defn(&mut self, context: &Context, token_group_idx: TokenGroupIdx) -> Ast {
        self.parse_defn_aux(context, token_group_idx)
            .unwrap_or_else(|error| Ast::Err {
                token_group_idx,
                error,
            })
    }

    fn parse_defn_aux(
        &mut self,
        context: &Context,
        token_group_idx: TokenGroupIdx,
    ) -> AstResult<Ast> {
        let mut aux_parser = self.token_sheet.token_group_token_iter(token_group_idx);
        let (accessibility, entity_kind, ident, is_generic) = parse_head(aux_parser)?;
        let (body, body_kind) = {
            let ast_parent = match entity_kind {
                EntityKind::Module | EntityKind::EnumVariant => AstParent::NoChild,
                EntityKind::ModuleItem(module_item_kind) => match module_item_kind {
                    ModuleItemKind::Type(ty_kind) => match context.parent() {
                        AstParent::Form => match ty_kind {
                            TypeKind::Enum | TypeKind::Inductive => AstParent::EnumLike,
                            TypeKind::Record | TypeKind::Struct | TypeKind::Structure => {
                                AstParent::TraitOrNonEnumLikeType
                            }
                            TypeKind::Form => AstParent::Form,
                        },
                        AstParent::TraitOrNonEnumLikeType => AstParent::Form,
                        AstParent::Module => AstParent::Form,
                        AstParent::MatchStmt | AstParent::EnumLike => unreachable!(),
                        AstParent::NoChild => todo!(),
                        AstParent::Impl => todo!(),
                    },
                    ModuleItemKind::Trait => AstParent::TraitOrNonEnumLikeType,
                    ModuleItemKind::Form => AstParent::Form,
                },
            };
            let body = self.parse_asts(context.subcontext(ast_parent));
            match body.last() {
                Some(_) => (body, DefnBodyKind::Block),
                None => match self.token_groups.peek_with_exact_indent(context.indent()) {
                    Some((_token_group_idx, token_group)) => match token_group.first().kind {
                        TokenKind::Special(SpecialToken::Vertical) => (
                            self.parse_enum_variants(context.subcontext(ast_parent)),
                            DefnBodyKind::EnumVariants,
                        ),
                        _ => (Default::default(), DefnBodyKind::None),
                    },
                    None => (Default::default(), DefnBodyKind::None),
                },
            }
        };
        let entity_path = match entity_kind {
            EntityKind::Module => {
                EntityPath::Module(ModulePath::new_child(self.db, self.module_path, ident))
            }
            EntityKind::ModuleItem(_) => {
                EntityPath::ModuleItem(ModuleItemPath::new(self.db, self.module_path, ident))
            }
            EntityKind::EnumVariant => todo!(),
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
                TokenKind::Decorator(_) => todo!(),
                TokenKind::Keyword(_) => todo!(),
                TokenKind::Identifier(_) => todo!(),
                TokenKind::Special(_) => todo!(),
                TokenKind::WordOpr(_) => todo!(),
                TokenKind::Literal(_) => todo!(),
                TokenKind::Unrecognized(_) => todo!(),
                TokenKind::IllFormedLiteral(_) => todo!(),
                TokenKind::Comment => todo!(),
            },
            None => todo!(),
        };
        let module_item_path = match context.parent() {
            AstParent::Form => todo!(),
            AstParent::EnumLike => todo!(),
            AstParent::TraitOrNonEnumLikeType => todo!(),
            AstParent::Module => todo!(),
            AstParent::MatchStmt => todo!(),
            AstParent::NoChild => todo!(),
            AstParent::Impl => todo!(),
        };
        Ast::ModuleItemVariant {
            token_group_idx,
            module_item_variant_path: ModuleItemVariantPath::new(self.db, module_item_path, ident),
            ident: todo!(),
        }
    }
}

fn parse_head(
    mut aux_parser: husky_token::TokenIter,
) -> Result<(Accessibility, EntityKind, Identifier, bool), AstError> {
    let accessibility = aux_parser.parse_accessibility()?;
    let entity_kind = aux_parser.parse_entity_kind()?;
    let ident = aux_parser.parse_ident()?;
    let is_generic = aux_parser.parse_is_generic();
    Ok((accessibility, entity_kind, ident, is_generic))
}
