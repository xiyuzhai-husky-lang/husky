use super::*;
use husky_symbol::Symbol;
use husky_term::Term;
use husky_token::Punctuation;
use std::ops::ControlFlow;

pub type TokenResolveResult<T> = ControlFlow<ExprParsingStopReason, T>;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn resolve_token(
        &self,
        token_idx: TokenIdx,
        token: &Token,
    ) -> TokenResolveResult<ResolvedToken> {
        TokenResolveResult::Continue(ResolvedToken {
            token_idx,
            kind: self.resolve_token_kind(token)?,
        })
    }

    fn resolve_token_kind(&self, token: &Token) -> TokenResolveResult<ResolvedTokenKind> {
        TokenResolveResult::Continue(match token.kind {
            TokenKind::Attr(_) => todo!(),
            TokenKind::Keyword(_keyword) => todo!(),
            TokenKind::Identifier(ident) => self.resolve_ident(ident),
            TokenKind::Punctuation(special) => match special {
                Punctuation::BinaryOpr(opr) => ResolvedTokenKind::BinaryOpr(opr),
                Punctuation::Bra(bra) => ResolvedTokenKind::Bra(bra),
                Punctuation::Ket(ket) => ResolvedTokenKind::Ket(ket),
                Punctuation::LAngle => todo!(),
                Punctuation::RAngle => todo!(),
                Punctuation::DeriveAssign => todo!(),
                Punctuation::Minus => ResolvedTokenKind::Prefix(PrefixOpr::Minus),
                Punctuation::Exclamation => ResolvedTokenKind::Prefix(PrefixOpr::Not),
                Punctuation::Incr => ResolvedTokenKind::Suffix(RawSuffixOpr::Incr),
                Punctuation::Decr => ResolvedTokenKind::Suffix(RawSuffixOpr::Decr),
                Punctuation::DoubleVertical => todo!(),
                Punctuation::BitNot => todo!(),
                Punctuation::Dot => ResolvedTokenKind::Dot,
                Punctuation::BinaryOpr(BinaryOpr::Curry) => todo!(),
                Punctuation::BinaryOpr(BinaryOpr::ScopeResolution) => todo!(),
                Punctuation::Colon => todo!(),
                Punctuation::Comma => {
                    match self.top_opn() {
                        Some(_) => todo!(),
                        None => return TokenResolveResult::Break(ExprParsingStopReason::Comma),
                    }
                    // ResolvedTokenKind::Comma
                }
                Punctuation::Ambersand => todo!(),
                Punctuation::Vertical => todo!(),
                Punctuation::DoubleExclamation => todo!(),
                Punctuation::Semicolon => {
                    return TokenResolveResult::Break(ExprParsingStopReason::Semicolon)
                }
                Punctuation::XmlKet => todo!(),
                Punctuation::At => todo!(),
                Punctuation::QuestionMark => todo!(),
                Punctuation::PoundSign => todo!(),
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(ref literal) => ResolvedTokenKind::Atom(literal.clone().into()),
            TokenKind::Comment => todo!(),
            TokenKind::Err(_) => todo!(),
        })
    }

    fn resolve_ident(&self, ident: Identifier) -> ResolvedTokenKind {
        if let Some(opn) = self.top_opn() {
            match opn {
                PartialOpn::Binary {
                    binary: BinaryOpr::ScopeResolution,
                    ..
                } => {
                    if let Some(previous_expr) = self.top_expr() {
                        match self.top_base_entity_path().unwrap() {
                            BaseEntityPath::None => todo!(),
                            BaseEntityPath::Some(_) => todo!(),
                            BaseEntityPath::Uncertain => {
                                todo!()
                                // return ResolvedTokenKind::Atom(AtomExpr::Uncertain(ident))
                            }
                        }
                    } else {
                        todo!()
                    }
                }
                _ => (),
            }
        }
        match self.ctx.resolve_ident(ident) {
            Some(symbol) => symbol.into(),
            None => ResolvedTokenKind::Atom(AtomExpr::Unrecognized(ident)),
        }
    }

    fn resolve_previous_entity(&self) -> Option<Term> {
        self.top_expr().map(|expr| self.resolve_entity(expr))
    }

    fn resolve_entity(&self, expr: &Expr) -> Term {
        todo!()
        // match expr.variant {
        //     Expr::Atom(ref atom) => match atom {
        //         AtomExpr::Literal(_) => todo!(),
        //         AtomExpr::Symbol(_) => todo!(),
        //         AtomExpr::Uncertain(_) => todo!(),
        //         AtomExpr::Unrecognized(_) => todo!(),
        //     },
        //     Expr::Opn {
        //         ref opn_variant,
        //         ref opds,
        //     } => match opn_variant {
        //         RawOpnVariant::Binary(_) => todo!(),
        //         RawOpnVariant::Prefix(_) => todo!(),
        //         RawOpnVariant::Suffix(_) => todo!(),
        //         RawOpnVariant::List(_) => todo!(),
        //         RawOpnVariant::Field(_) => todo!(),
        //         RawOpnVariant::CurlBracketed => self.resolve_entity(&self.arena[opds.start()]),
        //         RawOpnVariant::Abstraction => todo!(),
        //     },
        // }
    }
}

#[derive(Clone)]
pub(crate) struct ResolvedToken {
    kind: ResolvedTokenKind,
    token_idx: TokenIdx,
}

impl ResolvedToken {
    pub(super) fn kind(&self) -> &ResolvedTokenKind {
        &self.kind
    }

    pub(super) fn to_expr(self, arena: &ExprArena) -> Expr {
        match self.kind {
            ResolvedTokenKind::Atom(variant) => variant.into(),
            ResolvedTokenKind::BinaryOpr(_) => todo!(),
            ResolvedTokenKind::Prefix(_) => todo!(),
            ResolvedTokenKind::Suffix(_) => todo!(),
            ResolvedTokenKind::Bra(_) => todo!(),
            ResolvedTokenKind::Ket(_) => todo!(),
            ResolvedTokenKind::Dot => todo!(),
            ResolvedTokenKind::Comma => todo!(),
        }
    }

    pub(crate) fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Clone)]
pub(crate) enum ResolvedTokenKind {
    Atom(AtomExpr),
    BinaryOpr(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(RawSuffixOpr),
    Bra(Bracket),
    Ket(Bracket),
    Dot,
    Comma,
}

impl From<Symbol> for ResolvedTokenKind {
    fn from(symbol: Symbol) -> Self {
        ResolvedTokenKind::Atom(symbol.into())
    }
}
