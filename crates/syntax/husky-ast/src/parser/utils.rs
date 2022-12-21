use crate::*;
use husky_entity_kind::{ModuleItemKind, TypeKind};
use husky_opn_syntax::Bracket;
use husky_token::*;
use std::iter::Peekable;

pub(super) trait AuxAstParser<'aux> {
    fn token_iter_mut(&mut self) -> &mut TokenIter<'aux>;

    fn parse_accessibility(&mut self) -> AstResult<Accessibility> {
        Ok(match self.token_iter_mut().peek().unwrap().kind {
            TokenKind::Decorator(decor) => match decor {
                Decorator::Pub => {
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
                Decorator::Protected => todo!(),
                Decorator::Private => todo!(),
                Decorator::Async => todo!(),
                Decorator::Static => Accessibility::Public,
            },
            _ => Accessibility::Protected,
        })
    }

    fn parse_entity_kind(&mut self) -> AstResult<EntityKind> {
        Ok(
            match self
                .token_iter_mut()
                .next()
                .ok_or(AstError::ExpectEntityKeyword)?
                .kind
            {
                TokenKind::Decorator(_decor) => self.parse_entity_kind()?,
                TokenKind::Keyword(kw) => match kw {
                    Keyword::Paradigm(_) | Keyword::Visual => {
                        EntityKind::ModuleItem(ModuleItemKind::Form)
                    }
                    Keyword::Type(ty_kw) => {
                        let ty_kind = match ty_kw {
                            TypeKeyword::Type => TypeKind::Form,
                            TypeKeyword::Struct => TypeKind::Struct,
                            TypeKeyword::Enum => TypeKind::Enum,
                            TypeKeyword::Record => TypeKind::Record,
                            TypeKeyword::Structure => TypeKind::Structure,
                            TypeKeyword::Inductive => TypeKind::Inductive,
                        };
                        EntityKind::ModuleItem(ModuleItemKind::Type(ty_kind))
                    }
                    Keyword::Trait => EntityKind::ModuleItem(ModuleItemKind::Trait),
                    Keyword::Mod => EntityKind::Module,
                    Keyword::Impl | Keyword::End(_) => return Err(AstError::ExpectEntityKeyword),
                    Keyword::Config(_)
                    | Keyword::Stmt(_)
                    | Keyword::Liason(_)
                    | Keyword::Main
                    | Keyword::Use => unreachable!(),
                },
                TokenKind::Comment => todo!(),
                _ => return Err(AstError::ExpectEntityKeyword),
            },
        )
    }

    fn parse_ident(&mut self) -> AstResult<Identifier> {
        let token = self
            .token_iter_mut()
            .next()
            .ok_or(AstError::ExpectIdentifier(None))?;
        token
            .identify()
            .ok_or(AstError::ExpectIdentifier(Some(token.range)))
    }

    fn parse_is_generic(&mut self) -> bool {
        let Some (token) = &self
            .token_iter_mut()
            .next() else { return false };
        match token.kind {
            TokenKind::Special(SpecialToken::LAngle) => true,
            _ => false,
        }
    }
}

impl<'a> AuxAstParser<'a> for TokenIter<'a> {
    fn token_iter_mut(&mut self) -> &mut TokenIter<'a> {
        self
    }
}
