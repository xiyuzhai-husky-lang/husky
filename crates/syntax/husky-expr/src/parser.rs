mod accept;
mod alloc;
mod block;
mod debug;
mod disambiguate_token;
mod env;
mod expr_stack;
mod incomplete_expr;
mod root;

pub use self::block::*;
pub use self::env::*;
pub use self::root::*;

use self::incomplete_expr::*;
use crate::symbol::*;
use crate::*;
use disambiguate_token::*;
use expr_stack::*;
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_entity_tree::*;
use husky_token::Token;
use husky_token::TokenStream;
use husky_vfs::{ModulePath, Toolchain};
use original_error::IntoError;
use parsec::{HasStreamState, StreamParser};
use salsa::DebugWithDb;
use std::ops::ControlFlow;

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
    // todo: make this option
    module_path: ModulePath,
    crate_root_path: ModulePath,
    token_sheet_data: &'a TokenSheetData,
    parent_expr_region: Option<ExprRegion>,
    symbol_context: SymbolContextMut<'a>,
    expr_arena: ExprArena,
    principal_entity_path_expr_arena: PrincipalEntityPathExprArena,
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
        let module_path = path.module_path(db);
        Self {
            db,
            path,
            module_path,
            crate_root_path: module_path.crate_path(db).root_module_path(db),
            token_sheet_data,
            parent_expr_region,
            symbol_context: SymbolContextMut::new(
                module_symbol_context,
                parent_expr_region.map(|er| er.data(db).symbol_region()),
                allow_self_type,
                allow_self_value,
            ),
            expr_arena: Default::default(),
            principal_entity_path_expr_arena: Default::default(),
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
            self.principal_entity_path_expr_arena,
            self.pattern_expr_region,
            self.stmt_arena,
            self.expr_roots,
        )
    }

    pub fn ctx<'b>(
        &'b mut self,
        env: Option<ExprEnvironment>,
        token_group_idx: TokenGroupIdx,
        state: impl Into<Option<TokenStreamState>>,
    ) -> ExprParseContext<'a, 'b>
    where
        'a: 'b,
    {
        ExprParseContext::new(
            self,
            env,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, state),
        )
    }

    pub(crate) fn pattern_expr_region(&self) -> &PatternExprRegion {
        &self.pattern_expr_region
    }

    #[inline(always)]
    fn define_symbol(
        &mut self,
        variable: CurrentSymbol,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> CurrentSymbolIdx {
        self.symbol_context.define_symbol(variable, ty_constraint)
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
    env_stack: ExprEnvironmentStack,
    token_stream: TokenStream<'a>,
    stack: ExprStack,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(super) fn new(
        parser: &'b mut ExprParser<'a>,
        env: Option<ExprEnvironment>,
        token_stream: TokenStream<'a>,
    ) -> Self {
        Self {
            parser,
            env_stack: ExprEnvironmentStack::new(env),
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

    pub fn expr_arena(&self) -> &ExprArena {
        &self.parser.expr_arena
    }

    pub fn parse_expr_root(
        &mut self,
        env: impl Into<Option<ExprEnvironment>>,
        expr_root_kind: ExprRootKind,
    ) -> Option<ExprIdx> {
        let env = env.into();
        if let Some(env) = env {
            self.env_stack.set(env);
        }
        loop {
            let Some((token_idx, token)) = self.token_stream.next_indexed()
                else {
                    break
                };
            match self.disambiguate_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback_raw(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        if env.is_some() {
            self.env_stack.unset();
        }
        let opt_expr_idx = self.finish_batch();
        opt_expr_idx.map(|expr_idx| self.parser.add_expr_root(expr_root_kind, expr_idx));
        opt_expr_idx
    }

    pub fn parse_expr_expected<E: IntoError>(
        &mut self,
        env: Option<ExprEnvironment>,
        err: impl FnOnce(TokenStreamState) -> E,
    ) -> Result<ExprIdx, E::Error> {
        let state = self.save_state();
        if let Some(env) = env {
            self.env_stack.set(env);
        }
        loop {
            let Some((token_idx, token)) = self.token_stream.next_indexed()
                else {
                    break
                };
            match self.disambiguate_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback_raw(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        if env.is_some() {
            self.env_stack.unset();
        }
        match self.finish_batch() {
            Some(expr_idx) => Ok(expr_idx),
            None => Err(err(state).into()),
        }
    }

    pub fn parse_expr_expected2(
        &mut self,
        env: Option<ExprEnvironment>,
        expr_root_kind: ExprRootKind,
        err: impl FnOnce(TokenStreamState) -> OriginalExprError,
    ) -> ExprIdx {
        let state = self.save_state();
        if let Some(env) = env {
            self.env_stack.set(env);
        }
        loop {
            let Some((token_idx, token)) = self.token_stream.next_indexed()
                else {
                    break
                };
            match self.disambiguate_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback_raw(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        if env.is_some() {
            self.env_stack.unset();
        }
        let expr_idx = {
            match self.finish_batch() {
                Some(expr_idx) => expr_idx,
                None => self.alloc_expr(Expr::Err(err(state).into())),
            }
        };
        self.parser.add_expr_root(expr_root_kind, expr_idx);
        expr_idx
    }

    pub(crate) fn pattern_expr_region(&self) -> &PatternExprRegion {
        self.parser.pattern_expr_region()
    }

    pub(crate) fn define_symbol(
        &mut self,
        variable: CurrentSymbol,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> CurrentSymbolIdx {
        self.parser.define_symbol(variable, ty_constraint)
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
        let modifier_keyword_group = self.try_parse_optional()?;
        let ident_token = match modifier_keyword_group {
            Some(_) => self.try_parse_expected(OriginalExprError::ExpectedIdentAfterModifier)?,
            None => match self.try_parse_optional::<IdentToken>()? {
                Some(ident_token) => ident_token,
                None => return Ok(None),
            },
        };
        Ok(Some(self.alloc_pattern_expr(
            PatternExpr::Ident {
                modifier_keyword_group,
                ident_token,
            },
            env,
        )))
        // if let Some(ref_token) = self.parse::<RefToken>()? {
        //     todo!()
        // } else if let Some(mut_token) = self.parse::<MutToken>()? {
        //     let ident_token: IdentToken =
        //         self.parse_expected(OriginalExprError::ExpectedIdentAfterMut)?;
        //     Ok(Some(self.alloc_pattern_expr(
        //         PatternExpr::Ident {
        //             ident_token,
        //             modifier: SymbolModifierKeywordGroup::Mut(mut_token),
        //         },
        //         env,
        //     )))
        // } else if let Some(ident_token) = self.parse::<IdentToken>()? {
        //     Ok(Some(self.alloc_pattern_expr(
        //         PatternExpr::Ident {
        //             ident_token,
        //             modifier: SymbolModifierKeywordGroup::Pure,
        //         },
        //         env,
        //     )))
        // } else {
        //     Ok(None)
        // }
    }

    fn allow_self_ty(&self) -> AllowSelfType {
        self.parser.symbol_context.symbol_region().allow_self_ty()
    }

    fn allow_self_value(&self) -> AllowSelfValue {
        self.parser
            .symbol_context
            .symbol_region()
            .allow_self_value()
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
