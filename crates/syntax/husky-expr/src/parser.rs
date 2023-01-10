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

use crate::*;
use expr_stack::*;
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_entity_tree::{
    AssociatedItem, CratePrelude, EntityTreeDb, ImplBlock, ImplBlockId, ModuleSymbolContext,
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
    pattern_expr_sheet: PatternExprSheet,
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
            pattern_expr_sheet: Default::default(),
            stmt_arena: Default::default(),
        }
    }

    pub fn finish(self) -> ExprSheet {
        self.symbol_context.into_expr_sheet(
            self.db,
            self.expr_arena,
            self.entity_path_expr_arena,
            self.pattern_expr_sheet,
            self.stmt_arena,
        )
    }

    pub fn ctx<'b>(&'b mut self, token_stream: TokenStream<'a>) -> ExprParseContext<'a, 'b>
    where
        'a: 'b,
    {
        ExprParseContext::new(self, token_stream)
    }

    pub(crate) fn pattern_expr_sheet(&self) -> &PatternExprSheet {
        &self.pattern_expr_sheet
    }

    #[inline(always)]
    fn define_variables(&mut self, variables: Vec<LocalSymbol>) -> LocalSymbolIdxRange {
        self.symbol_context.define_variables(variables)
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

    pub(crate) fn pattern_expr_sheet(&self) -> &PatternExprSheet {
        self.parser.pattern_expr_sheet()
    }

    pub(crate) fn define_variables(&mut self, variables: Vec<LocalSymbol>) -> LocalSymbolIdxRange {
        self.parser.define_variables(variables)
    }

    pub fn parse_pattern_expr(&mut self, env: PatternInfo) -> ExprResult<Option<PatternExprIdx>> {
        if let Some(mut_token) = self.parse::<MutToken>()? {
            let ident_token =
                self.parse_expected2::<IdentifierToken>(ExprError::ExpectIdentifierAfterMut)?;
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
    ) -> (EntityPathExprIdx, ExprResult<EntityPath>) {
        let root = self.alloc_entity_path_expr(EntityPathExpr::Root {
            token_idx,
            ident,
            entity_path,
        });
        match self.try_parse::<ScopeResolutionToken>() {
            Some(scope_resolution_token) => {
                self.parse_subentity_path_expr(root, Ok(entity_path), scope_resolution_token)
            }
            None => (root, Ok(entity_path)),
        }
    }

    fn parse_subentity_path_expr(
        &mut self,
        parent: EntityPathExprIdx,
        parent_path: ExprResult<EntityPath>,
        scope_resolution_token: ScopeResolutionToken,
    ) -> (EntityPathExprIdx, ExprResult<EntityPath>) {
        let ident_token = self
            .parse_expected2::<IdentifierToken>(ExprError::ExpectIdentifierAfterScopeResolution);
        let ident: ExprResult<Identifier> = match ident_token {
            Ok(ident_token) => Ok(ident_token.ident()),
            Err(_) => todo!(),
        };
        let path: ExprResult<EntityPath> = parent_path
            .map(|parent_path| -> ExprResult<EntityPath> {
                Ok(self.parser.db.subentity_path(parent_path, ident?)?)
            })
            .flatten();
        let expr = EntityPathExpr::Subentity {
            parent,
            scope_resolution_token,
            ident_token,
            entity_path: path.clone(),
        };
        let expr = self.alloc_entity_path_expr(expr);
        match self.try_parse::<ScopeResolutionToken>() {
            Some(scope_resolution_token) => {
                self.parse_subentity_path_expr(expr, path, scope_resolution_token)
            }
            None => (expr, path),
        }
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
