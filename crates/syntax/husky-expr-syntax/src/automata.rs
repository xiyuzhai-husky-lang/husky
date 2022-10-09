mod opr;
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
    pub(super) fn new(arena: &'a mut RawExprArena, tokens: &'a [Token]) -> Self {
        Self {
            arena,
            oprs: vec![],
            exprs: vec![],
            state: Default::default(),
            stream: tokens.into(),
        }
    }

    fn parse_next(&mut self) {
        let token = self.stream.next().expect("non empty");
        match token.kind {
            TokenKind::Decorator(_) => todo!(),
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(_) => todo!(),
            TokenKind::Special(_) => todo!(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::WordPattern(_) => todo!(),
            TokenKind::PrimitiveLiteral(_) => todo!(),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
        }
    }

    pub(super) fn parse_all(mut self) {
        while !self.stream.is_empty() {
            self.parse_next()
        }
    }
}
