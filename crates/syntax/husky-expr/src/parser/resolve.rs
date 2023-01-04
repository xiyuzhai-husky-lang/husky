use super::*;
use husky_print_utils::p;
use husky_symbol::Symbol;
use husky_token::Punctuation;
use std::ops::ControlFlow;

pub type TokenResolveResult<T> = ControlFlow<(), T>;

impl<'a, 'b, 'c> ExprParseContext<'a, 'b> {
    pub(crate) fn resolve_token(
        &mut self,
        token_idx: TokenIdx,
        token: &Token,
    ) -> TokenResolveResult<ResolvedToken> {
        TokenResolveResult::Continue(match token.kind {
            TokenKind::Attr(_) => todo!(),
            TokenKind::Keyword(keyword) => {
                ResolvedToken::Atom(Expr::Err(ExprError::UnexpectedKeyword(token_idx)))
            }
            TokenKind::Identifier(ident) => self.resolve_ident(ident),
            TokenKind::Punctuation(punc) => match punc {
                Punctuation::Binary(binary) => ResolvedToken::BinaryOpr(token_idx, binary),
                Punctuation::Bra(bra) => ResolvedToken::Bra(token_idx, bra),
                Punctuation::Ket(ket) => match self.last_bra() {
                    Some(bra) => {
                        if bra != ket {
                            p!(bra, ket);
                            p!(self.unfinished_exprs());
                            todo!()
                        }
                        ResolvedToken::Ket(token_idx, ket)
                    }
                    None => return TokenResolveResult::Break(()),
                },
                Punctuation::Suffix(suffix) => ResolvedToken::SuffixOpr(token_idx, suffix),
                Punctuation::LAngle => match self.top_expr() {
                    TopExprRef::Unfinished(_) => todo!(),
                    TopExprRef::Finished(expr) => {
                        match expr.base_entity_path(self.db(), &self.parser.expr_arena) {
                            BaseEntityPath::None => todo!(),
                            BaseEntityPath::Some(_) => todo!(),
                            BaseEntityPath::Uncertain { inclination } => match inclination {
                                BaseEntityPathInclination::GlobalValue
                                | BaseEntityPathInclination::TypeOrVariant => {
                                    ResolvedToken::Bra(token_idx, Bracket::Angle)
                                }
                                BaseEntityPathInclination::FunctionOrLocalValue => {
                                    ResolvedToken::BinaryOpr(token_idx, todo!())
                                }
                            },
                        }
                    }
                    TopExprRef::None => todo!(),
                },
                Punctuation::RAngle => match (self.last_bra(), self.env()) {
                    (Some(Bracket::Angle), _) => ResolvedToken::Ket(token_idx, Bracket::Angle),
                    (None, ExprParseEnvironment::WithinBracket(Bracket::Angle)) => {
                        return TokenResolveResult::Break(())
                    }
                    _ => ResolvedToken::BinaryOpr(
                        token_idx,
                        BinaryComparisonPunctuation::Greater.into(),
                    ),
                },
                Punctuation::DeriveAssign => return TokenResolveResult::Break(()),
                Punctuation::Minus => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Minus),
                Punctuation::Exclamation => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Not),
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
                        true => ResolvedToken::BinaryOpr(
                            token_idx,
                            BinaryOpr::PureClosed(BinaryPureClosedOpr::BitOr),
                        ),
                        false => ResolvedToken::Bra(token_idx, Bracket::Vertical),
                    },
                },
                Punctuation::DoubleExclamation => todo!(),
                Punctuation::Semicolon => todo!(),
                // return TokenResolveResult::Break(()),
                Punctuation::XmlKet => todo!(),
                Punctuation::At => todo!(),
                Punctuation::Question => match self.finished_expr() {
                    Some(_) => ResolvedToken::SuffixOpr(token_idx, SuffixOpr::Unveil),
                    None => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Option),
                },
                Punctuation::PoundSign => todo!(),
                Punctuation::Ambersand => match self.finished_expr() {
                    Some(_) => ResolvedToken::BinaryOpr(
                        token_idx,
                        BinaryOpr::PureClosed(BinaryPureClosedOpr::BitOr),
                    ),
                    None => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Ref),
                },
                Punctuation::DotDot => todo!(),
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => ResolvedToken::Atom(Expr::Literal(token_idx)),
            TokenKind::Comment => unreachable!(),
            TokenKind::Err(ref error) => ResolvedToken::Atom(Expr::Err(error.clone().into())),
        })
    }
}

impl<'a, 'b, 'c> ExprParseContext<'a, 'b> {
    fn resolve_ident(&self, ident: Identifier) -> ResolvedToken {
        if let Some(opn) = self.last_unfinished_expr() {
            match opn {
                UnfinishedExpr::Binary {
                    punctuation: BinaryOpr::ScopeResolution,
                    ..
                } => {
                    if let Some(previous_expr) = self.finished_expr() {
                        match previous_expr.base_entity_path(self.db(), &self.parser.expr_arena) {
                            BaseEntityPath::None => todo!(),
                            BaseEntityPath::Some(_) => todo!(),
                            BaseEntityPath::Uncertain { .. } => {
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
        match self.parser.symbol_stack.resolve_ident(ident) {
            Some(symbol) => todo!(),
            // symbol.into(),
            None => ResolvedToken::Atom(Expr::Unrecognized(ident)),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ResolvedToken {
    Atom(Expr),
    BinaryOpr(TokenIdx, BinaryOpr),
    PrefixOpr(TokenIdx, PrefixOpr),
    SuffixOpr(TokenIdx, SuffixOpr),
    Bra(TokenIdx, Bracket),
    Ket(TokenIdx, Bracket),
    Dot(TokenIdx),
    ListItem(TokenIdx),
}
