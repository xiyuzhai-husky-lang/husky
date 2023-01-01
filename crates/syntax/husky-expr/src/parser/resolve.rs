use super::*;
use husky_symbol::Symbol;
use husky_term::Term;
use husky_token::Punctuation;
use std::ops::ControlFlow;

pub type TokenResolveResult<T> = ControlFlow<(), T>;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn resolve_token(
        &mut self,
        token_idx: TokenIdx,
        token: &Token,
    ) -> TokenResolveResult<ResolvedToken> {
        TokenResolveResult::Continue(ResolvedToken {
            token_idx,
            kind: self.resolve_token_kind(token_idx, token)?,
        })
    }

    fn resolve_token_kind(
        &mut self,
        token_idx: TokenIdx,
        token: &Token,
    ) -> TokenResolveResult<ResolvedTokenKind> {
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
                Punctuation::Minus => ResolvedTokenKind::Prefix(PrefixPunctuation::Minus),
                Punctuation::Exclamation => ResolvedTokenKind::Prefix(PrefixPunctuation::Not),
                Punctuation::Incr => ResolvedTokenKind::Suffix(SuffixPunctuation::Incr),
                Punctuation::Decr => ResolvedTokenKind::Suffix(SuffixPunctuation::Decr),
                Punctuation::DoubleVertical => todo!(),
                Punctuation::BitNot => todo!(),
                Punctuation::Dot => ResolvedTokenKind::Dot,
                Punctuation::BinaryOpr(BinaryPunctuation::Curry) => todo!(),
                Punctuation::BinaryOpr(BinaryPunctuation::ScopeResolution) => todo!(),
                Punctuation::Colon => todo!(),
                Punctuation::Comma => {
                    self.reduce(Precedence::ListItem);
                    match self.last_unfinished_expr() {
                        Some(expr) => match expr {
                            UnfinishedExpr::Binary { .. } => return TokenResolveResult::Break(()),
                            UnfinishedExpr::ListItem {
                                separator_token_idx,
                            } => todo!(),
                            UnfinishedExpr::Prefix {
                                prefix,
                                prefix_token_idx,
                            } => todo!(),
                            UnfinishedExpr::List {
                                opr,
                                bra,
                                bra_token_idx,
                                items,
                            } => todo!(),
                            UnfinishedExpr::LambdaHead { inputs, start } => todo!(),
                            UnfinishedExpr::Dot { dot_token_idx } => todo!(),
                            UnfinishedExpr::Application { function } => todo!(),
                        },
                        None => return TokenResolveResult::Break(()),
                    }
                }
                Punctuation::Vertical => todo!(),
                Punctuation::DoubleExclamation => todo!(),
                Punctuation::Semicolon => todo!(),
                // return TokenResolveResult::Break(()),
                Punctuation::XmlKet => todo!(),
                Punctuation::At => todo!(),
                Punctuation::QuestionMark => match self.top_expr() {
                    Some(_) => ResolvedTokenKind::Suffix(SuffixPunctuation::Unveil),
                    None => todo!(),
                },
                Punctuation::PoundSign => todo!(),
                Punctuation::Ambersand => match self.top_expr() {
                    Some(_) => ResolvedTokenKind::BinaryOpr(BinaryPunctuation::PureClosed(
                        BinaryPureClosedPunctuation::BitOr,
                    )),
                    None => ResolvedTokenKind::Prefix(PrefixPunctuation::Ref),
                },
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => ResolvedTokenKind::Atom(AtomExpr::Literal(token_idx)),
            TokenKind::Comment => todo!(),
            TokenKind::Err(_) => todo!(),
        })
    }

    fn resolve_ident(&self, ident: Identifier) -> ResolvedTokenKind {
        if let Some(opn) = self.last_unfinished_expr() {
            match opn {
                UnfinishedExpr::Binary {
                    binary: BinaryPunctuation::ScopeResolution,
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
    BinaryOpr(BinaryPunctuation),
    Prefix(PrefixPunctuation),
    Suffix(SuffixPunctuation),
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
