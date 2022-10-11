use husky_symbol_syntax::Symbol;
use husky_token::{Convexity, SpecialToken};

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
            TokenKind::Special(special) => match special {
                SpecialToken::LAngle => todo!(),
                SpecialToken::Leq => todo!(),
                SpecialToken::RAngle => todo!(),
                SpecialToken::Geq => todo!(),
                SpecialToken::Neq => todo!(),
                SpecialToken::DeriveAssign => todo!(),
                SpecialToken::Eq => todo!(),
                SpecialToken::Shl => todo!(),
                SpecialToken::LCurl => todo!(),
                SpecialToken::RCurl => todo!(),
                SpecialToken::LBox => todo!(),
                SpecialToken::RBox => todo!(),
                SpecialToken::LPar => todo!(),
                SpecialToken::RPar => todo!(),
                SpecialToken::Add => {
                    ResolvedTokenKind::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Add))
                }
                SpecialToken::Sub => {
                    ResolvedTokenKind::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Sub))
                }
                SpecialToken::Minus => ResolvedTokenKind::Prefix(PrefixOpr::Minus),
                SpecialToken::Star => todo!(),
                SpecialToken::Div => todo!(),
                SpecialToken::Power => todo!(),
                SpecialToken::And => todo!(),
                SpecialToken::DoubleVertical => todo!(),
                SpecialToken::BitNot => todo!(),
                SpecialToken::Modulo => todo!(),
                SpecialToken::FieldAccess => todo!(),
                SpecialToken::LightArrow => todo!(),
                SpecialToken::HeavyArrow => todo!(),
                SpecialToken::DoubleColon => todo!(),
                SpecialToken::Colon => todo!(),
                SpecialToken::Comma => todo!(),
                SpecialToken::Ambersand => todo!(),
                SpecialToken::Incr => todo!(),
                SpecialToken::Decr => todo!(),
                SpecialToken::Vertical => todo!(),
                SpecialToken::Assign => todo!(),
                SpecialToken::AddAssign => todo!(),
                SpecialToken::SubAssign => todo!(),
                SpecialToken::MulAssign => todo!(),
                SpecialToken::DivAssign => todo!(),
                SpecialToken::BitAndAssign => todo!(),
                SpecialToken::BitOrAssign => todo!(),
                SpecialToken::Exclamation => todo!(),
                SpecialToken::DoubleExclamation => todo!(),
                SpecialToken::Semicolon => todo!(),
                SpecialToken::XmlKet => todo!(),
                SpecialToken::At => todo!(),
                SpecialToken::QuestionMark => todo!(),
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::WordPattern(_) => todo!(),
            TokenKind::Literal(literal) => ResolvedTokenKind::Atom(literal.into()),
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

#[derive(Clone)]
pub(crate) struct ResolvedToken {
    kind: ResolvedTokenKind,
    range: TextRange,
}

impl TextRanged for ResolvedToken {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl ResolvedToken {
    pub(super) fn kind(&self) -> &ResolvedTokenKind {
        &self.kind
    }

    pub(super) fn to_expr(self) -> RawExpr {
        let variant = match self.kind {
            ResolvedTokenKind::Atom(variant) => variant.into(),
            ResolvedTokenKind::BinaryOpr(_) => todo!(),
            ResolvedTokenKind::Prefix(_) => todo!(),
        };
        RawExpr::new(variant, self.range)
    }
}

#[derive(Clone)]
pub(crate) enum ResolvedTokenKind {
    Atom(RawAtom),
    BinaryOpr(BinaryOpr),
    Prefix(PrefixOpr),
}

impl From<Symbol> for ResolvedTokenKind {
    fn from(symbol: Symbol) -> Self {
        ResolvedTokenKind::Atom(symbol.into())
    }
}
