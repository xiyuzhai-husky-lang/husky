mod accept;
mod opr;
mod resolve;
mod stack;

use crate::*;
use husky_symbol_syntax::SymbolContext;
use husky_token::{Token, TokenKind, TokenStream};
use opr::*;
use resolve::*;
use stack::*;

pub(crate) struct Automata<'a> {
    ctx: &'a mut SymbolContext<'a>,
    stream: TokenStream<'a>,
    stack: AutomataStack,
    arena: &'a mut RawExprArena,
}

impl<'a> Automata<'a> {
    pub(crate) fn stream(&self) -> &TokenStream<'a> {
        &self.stream
    }

    fn new(
        ctx: &'a mut SymbolContext<'a>,
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
}

pub fn parse_raw_exprs<'a>(
    ctx: &'a mut SymbolContext<'a>,
    arena: &'a mut RawExprArena,
    tokens: &'a [Token],
) {
    Automata::new(ctx, tokens, arena).parse_all()
}
