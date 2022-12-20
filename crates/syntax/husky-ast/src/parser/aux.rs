use crate::*;
use husky_opn_syntax::Bracket;
use husky_token::*;
use std::iter::Peekable;

pub(super) struct AuxAstParser<'a> {
    token_iter: Peekable<core::slice::Iter<'a, Token>>,
}

impl<'a> AstParser<'a> {
    pub(super) fn aux_parser(&self, token_group_idx: TokenGroupIdx) -> AuxAstParser {
        let token_iter = self.token_sheet[token_group_idx].iter().peekable();
        AuxAstParser { token_iter }
    }
}

impl<'a> AuxAstParser<'a> {
    pub(super) fn parse_accessibility(&mut self) -> AstResult<Accessibility> {
        Ok(match self.token_iter.peek().unwrap().kind {
            TokenKind::Decorator(decor) => match decor {
                Decorator::Pub => {
                    self.token_iter.next();
                    match self
                        .token_iter
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

    pub(super) fn parse_entity_card(&mut self) -> AstResult<EntityCard> {
        Ok(
            match self
                .token_iter
                .next()
                .ok_or(AstError::ExpectEntityKeyword)?
                .kind
            {
                TokenKind::Decorator(decor) => self.parse_entity_card()?,
                TokenKind::Keyword(kw) => match kw {
                    Keyword::Paradigm(_) | Keyword::Visual => EntityCard::Form,
                    Keyword::Type(ty_kw) => EntityCard::Type,
                    Keyword::Trait => EntityCard::Trait,
                    Keyword::Mod => EntityCard::Module,
                    Keyword::Impl | Keyword::End(_) => return Err(AstError::ExpectEntityKeyword),
                    Keyword::Config(_)
                    | Keyword::Stmt(_)
                    | Keyword::Liason(_)
                    | Keyword::Main
                    | Keyword::Use => unreachable!(),
                },
                _ => return Err(AstError::ExpectEntityKeyword),
                TokenKind::Comment => todo!(),
            },
        )
    }

    pub(super) fn parse_ident(&mut self) -> AstResult<Identifier> {
        let token = self
            .token_iter
            .next()
            .ok_or(AstError::ExpectIdentifier(None))?;
        token
            .identify()
            .ok_or(AstError::ExpectIdentifier(Some(token.range)))
    }

    pub(super) fn parse_is_generic(&mut self) -> bool {
        let Some (token) = &self
            .token_iter
            .next() else { return false };
        match token.kind {
            TokenKind::Special(SpecialToken::LAngle) => true,
            _ => false,
        }
    }
}
