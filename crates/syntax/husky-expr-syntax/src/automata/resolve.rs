use husky_symbol_syntax::Symbol;

use crate::*;

impl<'a> Automata<'a> {
    pub(crate) fn resolve_token(&self, token: &Token) -> ResolvedToken {
        ResolvedToken {
            kind: self.resolve_token_kind(token),
            range: token.range,
        }
    }

    fn resolve_token_kind(&self, token: &Token) -> ResolvedTokenKind {
        match token.kind {
            TokenKind::Decorator(_) => todo!(),
            TokenKind::Keyword(keyword) => todo!(),
            TokenKind::Identifier(ident) => self.resolve_ident(ident),
            TokenKind::Special(_) => todo!(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::WordPattern(_) => todo!(),
            TokenKind::PrimitiveLiteral(_) => todo!(),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
        }
    }

    fn resolve_ident(&self, ident: Identifier) -> ResolvedTokenKind {
        if let Some(opr) = self.stack.top_opr() {
            match opr.variant {
                automata::opr::OnStackOprVariant::Binary(BinaryOpr::ScopeResolution) => todo!(),
                _ => (),
            }
        }
        self.ctx.resolve_ident(ident).into()
    }
}

pub(crate) struct ResolvedToken {
    kind: ResolvedTokenKind,
    range: TextRange,
}

impl ResolvedToken {
    pub(super) fn kind(&self) -> &ResolvedTokenKind {
        &self.kind
    }

    pub(super) fn to_expr(self) -> RawExpr {
        let variant = match self.kind {
            ResolvedTokenKind::Atom(variant) => variant.into(),
        };
        RawExpr::new(variant, self.range)
    }
}

pub(crate) enum ResolvedTokenKind {
    Atom(RawAtom),
}

impl From<Symbol> for ResolvedTokenKind {
    fn from(symbol: Symbol) -> Self {
        ResolvedTokenKind::Atom(symbol.into())
    }
}
