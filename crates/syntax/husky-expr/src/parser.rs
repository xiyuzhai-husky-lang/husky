mod accept;
mod opr;
mod resolve;
mod stack;
mod synthesize;

use crate::*;
use husky_check_utils::should;
use husky_entity_tree::EntityTreeDb;
use husky_symbol::SymbolContext;
use husky_token::TokenIter;
use husky_token::{Token, TokenKind};
use opr::*;
use resolve::*;
use stack::*;

pub(crate) struct ExprParser<'a, 'b, 'c> {
    ctx: SymbolContext<'c>,
    token_iter: &'a mut TokenIter<'b>,
    arena: &'a mut ExprArena,
    stack: AutomataStack,
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn tokens(&self) -> &TokenIter<'a> {
        &self.token_iter
    }

    fn new(
        ctx: SymbolContext<'c>,
        token_iter: &'a mut TokenIter<'b>,
        arena: &'a mut ExprArena,
    ) -> Self {
        Self {
            ctx,
            token_iter,
            arena,
            stack: Default::default(),
        }
    }

    fn parse_all(mut self) -> ExprIdx {
        while !self.tokens().is_empty() {
            let (token_idx, token) = self.token_iter.next_indexed().unwrap();
            match self.accept_token(self.resolve_token(token_idx, token)) {
                Ok(()) => (),
                Err(_) => todo!(),
            }
        }
        self.synthesize_all_above(Precedence::None).expect("todo");
        should!(self.number_of_exprs() == 1);
        let last_expr = self.pop_expr().unwrap();
        self.arena.alloc_one(last_expr)
    }
}

pub fn parse_expr<'a>(
    ctx: SymbolContext,
    token_iter: &mut TokenIter<'a>,
    arena: &mut ExprArena,
) -> ExprIdx {
    ExprParser::new(ctx, token_iter, arena).parse_all()
}
