use super::*;
use husky_entity_taxonomy::{ModuleItemKind, TypeKind};
use husky_opn_syntax::Bracket;
use husky_token::*;
use std::iter::Peekable;

pub(super) trait AstTokenParseContext<'a>: TokenParseContext<'a> {
    fn ast_context_kind(&self) -> AstContextKind;
    fn module_path(&self) -> ModulePath;

    fn parse_accessibility(&mut self) -> AstResult<Accessibility> {
        Ok(match self.token_iter_mut().peek().unwrap().kind {
            TokenKind::Attr(decor) => match decor {
                AttrKeyword::Pub => {
                    self.token_iter_mut().next();
                    match self
                        .token_iter_mut()
                        .peek()
                        .ok_or(AstError::ExpectParBraOrDecoratorOrIdentifier(None))?
                        .kind
                    {
                        TokenKind::Special(SpecialToken::Bra(Bracket::Par)) => todo!(),
                        _ => Accessibility::Public,
                    }
                }
                AttrKeyword::Protected => todo!(),
                AttrKeyword::Private => todo!(),
                AttrKeyword::Async => todo!(),
                AttrKeyword::Static => Accessibility::Public,
            },
            _ => Accessibility::PublicUnder(self.module_path()),
        })
    }

    fn take_entity_kind_keyword(&mut self) -> AstResult<Keyword> {
        let (idx, token) = self
            .token_iter_mut()
            .next_indexed()
            .ok_or(AstError::ExpectEntityKeyword)?;
        Ok(match token.kind {
            TokenKind::Attr(_) => self.take_entity_kind_keyword()?,
            TokenKind::Keyword(kw) => kw,
            // match kw {
            //     Keyword::Paradigm(_) | Keyword::Visual => match self.ast_parent() {
            //         AstContextInside::Trait { .. } | AstContextInside::Impl => {
            //             EntityKind::AssociatedItem
            //         }
            //         AstContextInside::Form | AstContextInside::Module => {
            //             EntityKind::ModuleItem(ModuleItemKind::Form)
            //         }
            //         AstContextInside::MatchStmt => todo!(),
            //         AstContextInside::NoChild => todo!(),
            //         AstContextInside::EnumLikeType { module_item_path } => todo!(),
            //         AstContextInside::OtherType { module_item_path } => todo!(),
            //     },
            //     Keyword::Type(ty_kw) => {
            //         let ty_kind = match ty_kw {
            //             TypeKeyword::Type => TypeKind::Form,
            //             TypeKeyword::Struct => TypeKind::Struct,
            //             TypeKeyword::Enum => TypeKind::Enum,
            //             TypeKeyword::Record => TypeKind::Record,
            //             TypeKeyword::Structure => TypeKind::Structure,
            //             TypeKeyword::Inductive => TypeKind::Inductive,
            //         };
            //         EntityKind::ModuleItem(ModuleItemKind::Type(ty_kind))
            //     }
            //     Keyword::Trait => EntityKind::ModuleItem(ModuleItemKind::Trait),
            //     Keyword::Mod => EntityKind::Module,
            //     Keyword::Impl | Keyword::End(_) => return Err(AstError::ExpectEntityKeyword),
            //     Keyword::Config(_)
            //     | Keyword::Stmt(_)
            //     | Keyword::Liason(_)
            //     | Keyword::Main
            //     | Keyword::Use => unreachable!(),
            // },
            TokenKind::Comment => todo!(),
            _ => return Err(AstError::ExpectEntityKeyword),
        })
    }

    fn parse_is_generic(&self) -> bool {
        let Some (token) = &self
            .token_iter()
            .peek() else { return false };
        match token.kind {
            TokenKind::Special(SpecialToken::LAngle) => true,
            _ => false,
        }
    }
}

impl<'a> std::ops::Deref for BasicAuxAstParser<'a> {
    type Target = TokenIter<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_iter
    }
}

impl<'a> std::ops::DerefMut for BasicAuxAstParser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_iter
    }
}

impl<'a> AstTokenParseContext<'a> for BasicAuxAstParser<'a> {
    fn ast_context_kind(&self) -> AstContextKind {
        self.ast_parent
    }

    fn module_path(&self) -> ModulePath {
        self.module_path
    }
}

pub(crate) struct BasicAuxAstParser<'a> {
    db: &'a dyn AstDb,
    ast_parent: AstContextKind,
    module_path: ModulePath,
    token_iter: TokenIter<'a>,
}

impl<'a> BasicAuxAstParser<'a> {
    pub(super) fn new(
        db: &'a dyn AstDb,
        ctx: &Context,
        module_path: ModulePath,
        token_iter: TokenIter<'a>,
    ) -> Self {
        Self {
            db,
            ast_parent: ctx.kind(),
            module_path,
            token_iter,
        }
    }

    pub(super) fn finish_with_saved_stream_state(self) -> TokenIdx {
        self.token_iter.save_state()
    }

    pub(crate) fn db(&self) -> &dyn AstDb {
        self.db
    }

    pub(crate) fn text_start(&self) -> TextPosition {
        self.token_iter.text_start()
    }
}
