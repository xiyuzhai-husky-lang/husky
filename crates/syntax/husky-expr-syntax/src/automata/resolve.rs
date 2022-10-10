use crate::*;

impl<'a> Automata<'a> {
    pub(crate) fn resolve_token(&self, token: &Token) -> ResolvedToken {
        match token.kind {
            TokenKind::Decorator(_) => todo!(),
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(ident) => self.resolve_ident(ident),
            TokenKind::Special(_) => todo!(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::WordPattern(_) => todo!(),
            TokenKind::PrimitiveLiteral(_) => todo!(),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
        }
    }

    fn resolve_ident(&self, ident: Identifier) -> ResolvedToken {
        if let Some(opr) = self.stack.top_opr() {
            match opr.variant {
                automata::opr::OnStackOprVariant::Binary(BinaryOpr::ScopeResolution) => todo!(),
                _ => (),
            }
        }
        todo!()
    }
}

pub(crate) enum ResolvedToken {}
