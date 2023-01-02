use super::*;
use husky_symbol::Symbol;
use husky_token::Punctuation;
use std::ops::ControlFlow;

pub type TokenResolveResult<T> = ControlFlow<(), T>;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn resolve_token(
        &mut self,
        token_idx: TokenIdx,
        token: &Token,
    ) -> TokenResolveResult<ResolvedToken> {
        TokenResolveResult::Continue(match token.kind {
            TokenKind::Attr(_) => todo!(),
            TokenKind::Keyword(_keyword) => todo!(),
            TokenKind::Identifier(ident) => self.resolve_ident(ident),
            TokenKind::Punctuation(punc) => match punc {
                Punctuation::Binary(binary) => ResolvedToken::BinaryPunctuation(token_idx, binary),
                Punctuation::Bra(bra) => ResolvedToken::Bra(token_idx, bra),
                Punctuation::Ket(ket) => ResolvedToken::Ket(token_idx, ket),
                Punctuation::Suffix(suffix) => ResolvedToken::SuffixPunctuation(token_idx, suffix),
                Punctuation::LAngle => todo!(),
                Punctuation::RAngle => match (self.last_bra(), self.env()) {
                    (Some(Bracket::Angle), _) => ResolvedToken::Ket(token_idx, Bracket::Angle),
                    (None, ExprEnvironment::WithinBracket(Bracket::Angle)) => {
                        return TokenResolveResult::Break(())
                    }
                    _ => ResolvedToken::BinaryPunctuation(
                        token_idx,
                        BinaryComparisonPunctuation::Greater.into(),
                    ),
                },
                Punctuation::DeriveAssign => return TokenResolveResult::Break(()),
                Punctuation::Minus => {
                    ResolvedToken::PrefixPunctuation(token_idx, PrefixPunctuation::Minus)
                }
                Punctuation::Exclamation => {
                    ResolvedToken::PrefixPunctuation(token_idx, PrefixPunctuation::Not)
                }
                Punctuation::DoubleVertical => todo!(),
                Punctuation::BitNot => todo!(),
                Punctuation::Dot => ResolvedToken::Dot(token_idx),
                Punctuation::Colon => todo!(),
                Punctuation::Comma => {
                    self.reduce(Precedence::ListItem);
                    match self.last_unfinished_expr() {
                        Some(expr) => match expr {
                            UnfinishedExpr::List { .. } => ResolvedToken::ListItem(token_idx),
                            _ => return TokenResolveResult::Break(()),
                        },
                        None => return TokenResolveResult::Break(()),
                    }
                }
                Punctuation::Vertical => match self.last_unfinished_expr() {
                    Some(UnfinishedExpr::List {
                        bra: Bracket::Vertical,
                        ..
                    }) => ResolvedToken::Ket(token_idx, Bracket::Vertical),
                    _ => match self.finished_expr().is_some() {
                        true => ResolvedToken::BinaryPunctuation(
                            token_idx,
                            BinaryPunctuation::PureClosed(BinaryPureClosedPunctuation::BitOr),
                        ),
                        false => ResolvedToken::Bra(token_idx, Bracket::Vertical),
                    },
                },
                Punctuation::DoubleExclamation => todo!(),
                Punctuation::Semicolon => todo!(),
                // return TokenResolveResult::Break(()),
                Punctuation::XmlKet => todo!(),
                Punctuation::At => todo!(),
                Punctuation::QuestionMark => match self.finished_expr() {
                    Some(_) => {
                        ResolvedToken::SuffixPunctuation(token_idx, SuffixPunctuation::Unveil)
                    }
                    None => todo!(),
                },
                Punctuation::PoundSign => todo!(),
                Punctuation::Ambersand => match self.finished_expr() {
                    Some(_) => ResolvedToken::BinaryPunctuation(
                        token_idx,
                        BinaryPunctuation::PureClosed(BinaryPureClosedPunctuation::BitOr),
                    ),
                    None => ResolvedToken::PrefixPunctuation(token_idx, PrefixPunctuation::Ref),
                },
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => ResolvedToken::Atom(Expr::Literal(token_idx)),
            TokenKind::Comment => unreachable!(),
            TokenKind::Err(ref error) => ResolvedToken::Atom(Expr::Err(error.clone().into())),
        })
    }

    fn resolve_ident(&self, ident: Identifier) -> ResolvedToken {
        if let Some(opn) = self.last_unfinished_expr() {
            match opn {
                UnfinishedExpr::Binary {
                    punctuation: BinaryPunctuation::ScopeResolution,
                    ..
                } => {
                    if let Some(previous_expr) = self.finished_expr() {
                        match previous_expr.base_entity_path() {
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
            Some(symbol) => todo!(),
            // symbol.into(),
            None => ResolvedToken::Atom(Expr::Unrecognized(ident)),
        }
    }
}

pub(crate) enum ResolvedToken {
    Atom(Expr),
    BinaryPunctuation(TokenIdx, BinaryPunctuation),
    PrefixPunctuation(TokenIdx, PrefixPunctuation),
    SuffixPunctuation(TokenIdx, SuffixPunctuation),
    Bra(TokenIdx, Bracket),
    Ket(TokenIdx, Bracket),
    Dot(TokenIdx),
    ListItem(TokenIdx),
}
