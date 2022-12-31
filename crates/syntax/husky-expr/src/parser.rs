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
use husky_token::TokenStream;
use husky_token::{Token, TokenKind};
use opr::*;
use resolve::*;
use stack::*;
use std::ops::ControlFlow;

pub struct ExprParser<'sheet, 'token, 'context> {
    ctx: SymbolContext<'context>,
    token_iter: TokenStream<'token>,
    sheet: &'sheet mut ExprSheet,
    stack: AutomataStack,
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub fn new(
        ctx: SymbolContext<'c>,
        token_iter: TokenStream<'b>,
        sheet: &'a mut ExprSheet,
    ) -> Self {
        Self {
            ctx,
            token_iter,
            sheet,
            stack: Default::default(),
        }
    }

    pub(crate) fn tokens(&self) -> &TokenStream<'b> {
        &self.token_iter
    }

    pub fn parse_exprs(&mut self) -> (ExprIdxRange, ExprParsingStopReason) {
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
                ControlFlow::Break(reason) => return (self.finish_batch(), reason),
            }
        }
        self.synthesize_all_above(Precedence::None).expect("todo");
        if self.number_of_exprs() != 1 {
            p!(self.stack);
            todo!()
        }
        should!(self.number_of_exprs() == 1);
        (self.finish_batch(), ExprParsingStopReason::NoTokens)
    }
}

pub fn parse_expr<'a>(
    ctx: SymbolContext,
    token_iter: TokenStream<'a>,
    sheet: &mut ExprSheet,
) -> (ExprIdxRange, ExprParsingStopReason) {
    ExprParser::new(ctx, token_iter, sheet).parse_exprs()
}

impl<'a, 'b, 'c> parsec::HasParseError for ExprParser<'a, 'b, 'c> {
    type Error = ExprError;
}

impl<'a, 'b, 'c> std::ops::Deref for ExprParser<'a, 'b, 'c> {
    type Target = TokenStream<'b>;
    fn deref(&self) -> &Self::Target {
        &self.token_iter
    }
}

impl<'a, 'b, 'c> std::ops::DerefMut for ExprParser<'a, 'b, 'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_iter
    }
}

impl<'a, 'b, 'c> std::borrow::Borrow<TokenStream<'b>> for ExprParser<'a, 'b, 'c> {
    fn borrow(&self) -> &TokenStream<'b> {
        &self.token_iter
    }
}

impl<'a, 'b, 'c> std::borrow::BorrowMut<TokenStream<'b>> for ExprParser<'a, 'b, 'c> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'b> {
        &mut self.token_iter
    }
}

impl<'a, 'b, 'c> parsec::StreamWrapper for ExprParser<'a, 'b, 'c> {}
