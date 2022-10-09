mod opr;
mod state;

use crate::*;
use husky_token::{HuskyToken, HuskyTokenKind, TokenStream};
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
    pub(super) fn new(arena: &'a mut RawExprArena, tokens: &'a [HuskyToken]) -> Self {
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
            HuskyTokenKind::Decorator(_) => todo!(),
            HuskyTokenKind::Keyword(_) => todo!(),
            HuskyTokenKind::Identifier(_) => todo!(),
            HuskyTokenKind::Special(_) => todo!(),
            HuskyTokenKind::WordOpr(_) => todo!(),
            HuskyTokenKind::WordPattern(_) => todo!(),
            HuskyTokenKind::PrimitiveLiteral(_) => todo!(),
            HuskyTokenKind::Unrecognized(_) => todo!(),
            HuskyTokenKind::IllFormedLiteral(_) => todo!(),
        }
    }

    pub(super) fn parse_all(mut self) {
        while !self.stream.is_empty() {
            self.parse_next()
        }
    }
}
