mod accept;
mod opr;
mod resolve;
mod stack;
mod synthesize;

use crate::*;
use husky_check_utils::should;
use husky_entity_tree::EntityTreeDb;
use husky_symbol_syntax::{SymbolContext, SymbolSheet};
use husky_token::TokenIter;
use husky_token::{Token, TokenKind};
use opr::*;
use resolve::*;
use stack::*;

pub(crate) struct Automata<'a, 'b> {
    db: &'a dyn EntityTreeDb,
    tokens: TokenIter<'a>,
    symbols: &'a mut SymbolContext<'b>,
    arena: &'a mut ExprArena,
    stack: AutomataStack,
}

impl<'a, 'b> Automata<'a, 'b> {
    pub(crate) fn tokens(&self) -> &TokenIter<'a> {
        &self.tokens
    }

    fn new(
        db: &'a dyn EntityTreeDb,
        tokens: &'a [Token],
        symbols: &'a mut SymbolContext<'b>,
        arena: &'a mut ExprArena,
    ) -> Self {
        Self {
            db,
            tokens: tokens.into(),
            symbols,
            arena,
            stack: Default::default(),
        }
    }

    fn parse_all(mut self) -> ExprIdx {
        while !self.tokens().is_empty() {
            let token = &self.tokens.next().unwrap();
            match self.accept_token(self.resolve_token(token)) {
                Ok(()) => (),
                Err(_) => todo!(),
            }
        }
        self.synthesize_all_above(Precedence::None).expect("todo");
        should!(self.stack.number_of_exprs() == 1);
        self.arena.alloc_one(self.stack.pop_expr().unwrap())
    }
}

pub fn parse_expr(
    db: &dyn EntityTreeDb,
    tokens: &[Token],
    symbols: &mut SymbolContext,
    arena: &mut ExprArena,
) -> ExprIdx {
    Automata::new(db, tokens, symbols, arena).parse_all()
}
