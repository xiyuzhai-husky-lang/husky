mod accept;
mod opr;
mod resolve;
mod stack;
mod stop_reason;
mod synthesize;
mod utils;

pub use stop_reason::ExprParsingStopReason;

use crate::*;
use husky_check_utils::should;
use husky_entity_tree::EntityTreeDb;
use husky_print_utils::p;
use husky_symbol::SymbolContext;
use husky_token::TokenIter;
use husky_token::{Token, TokenKind};
use opr::*;
use resolve::*;
use stack::*;
use std::ops::ControlFlow;

pub(crate) struct ExprParser<'a, 'b, 'c> {
    ctx: SymbolContext<'c>,
    token_iter: &'a mut TokenIter<'b>,
    sheet: &'a mut ExprSheet,
    stack: AutomataStack,
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn tokens(&self) -> &TokenIter<'a> {
        &self.token_iter
    }

    fn new(
        ctx: SymbolContext<'c>,
        token_iter: &'a mut TokenIter<'b>,
        sheet: &'a mut ExprSheet,
    ) -> Self {
        Self {
            ctx,
            token_iter,
            sheet,
            stack: Default::default(),
        }
    }

    fn parse_all(mut self) -> (ExprIdxRange, ExprParsingStopReason) {
        while !self.tokens().is_empty() {
            let (token_idx, token) = self.token_iter.next_indexed(IgnoreComment::True).unwrap();
            match self.resolve_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => match self.accept_token(resolved_token) {
                    Ok(()) => (),
                    Err(e) => {
                        p!(self.report_position());
                        todo!("error = {e}")
                    }
                },
                ControlFlow::Break(reason) => return (self.finish(), reason),
            }
        }
        self.synthesize_all_above(Precedence::None).expect("todo");
        if self.number_of_exprs() != 1 {
            p!(self.stack);
            todo!()
        }
        should!(self.number_of_exprs() == 1);
        (self.finish(), ExprParsingStopReason::NoTokens)
    }
}

pub fn parse_expr<'a>(
    ctx: SymbolContext,
    token_iter: &mut TokenIter<'a>,
    sheet: &mut ExprSheet,
) -> (ExprIdxRange, ExprParsingStopReason) {
    ExprParser::new(ctx, token_iter, sheet).parse_all()
}
