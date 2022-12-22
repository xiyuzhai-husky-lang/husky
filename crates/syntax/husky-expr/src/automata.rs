mod accept;
mod opr;
mod resolve;
mod stack;
mod synthesize;

use crate::*;
use husky_check_utils::should;
use husky_entity_symbol::EntitySymbolDb;
use husky_symbol_syntax::SymbolContext;
use husky_token::TokenIter;
use husky_token::{Token, TokenKind};
use opr::*;
use resolve::*;
use stack::*;

pub(crate) struct Automata<'a, 'b> {
    db: &'a dyn EntitySymbolDb,
    token_iter: TokenIter<'a>,
    symbols: &'a mut SymbolContext<'b>,
    arena: &'a mut ExprArena,
    stack: AutomataStack,
}

impl<'a, 'b> Automata<'a, 'b> {
    pub(crate) fn tokens(&self) -> &TokenIter<'a> {
        &self.token_iter
    }

    fn new(
        db: &'a dyn EntitySymbolDb,
        token_iter: TokenIter<'a>,
        symbols: &'a mut SymbolContext<'b>,
        arena: &'a mut ExprArena,
    ) -> Self {
        Self {
            db,
            token_iter,
            symbols,
            arena,
            stack: Default::default(),
        }
    }

    fn parse_all(mut self) -> ExprIdx {
        while !self.tokens().is_empty() {
            let token = &self.token_iter.next().unwrap();
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
    db: &dyn EntitySymbolDb,
    token_iter: TokenIter,
    symbols: &mut SymbolContext,
    arena: &mut ExprArena,
) -> ExprIdx {
    Automata::new(db, token_iter, symbols, arena).parse_all()
}
