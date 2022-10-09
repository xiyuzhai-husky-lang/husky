mod accept;
mod context;
mod opr;
mod resolve;
mod state;

use crate::*;
use context::*;
use husky_token::{Token, TokenKind, TokenStream};
use opr::*;
use resolve::*;
use state::*;

pub(crate) struct Automata<'a> {
    ctx: AutomataContext<'a>,
    stream: TokenStream<'a>,
    stack: AutomataStack,
    arena: &'a mut RawExprArena,
}

impl<'a> Automata<'a> {
    pub(crate) fn stream(&self) -> &TokenStream<'a> {
        &self.stream
    }

    fn new(db: &'a dyn ExprSyntaxQuery, tokens: &'a [Token], arena: &'a mut RawExprArena) -> Self {
        Self {
            ctx: AutomataContext::new(db),
            stream: tokens.into(),
            stack: Default::default(),
            arena,
        }
    }

    fn parse_all(mut self) {
        while !self.stream().is_empty() {
            let token = &self.stream.next().unwrap();
            self.accept_token(self.resolve_token(token))
        }
    }

    pub fn parse_raw_exprs(
        db: &'a dyn ExprSyntaxQuery,
        arena: &'a mut RawExprArena,
        tokens: &'a [Token],
    ) {
        Self::new(db, tokens, arena).parse_all()
    }
}
