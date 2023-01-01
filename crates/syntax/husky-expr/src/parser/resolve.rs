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
                Punctuation::RAngle => todo!(),
                Punctuation::DeriveAssign => todo!(),
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
                            UnfinishedExpr::Binary { .. } => return TokenResolveResult::Break(()),
                            UnfinishedExpr::ListItem { .. } => todo!(),
                            UnfinishedExpr::Prefix { .. } => todo!(),
                            UnfinishedExpr::List { .. } => todo!(),
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
                    Some(_) => {
                        ResolvedToken::SuffixPunctuation(token_idx, SuffixPunctuation::Unveil)
                    }
                    None => todo!(),
                },
                Punctuation::PoundSign => todo!(),
                Punctuation::Ambersand => match self.top_expr() {
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
            TokenKind::Err(_) => todo!(),
        })
    }

    fn resolve_ident(&self, ident: Identifier) -> ResolvedToken {
        if let Some(opn) = self.last_unfinished_expr() {
            match opn {
                UnfinishedExpr::Binary {
                    binary: BinaryPunctuation::ScopeResolution,
                    ..
                } => {
                    if let Some(previous_expr) = self.top_expr() {
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
            Some(symbol) => symbol.into(),
            None => ResolvedToken::Atom(Expr::Unrecognized(ident)),
        }
    }
}

#[derive(Clone)]
pub(crate) enum ResolvedToken {
    Atom(Expr),
    BinaryPunctuation(TokenIdx, BinaryPunctuation),
    PrefixPunctuation(TokenIdx, PrefixPunctuation),
    SuffixPunctuation(TokenIdx, SuffixPunctuation),
    Bra(TokenIdx, Bracket),
    Ket(TokenIdx, Bracket),
    Dot(TokenIdx),
    Comma(TokenIdx),
}

impl From<Symbol> for ResolvedToken {
    fn from(symbol: Symbol) -> Self {
        let expr = match symbol {
            Symbol::Entity(_) => todo!(),
            Symbol::Variable(_) => todo!(),
            Symbol::Lifetime(_) => todo!(),
            Symbol::Label(_) => todo!(),
        };
        ResolvedToken::Atom(expr)
    }
}
