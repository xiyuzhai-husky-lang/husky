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
use husky_vfs::{ModulePath, Toolchain};

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
use parsec::{OriginalError, ParseContext};
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
    path: RegionPath,
    token_sheet_data: &'a TokenSheetData,
    parent_expr_region: Option<ExprRegion>,
    symbol_context: SymbolContextMut<'a>,
    expr_arena: ExprArena,
    entity_path_expr_arena: EntityPathExprArena,
    pattern_expr_region: PatternExprRegion,
    stmt_arena: StmtArena,
    expr_roots: Vec<ExprRoot>,
}

impl<'a> ExprParser<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        path: RegionPath,
        token_sheet_data: &'a TokenSheetData,
        module_symbol_context: ModuleSymbolContext<'a>,
        parent_expr_region: Option<ExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> Self {
        Self {
            db,
            path: path.into(),
            token_sheet_data,
            parent_expr_region,
            symbol_context: SymbolContextMut::new(
                module_symbol_context,
                parent_expr_region.map(|er| er.data(db).symbol_region()),
                allow_self_type,
                allow_self_value,
            ),
            expr_arena: Default::default(),
            entity_path_expr_arena: Default::default(),
            pattern_expr_region: Default::default(),
            stmt_arena: Default::default(),
            expr_roots: vec![],
        }
    }

    pub fn finish(self) -> ExprRegion {
        self.symbol_context.into_expr_region(
            self.db,
            self.parent_expr_region,
            self.path,
            self.expr_arena,
            self.entity_path_expr_arena,
            self.pattern_expr_region,
            self.stmt_arena,
            self.expr_roots,
        )
    }

    pub fn ctx<'b>(&'b mut self, token_stream: TokenStream<'a>) -> ExprParseContext<'a, 'b>
    where
        'a: 'b,
    {
        ExprParseContext::new(self, token_stream)
    }

    pub(crate) fn pattern_expr_region(&self) -> &PatternExprRegion {
        &self.pattern_expr_region
    }

    #[inline(always)]
    fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSymbol>,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> CurrentSymbolIdxRange {
        self.symbol_context.define_symbols(variables, ty_constraint)
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

    pub fn parse_expr_expected<E: OriginalError>(
        &mut self,
        env: ExprParseEnvironment,
        err: impl FnOnce(TokenIdx) -> E,
    ) -> Result<ExprIdx, E::Error> {
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
            None => Err(err(state).into()),
        }
    }

    pub fn parse_expr_expected2(
        &mut self,
        env: ExprParseEnvironment,
        err: impl FnOnce(TokenIdx) -> OriginalExprError,
    ) -> ExprIdx {
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
            Some(expr_idx) => expr_idx,
            None => todo!(), // Err(err(state)),
        }
    }

    pub(crate) fn pattern_expr_region(&self) -> &PatternExprRegion {
        self.parser.pattern_expr_region()
    }

    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSymbol>,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> CurrentSymbolIdxRange {
        self.parser.define_symbols(variables, ty_constraint)
    }

    pub fn parse_pattern_expr(
        &mut self,
        env: PatternExprInfo,
    ) -> ExprResult<Option<PatternExprIdx>> {
        if let Some(mut_token) = self.parse::<MutToken>()? {
            let ident_token: IdentifierToken =
                self.parse_expected(OriginalExprError::ExpectIdentifierAfterMut)?;
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
        let ident_token: EntityPathExprResult<IdentifierToken> =
            self.parse_expected(OriginalEntityPathExprError::ExpectIdentifierAfterScopeResolution);
        let path: EntityPathExprResult<EntityPath> = match parent_path {
            Some(parent_path) => match ident_token {
                Ok(ident_token) => {
                    let ident = ident_token.ident();
                    match self.parser.db.subentity_path(parent_path, ident) {
                        Ok(path) => Ok(path),
                        Err(error) => Err(OriginalEntityPathExprError::EntityTree {
                            token_idx: ident_token.token_idx(),
                            error,
                        }
                        .into()),
                    }
                }
                Err(_) => todo!(),
            },
            None => todo!(),
        };
        let parent_path = match path {
            Ok(path) => Some(path),
            Err(_) => None,
        };
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
        self.parser.symbol_context.symbol_region().allow_self_type()
    }

    fn allow_self_value(&self) -> AllowSelfValue {
        self.parser
            .symbol_context
            .symbol_region()
            .allow_self_value()
    }

    pub(crate) fn add_expr_root(&mut self, expr_root: ExprRoot) {
        self.parser.expr_roots.push(expr_root)
    }
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
