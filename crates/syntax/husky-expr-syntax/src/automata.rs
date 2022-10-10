mod accept;
mod opr;
mod resolve;
mod state;

use crate::*;
use husky_token::{Token, TokenKind, TokenStream};
use husky_variable_syntax::VariableContext;
use opr::*;
use resolve::*;
use state::*;

pub(crate) struct Automata<'a> {
    ctx: &'a mut VariableContext<'a>,
    stream: TokenStream<'a>,
    stack: AutomataStack,
    arena: &'a mut RawExprArena,
}

impl<'a> Automata<'a> {
    pub(crate) fn stream(&self) -> &TokenStream<'a> {
        &self.stream
    }

    fn new(
        ctx: &'a mut VariableContext<'a>,
        tokens: &'a [Token],
        arena: &'a mut RawExprArena,
    ) -> Self {
        Self {
            ctx,
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
        ctx: &'a mut VariableContext<'a>,
        arena: &'a mut RawExprArena,
        tokens: &'a [Token],
    ) {
        Self::new(ctx, tokens, arena).parse_all()
    }
}
