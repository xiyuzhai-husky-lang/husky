mod accept;
mod alloc;
mod block;
mod env;
mod expr_stack;
mod list;
mod resolve;
mod symbol;
mod unfinished_expr;

pub use block::*;
pub use env::*;

use crate::*;
use expr_stack::*;
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_entity_tree::{CratePrelude, EntityTreeDb};
use husky_symbol::SymbolContext;
use husky_token::Token;
use husky_token::TokenStream;
use list::*;
use parsec::ParseContext;
use resolve::*;
use salsa::DebugWithDb;
use std::ops::ControlFlow;
use symbol::*;
use unfinished_expr::*;

#[macro_use]
macro_rules! report {
    ($self: expr) => {{
        p!(
            $self.stack,
            $self.parser.entity_path.debug($self.db()) // $self.token_stream.text_range()
        );
    }};
}
use report;

pub struct ExprParser<'a> {
    db: &'a dyn ExprDb,
    entity_path: Option<EntityPath>,
    token_sheet_data: &'a TokenSheetData,
    symbol_stack: SymbolStack<'a>,
    expr_arena: ExprArena,
    entity_path_expr_arena: EntityPathExprArena,
    pattern_expr_arena: PatternExprSheet,
    stmt_arena: StmtArena,
}

impl<'a> ExprParser<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        entity_path: Option<EntityPath>,
        token_sheet_data: &'a TokenSheetData,
        crate_prelude: CratePrelude<'a>,
    ) -> Self {
        Self {
            db,
            entity_path,
            token_sheet_data,
            symbol_stack: SymbolStack::new(crate_prelude),
            expr_arena: Default::default(),
            entity_path_expr_arena: Default::default(),
            pattern_expr_arena: Default::default(),
            stmt_arena: Default::default(),
        }
    }

    pub fn finish(self) -> ExprSheet {
        ExprSheet::new(
            self.db,
            self.expr_arena,
            self.entity_path_expr_arena,
            self.pattern_expr_arena,
            self.stmt_arena,
        )
    }

    pub fn ctx<'b>(&'b mut self, token_stream: TokenStream<'a>) -> ExprParseContext<'a, 'b>
    where
        'a: 'b,
    {
        ExprParseContext::new(self, token_stream)
    }
}

pub struct ExprParseContext<'a, 'b> {
    parser: &'b mut ExprParser<'a>,
    env: ExprParseEnvironmentPlace,
    token_stream: TokenStream<'a>,
    stack: ExprStack,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    fn new(parser: &'b mut ExprParser<'a>, token_stream: TokenStream<'a>) -> Self {
        Self {
            parser,
            env: Default::default(),
            token_stream,
            stack: Default::default(),
        }
    }

    pub(crate) fn db(&self) -> &'a dyn EntityTreeDb {
        self.parser.db
    }

    pub(crate) fn tokens(&self) -> &TokenStream<'a> {
        &self.token_stream
    }

    pub fn parse_expr(&mut self, env: ExprParseEnvironment) -> Option<ExprIdx> {
        self.env.set(env);
        loop {
            let Some((token_idx, token)) = self.token_stream.next_indexed()
                else {
                    break
                };
            match self.resolve_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        self.env.unset();
        self.finish_batch()
    }
}

pub fn parse_expr<'a>(
    db: &'a dyn ExprDb,
    entity_path: Option<EntityPath>,
    crate_prelude: CratePrelude<'a>,
    token_sheet_data: &'a TokenSheetData,
    token_iter: TokenStream<'a>,
    env: ExprParseEnvironment,
) -> (ExprSheet, Option<ExprIdx>) {
    let mut expr_parser = ExprParser::new(db, entity_path, token_sheet_data, crate_prelude);
    let expr = expr_parser.ctx(token_iter).parse_expr(env);
    (expr_parser.finish(), expr)
}

impl<'a, 'b> parsec::HasParseError for ExprParseContext<'a, 'b> {
    type Error = ExprError;
}

impl<'a, 'b> std::ops::Deref for ExprParseContext<'a, 'b> {
    type Target = TokenStream<'a>;
    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for ExprParseContext<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for ExprParseContext<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for ExprParseContext<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b, 'c> parsec::StreamWrapper for ExprParseContext<'a, 'b> {}
