mod accept;
mod alloc;
mod block;
mod debug;
mod env;
mod expr_stack;
mod list;
mod resolve;
mod unfinished_expr;

pub use block::*;
pub use env::*;
use husky_print_utils::p;
use outcome::Outcome;

use crate::*;
use expr_stack::*;
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_entity_tree::{
    AssociatedItem, CrateSymbolContext, EntityTreeDb, ImplBlock, ImplBlockId, ModuleSymbolContext,
    PreludeResult,
};
use husky_token::Token;
use husky_token::TokenStream;
use list::*;
use parsec::ParseContext;
use resolve::*;
use salsa::DebugWithDb;
use std::ops::ControlFlow;
use symbol::*;
use unfinished_expr::*;
use Outcome::*;

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

pub enum ExprPath {
    None,
    Decl(DeclExprPath),
    Defn(DefnExprPath),
}

impl From<DefnExprPath> for ExprPath {
    fn from(v: DefnExprPath) -> Self {
        Self::Defn(v)
    }
}

impl From<DeclExprPath> for ExprPath {
    fn from(v: DeclExprPath) -> Self {
        Self::Decl(v)
    }
}

pub enum DeclExprPath {
    Entity(EntityPath),
    ImplBlock(ImplBlock),
    AssociatedItem(AssociatedItem),
}

pub enum DefnExprPath {
    Entity(EntityPath),
    AssociatedItem(AssociatedItem),
}

impl<Db: ExprDb + ?Sized> DebugWithDb<Db> for ExprPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

pub struct ExprParser<'a> {
    db: &'a dyn ExprDb,
    path: ExprPath,
    token_sheet_data: &'a TokenSheetData,
    symbol_context: SymbolContextMut<'a>,
    expr_arena: ExprArena,
    entity_path_expr_arena: EntityPathExprArena,
    pattern_expr_page: PatternExprSubsheet,
    stmt_arena: StmtArena,
}

impl<'a> ExprParser<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        path: ExprPath,
        token_sheet_data: &'a TokenSheetData,
        symbol_context: SymbolContextMut<'a>,
    ) -> Self {
        Self {
            db,
            path: path.into(),
            token_sheet_data,
            symbol_context,
            expr_arena: Default::default(),
            entity_path_expr_arena: Default::default(),
            pattern_expr_page: Default::default(),
            stmt_arena: Default::default(),
        }
    }

    pub fn finish(self) -> ExprPage {
        self.symbol_context.into_expr_page(
            self.db,
            self.expr_arena,
            self.entity_path_expr_arena,
            self.pattern_expr_page,
            self.stmt_arena,
        )
    }

    pub fn ctx<'b>(&'b mut self, token_stream: TokenStream<'a>) -> ExprParseContext<'a, 'b>
    where
        'a: 'b,
    {
        ExprParseContext::new(self, token_stream)
    }

    pub(crate) fn pattern_expr_page(&self) -> &PatternExprSubsheet {
        &self.pattern_expr_page
    }

    #[inline(always)]
    fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSymbol>,
    ) -> CurrentSymbolIdxRange {
        self.symbol_context.define_symbols(variables)
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

    pub(crate) fn db(&self) -> &'a dyn ExprDb {
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

    pub fn parse_expr_expected<Error>(
        &mut self,
        env: ExprParseEnvironment,
        err: impl FnOnce(TokenIdx) -> Error,
    ) -> Result<ExprIdx, Error> {
        let state = self.state();
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
        match self.finish_batch() {
            Some(expr_idx) => Ok(expr_idx),
            None => Err(err(state)),
        }
    }

    pub(crate) fn pattern_expr_page(&self) -> &PatternExprSubsheet {
        self.parser.pattern_expr_page()
    }

    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSymbol>,
    ) -> CurrentSymbolIdxRange {
        self.parser.define_symbols(variables)
    }

    pub fn parse_pattern_expr(
        &mut self,
        env: PatternExprInfo,
    ) -> ExprResult<Option<PatternExprIdx>> {
        if let Some(mut_token) = self.parse::<MutToken>()? {
            let ident_token =
                self.parse_expected2::<IdentifierToken, _>(ExprError::ExpectIdentifierAfterMut)?;
            Ok(Some(self.alloc_pattern_expr(
                PatternExpr::Identifier {
                    ident_token,
                    liason: PatternLiason::None,
                },
                env,
            )))
        } else if let Some(ident_token) = self.parse::<IdentifierToken>()? {
            Ok(Some(self.alloc_pattern_expr(
                PatternExpr::Identifier {
                    ident_token,
                    liason: PatternLiason::None,
                },
                env,
            )))
        } else {
            Ok(None)
        }
    }

    fn parse_entity_path_expr(
        &mut self,
        token_idx: TokenIdx,
        ident: Identifier,
        entity_path: EntityPath,
    ) -> (EntityPathExprIdx, Option<EntityPath>) {
        let root = self.alloc_entity_path_expr(EntityPathExpr::Root {
            token_idx,
            ident,
            entity_path,
        });
        match self.try_parse::<ScopeResolutionToken>() {
            Some(scope_resolution_token) => {
                self.parse_subentity_path_expr(root, Some(entity_path), scope_resolution_token)
            }
            None => (root, Some(entity_path)),
        }
    }

    fn parse_subentity_path_expr(
        &mut self,
        parent: EntityPathExprIdx,
        parent_path: Option<EntityPath>,
        scope_resolution_token: ScopeResolutionToken,
    ) -> (EntityPathExprIdx, Option<EntityPath>) {
        let ident_token = self.parse_expected2::<IdentifierToken, _>(
            EntityPathExprError::ExpectIdentifierAfterScopeResolution,
        );
        let path: EntityPathExprOutcome<EntityPath> = match parent_path {
            Some(parent_path) => match ident_token {
                Ok(ident_token) => {
                    let ident = ident_token.ident();
                    match self.parser.db.subentity_path(parent_path, ident) {
                        Ok(path) => Success(path),
                        Err(error) => Failure(EntityPathExprError::EntityTree {
                            token_idx: ident_token.token_idx(),
                            error,
                        }),
                    }
                }
                Err(_) => todo!(),
            },
            None => todo!(),
        };
        let parent_path = path.ok_copy();
        let expr = EntityPathExpr::Subentity {
            parent,
            scope_resolution_token,
            ident_token,
            path,
        };
        let expr = self.alloc_entity_path_expr(expr);
        match self.try_parse::<ScopeResolutionToken>() {
            Some(scope_resolution_token) => {
                self.parse_subentity_path_expr(expr, parent_path, scope_resolution_token)
            }
            None => (expr, parent_path),
        }
    }

    fn allow_self_type(&self) -> AllowSelfType {
        self.parser.symbol_context.symbol_sheet().allow_self_type()
    }

    fn allow_self_value(&self) -> AllowSelfValue {
        self.parser.symbol_context.symbol_sheet().allow_self_value()
    }
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

impl<'a, 'b> parsec::StreamWrapper for ExprParseContext<'a, 'b> {}
