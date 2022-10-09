use crate::*;

impl<'a> Automata<'a> {
    pub(crate) fn resolve_token(&self, token: &Token) -> ResolvedToken {
        match token.kind {
            TokenKind::Decorator(_) => todo!(),
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(_) => todo!(),
            TokenKind::Special(_) => todo!(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::WordPattern(_) => todo!(),
            TokenKind::PrimitiveLiteral(_) => todo!(),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
        }
    }
}

pub(crate) enum ResolvedToken {}
