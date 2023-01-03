mod accept;
mod alloc;
mod debug;
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

pub struct ExprParser<'token, 'context> {
    ctx: SymbolContext<'context>,
    token_iter: TokenStream<'token>,
    stack: ExprStack,
    env: ExprEnvironmentPlace,
    expr_arena: ExprArena,
    entity_path_expr_arena: EntityPathExprArena,
    pattern_expr_arena: PatternExprArena,
}

impl<'a, 'b> ExprParser<'a, 'b> {
    pub fn new(ctx: SymbolContext<'b>, token_iter: TokenStream<'a>) -> Self {
        Self {
            ctx,
            token_iter,
            stack: Default::default(),
            env: Default::default(),
            expr_arena: Default::default(),
            entity_path_expr_arena: Default::default(),
            pattern_expr_arena: Default::default(),
        }
    }

    pub(crate) fn db(&self) -> &'b dyn EntityTreeDb {
        self.ctx.db()
    }

    pub(crate) fn tokens(&self) -> &TokenStream<'a> {
        &self.token_iter
    }

    pub fn parse_expr(&mut self, env: ExprEnvironment) -> (ExprSheet, Option<ExprIdx>) {
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
        let expr = self.finish_batch();
        (todo!(), expr)
    }
}

pub fn parse_expr<'a>(
    ctx: SymbolContext,
    token_iter: TokenStream<'a>,
    env: ExprEnvironment,
) -> (ExprSheet, Option<ExprIdx>) {
    ExprParser::new(ctx, token_iter).parse_expr(env)
}

impl<'a, 'b> parsec::HasParseError for ExprParser<'a, 'b> {
    type Error = ExprError;
}

impl<'a, 'b> std::ops::Deref for ExprParser<'a, 'b> {
    type Target = TokenStream<'a>;
    fn deref(&self) -> &Self::Target {
        &self.token_iter
    }
}

impl<'a, 'b> std::ops::DerefMut for ExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_iter
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for ExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_iter
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for ExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_iter
    }
}

impl<'a, 'b, 'c> parsec::StreamWrapper for ExprParser<'a, 'b> {}
