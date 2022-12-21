use husky_opn_syntax::{BinaryOpr, BinaryPureClosedOpr, Bracket};
use husky_token::TokenIter;

use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_uses(&mut self, token_group_idx: TokenGroupIdx, _indent: u32) -> Ast {
        let token_iter = self.token_sheet.token_group_token_iter(token_group_idx);
        let (ident, mut aux_parser) =
            EntityUseExprParser::new(token_iter, &mut self.use_expr_arena);
        let accessibility = match aux_parser.parse_accessibility() {
            Ok(accessibility) => accessibility,
            Err(error) => {
                return Ast::Err {
                    token_group_idx,
                    error,
                }
            }
        };
        let use_expr = match aux_parser.parse_step() {
            Some(expr) => expr,
            None => self.use_expr_arena.alloc_one(UseExpr::Err(todo!())),
        };
        Ast::Use {
            token_group_idx,
            accessibility,
            use_expr_idx: use_expr,
        }
    }
}

pub struct EntityUseExprParser<'b> {
    token_iter: TokenIter<'b>,
    arena: &'b mut UseExprArena,
}

impl<'aux> AuxAstParser<'aux> for EntityUseExprParser<'aux> {
    fn token_iter_mut(&mut self) -> &mut TokenIter<'aux> {
        &mut self.token_iter
    }
}

impl<'b> EntityUseExprParser<'b> {
    fn new(mut token_iter: TokenIter<'b>, arena: &'b mut UseExprArena) -> (Identifier, Self) {
        while let Some(token) = token_iter.next() {
            match token.kind {
                TokenKind::Keyword(Keyword::Use) => break,
                _ => continue,
            }
        }
        let ident = match token_iter.peek() {
            Some(token) => match token.kind {
                TokenKind::Decorator(_) => todo!(),
                TokenKind::Keyword(_) => todo!(),
                TokenKind::Identifier(ident) => ident,
                TokenKind::Special(_) => todo!(),
                TokenKind::WordOpr(_) => todo!(),
                TokenKind::Literal(_) => todo!(),
                TokenKind::Unrecognized(_) => todo!(),
                TokenKind::IllFormedLiteral(_) => todo!(),
                TokenKind::Comment => todo!(),
            },
            None => todo!(),
        };
        (ident, Self { token_iter, arena })
    }

    fn parse_step(&mut self) -> Option<UseExprIdx> {
        let expr = self.parse_step_inner()?;
        Some(self.arena.alloc_one(expr))
    }

    fn parse_step_inner(&mut self) -> Option<UseExpr> {
        let token = self.token_iter.next()?;
        Some(match token.kind {
            TokenKind::Identifier(ident) => {
                let Some(next_token) = self.token_iter.next() else  {
                    return Some(UseExpr::One { ident })
                };
                match next_token.kind {
                    TokenKind::Special(SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)) => {}
                    TokenKind::Special(SpecialToken::Comma)
                    | TokenKind::Special(SpecialToken::Ket(Bracket::Curl)) => {
                        self.token_iter.step_back();
                        return Some(UseExpr::One { ident });
                    }
                    _ => return Some(UseExpr::Err(EntityUseExprError::ExpectScopeResolution)),
                }
                let Some(child) =  self.parse_step() else {
                    return Some(UseExpr::Err(
                        EntityUseExprError::ExpectScopeResolution,
                    ))
                };
                UseExpr::ScopeResolution {
                    parent: ident,
                    child,
                }
            }
            TokenKind::Special(SpecialToken::Bra(Bracket::Curl)) => self.parse_multiple(),
            // ad hoc; todo: change this to SpecialToken::Star
            TokenKind::Special(SpecialToken::BinaryOpr(BinaryOpr::PureClosed(
                BinaryPureClosedOpr::Mul,
            ))) => UseExpr::All {},
            _ => UseExpr::Err(EntityUseExprError::ExpectSomething),
        })
    }

    fn parse_multiple(&mut self) -> UseExpr {
        let mut exprs: Vec<UseExpr> = vec![];
        loop {
            match self.parse_step_inner() {
                Some(expr) => exprs.push(expr),
                None => todo!(),
            }
            match self.token_iter.next() {
                Some(token) => match token.kind {
                    TokenKind::Special(SpecialToken::Comma) => continue,
                    TokenKind::Special(SpecialToken::Ket(Bracket::Curl)) => break,
                    _ => todo!(),
                },
                None => todo!(),
            }
        }
        UseExpr::Multiple {
            exprs: self.arena.alloc_batch(exprs),
        }
    }
}
