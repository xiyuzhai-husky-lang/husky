use super::*;
use husky_entity_kind::TyKind;
use husky_opn_syntax::Bracket;
use husky_token::{Decorator, Token, TypeKeyword};
use std::iter::Peekable;

impl<'a> AstParser<'a> {
    pub(super) fn parse_defn(&mut self, token_group: TokenGroupIdx, indent: u32) -> Ast {
        self.parse_defn_aux(indent, token_group)
            .unwrap_or_else(|e| Ast::Err(token_group, e))
    }

    fn parse_defn_aux(&mut self, indent: u32, token_group: TokenGroupIdx) -> AstResult<Ast> {
        let (body, body_kind) = {
            let body = self.parse_asts(indent + INDENT_INCR);
            match body {
                Some(_) => (body, DefnBodyKind::Block),
                None => match self.token_groups.peek_with_exact_indent(indent) {
                    Some((token_group_idx, token_group)) => match token_group.first().kind {
                        TokenKind::Special(SpecialToken::Vertical) => {
                            (self.parse_case_stmts(indent), DefnBodyKind::Cases)
                        }
                        _ => (None, DefnBodyKind::None),
                    },
                    None => (None, DefnBodyKind::None),
                },
            }
        };
        let mut parser = DefnHeadParser {
            token_iter: self.token_sheet[token_group].iter().peekable(),
        };
        Ok(Ast::Defn {
            // order matters!
            accessibility: parser.parse_accessibility()?,
            entity_kind: parser.parse_entity_kind()?,
            ident: parser.parse_ident()?,
            is_generic: parser.parse_is_generic(),
            token_group,
            body,
            body_kind,
        })
    }
}

struct DefnHeadParser<'a> {
    token_iter: Peekable<core::slice::Iter<'a, Token>>,
}

impl<'a> DefnHeadParser<'a> {
    fn parse_accessibility(&mut self) -> AstResult<Accessibility> {
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
                Decorator::Static => todo!(),
            },
            _ => Accessibility::PubCrate,
        })
    }

    fn parse_entity_kind(&mut self) -> AstResult<EntityKind> {
        Ok(
            match self
                .token_iter
                .next()
                .ok_or(AstError::ExpectEntityKeyword)?
                .kind
            {
                TokenKind::Decorator(_) => todo!(),
                TokenKind::Keyword(kw) => match kw {
                    Keyword::Config(_) => todo!(),
                    Keyword::Paradigm(_) => todo!(),
                    Keyword::Type(ty_kw) => EntityKind::Type(match ty_kw {
                        TypeKeyword::Type => TyKind::Any,
                        TypeKeyword::Struct => TyKind::Struct,
                        TypeKeyword::Enum => TyKind::Enum,
                        TypeKeyword::Record => TyKind::Record,
                        TypeKeyword::Structure => TyKind::Structure,
                        TypeKeyword::Inductive => TyKind::Inductive,
                    }),
                    Keyword::Stmt(_) => todo!(),
                    Keyword::Liason(_) => todo!(),
                    Keyword::Main => EntityKind::Main,
                    Keyword::Mod => EntityKind::Module,
                    Keyword::Visual => todo!(),
                    Keyword::Use => unreachable!(),
                    Keyword::Impl | Keyword::End(_) => return Err(AstError::ExpectEntityKeyword),
                },
                _ => return Err(AstError::ExpectEntityKeyword),
                TokenKind::Comment => todo!(),
            },
        )
    }

    fn parse_ident(&mut self) -> AstResult<Identifier> {
        let token = self
            .token_iter
            .next()
            .ok_or(AstError::ExpectIdentifier(None))?;
        token
            .identify()
            .ok_or(AstError::ExpectIdentifier(Some(token.range)))
    }

    fn parse_is_generic(&mut self) -> bool {
        let Some (token) = &self
            .token_iter
            .next() else { return false };
        match token.kind {
            TokenKind::Special(SpecialToken::LAngle) => true,
            _ => false,
        }
    }
}
