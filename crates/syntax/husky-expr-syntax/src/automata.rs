mod opr;
mod ops;
mod state;

use crate::*;
use husky_token::{Token, TokenKind, TokenStream};
use opr::*;
use state::*;

pub(crate) struct Automata<'a> {
    arena: &'a mut RawExprArena,
    oprs: Vec<OnStackOpr>,
    exprs: Vec<RawExpr>,
    state: AutomataState,
    stream: TokenStream<'a>,
}

impl<'a> Automata<'a> {
    pub(crate) fn stream(&self) -> &TokenStream<'a> {
        &self.stream
    }

    fn new(arena: &'a mut RawExprArena, tokens: &'a [Token]) -> Self {
        Self {
            arena,
            oprs: vec![],
            exprs: vec![],
            state: Default::default(),
            stream: tokens.into(),
        }
    }

    fn parse_all(mut self) {
        while !self.stream().is_empty() {
            self.parse_step()
        }
    }

    pub fn parse_raw_exprs(arena: &'a mut RawExprArena, tokens: &'a [Token]) {
        Self::new(arena, tokens).parse_all()
    }
}
