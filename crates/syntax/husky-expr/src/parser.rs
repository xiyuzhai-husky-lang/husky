mod accept;
mod env;
mod list;
mod resolve;
mod stack;
mod unfinished_expr;

pub use env::*;
use husky_entity_tree::EntityTreeDb;

use crate::*;
use husky_symbol::SymbolContext;
use husky_token::TokenStream;
use husky_token::{Token, TokenKind};
use list::*;
use resolve::*;
use stack::*;
use std::ops::ControlFlow;
use unfinished_expr::*;

pub struct ExprParser<'sheet, 'token, 'context> {
    ctx: SymbolContext<'context>,
    token_iter: TokenStream<'token>,
    sheet: &'sheet mut ExprSheet,
    stack: ExprParserStack,
    env: ExprEnvironmentPlace,
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub fn new(
        ctx: SymbolContext<'c>,
        token_iter: TokenStream<'b>,
        sheet: &'a mut ExprSheet,
    ) -> Self {
        Self {
            ctx,
            token_iter,
            sheet,
            stack: Default::default(),
            env: Default::default(),
        }
    }

    pub(crate) fn db(&self) -> &'c dyn EntityTreeDb {
        self.ctx.db()
    }

    pub(crate) fn tokens(&self) -> &TokenStream<'b> {
        &self.token_iter
    }

    pub fn parse_expr(&mut self, env: ExprEnvironment) -> Option<ExprIdx> {
        self.env.set(env);
        while !self.tokens().is_empty() {
            let (token_idx, token) = self.token_iter.next_indexed(IgnoreComment::True).unwrap();
            match self.resolve_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        self.env.unset();
        self.finish_batch()
    }

    pub(crate) fn alloc_pattern_expr(&mut self, expr: PatternExpr) -> PatternExprIdx {
        self.sheet.alloc_pattern_expr(expr)
    }
}

pub fn parse_expr<'a>(
    ctx: SymbolContext,
    token_iter: TokenStream<'a>,
    sheet: &mut ExprSheet,
    env: ExprEnvironment,
) -> Option<ExprIdx> {
    ExprParser::new(ctx, token_iter, sheet).parse_expr(env)
}

impl<'a, 'b, 'c> parsec::HasParseError for ExprParser<'a, 'b, 'c> {
    type Error = ExprError;
}

impl<'a, 'b, 'c> std::ops::Deref for ExprParser<'a, 'b, 'c> {
    type Target = TokenStream<'b>;
    fn deref(&self) -> &Self::Target {
        &self.token_iter
    }
}

impl<'a, 'b, 'c> std::ops::DerefMut for ExprParser<'a, 'b, 'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_iter
    }
}

impl<'a, 'b, 'c> std::borrow::Borrow<TokenStream<'b>> for ExprParser<'a, 'b, 'c> {
    fn borrow(&self) -> &TokenStream<'b> {
        &self.token_iter
    }
}

impl<'a, 'b, 'c> std::borrow::BorrowMut<TokenStream<'b>> for ExprParser<'a, 'b, 'c> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'b> {
        &mut self.token_iter
    }
}

impl<'a, 'b, 'c> parsec::StreamWrapper for ExprParser<'a, 'b, 'c> {}
