mod accept;
mod alloc;
mod debug;
mod env;
mod expr_stack;
mod list;
mod resolve;
mod symbol_stack;
mod unfinished_expr;

pub use env::*;
use husky_ast::{Ast, AstIdxRange, AstSheet};
use parsec::ParseContext;

use crate::*;
use expr_stack::*;
use husky_entity_tree::{CratePrelude, EntityTreeDb};
use husky_symbol::SymbolContext;
use husky_token::TokenStream;
use husky_token::{Token, TokenKind};
use list::*;
use resolve::*;
use std::ops::ControlFlow;
use symbol_stack::*;
use unfinished_expr::*;

pub struct ExprParser<'a> {
    db: &'a dyn ExprDb,
    entity_path: Option<EntityPath>,
    token_sheet: &'a TokenSheet,
    ast_sheet: Option<&'a AstSheet>,
    symbol_stack: SymbolStack<'a>,
    expr_arena: ExprArena,
    entity_path_expr_arena: EntityPathExprArena,
    pattern_expr_arena: PatternExprArena,
    stmt_arena: StmtArena,
}

impl<'a> ExprParser<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        entity_path: Option<EntityPath>,
        token_sheet: &'a TokenSheet,
        ast_sheet: Option<&'a AstSheet>,
        crate_prelude: CratePrelude<'a>,
    ) -> Self {
        Self {
            db,
            entity_path,
            token_sheet,
            ast_sheet,
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

    pub fn parse_block(&mut self, body: &AstIdxRange) -> Option<ExprIdx> {
        if body.len() == 0 {
            return None;
        }
        let stmts = self.ast_sheet.unwrap()[body]
            .iter()
            .filter_map(|ast| self.parse_stmt(ast))
            .collect();
        let stmts = self.alloc_stmts(stmts);
        Some(self.alloc_expr(Expr::Block { stmts }))
    }

    fn parse_stmt(&mut self, ast: &Ast) -> Option<Stmt> {
        match ast {
            Ast::BasicStmt {
                token_group_idx,
                body,
            } => {
                let token_stream = self
                    .token_sheet
                    .token_group_token_stream(*token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                match ctx.parse::<BasicStmtKeywordToken>() {
                    Ok(Some(basic_stmt_keyword_token)) => Some(match basic_stmt_keyword_token {
                        BasicStmtKeywordToken::Let(let_token) => Stmt::Let {
                            let_token,
                            let_variable_pattern: ctx.parse_expected(),
                            assign_token: ctx.parse_expected(),
                            initial_value: ctx
                                .parse_expr(ExprParseEnvironment::None)
                                .ok_or(ExprError::MissingInitialValue),
                        },
                        BasicStmtKeywordToken::Return(return_token) => {
                            Stmt::Return { return_token }
                        }
                        BasicStmtKeywordToken::Require(require_token) => {
                            Stmt::Require { require_token }
                        }
                        BasicStmtKeywordToken::Break(break_token) => Stmt::Break { break_token },
                    }),
                    Ok(None) => Some(Stmt::Eval {}),
                    Err(_) => todo!(),
                }
            }
            Ast::IfElseStmts {
                if_stmt,
                elif_stmts,
                else_stmt,
            } => Some(Stmt::IfElse {}),
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => Some(Stmt::Match {}),
            Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Comment { .. }
            | Ast::Decor { .. }
            | Ast::Defn { .. }
            | Ast::ModuleItemVariant { .. }
            | Ast::Impl { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => None,
        }
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
        while !self.tokens().is_empty() {
            let (token_idx, token) = self.token_stream.next_indexed(IgnoreComment::True).unwrap();
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
    token_sheet: &'a TokenSheet,
    token_iter: TokenStream<'a>,
    env: ExprParseEnvironment,
) -> (ExprSheet, Option<ExprIdx>) {
    let mut expr_parser = ExprParser::new(db, entity_path, token_sheet, None, crate_prelude);
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
