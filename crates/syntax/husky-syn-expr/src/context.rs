mod root;
mod stmt;

pub use self::root::*;
pub use self::stmt::*;

use crate::*;
use husky_vfs::ModulePath;

pub struct SynExprContext<'a> {
    db: &'a dyn SynExprDb,
    path: RegionPath,
    module_path: ModulePath,
    crate_root_path: ModulePath,
    parent_syn_expr_region: Option<SynExprRegion>,
    syn_symbol_context: SynSymbolContextMut<'a>,
    syn_expr_arena: SynExprArena,
    syn_principal_entity_path_expr_arena: SynPrincipalEntityPathExprArena,
    syn_pattern_expr_region: SynPatternExprRegion,
    syn_stmt_arena: SynStmtArena,
    syn_pattern_expr_roots: Vec<SynPatternExprRoot>,
    syn_expr_roots: Vec<SynExprRoot>,
    has_self_lifetime: bool,
    has_self_place: bool,
}

pub trait IsSynExprContext<'a>:
    std::borrow::Borrow<SynExprContext<'a>> + std::borrow::BorrowMut<SynExprContext<'a>>
{
}

impl<'a> IsSynExprContext<'a> for SynExprContext<'a> {}

impl<'a, 'b> IsSynExprContext<'a> for &'b mut SynExprContext<'a> {}

impl<'a> SynExprContext<'a> {
    pub fn new(
        db: &'a dyn SynExprDb,
        path: RegionPath,
        module_symbol_context: ModuleSymbolContext<'a>,
        parent_expr_region: Option<SynExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> Self {
        let module_path = path.module_path(db);
        Self {
            db,
            path,
            module_path,
            crate_root_path: module_path.crate_path(db).root_module_path(db),
            parent_syn_expr_region: parent_expr_region,
            syn_symbol_context: SynSymbolContextMut::new(
                module_symbol_context,
                parent_expr_region.map(|er| er.data(db).symbol_region()),
                allow_self_type,
                allow_self_value,
            ),
            syn_expr_arena: Default::default(),
            syn_principal_entity_path_expr_arena: Default::default(),
            syn_pattern_expr_region: Default::default(),
            syn_stmt_arena: Default::default(),
            syn_pattern_expr_roots: vec![],
            syn_expr_roots: vec![],
            has_self_lifetime: false,
            has_self_place: false,
        }
    }

    pub fn finish(self) -> SynExprRegion {
        self.syn_symbol_context.into_expr_region(
            self.db,
            self.parent_syn_expr_region,
            self.path,
            self.syn_expr_arena,
            self.syn_principal_entity_path_expr_arena,
            self.syn_pattern_expr_region,
            self.syn_stmt_arena,
            self.syn_pattern_expr_roots,
            self.syn_expr_roots,
            self.has_self_lifetime,
            self.has_self_place,
        )
    }

    pub fn expr_parser(
        self,
        env: Option<ExprEnvironment>,
        token_stream: RegionalTokenStream<'a>,
    ) -> SynExprParser<'a, Self> {
        SynExprParser::new(self, env, token_stream)
    }

    pub(crate) fn pattern_expr_region(&self) -> &SynPatternExprRegion {
        &self.syn_pattern_expr_region
    }

    #[inline(always)]
    pub(crate) fn define_symbol(
        &mut self,
        variable: CurrentSynSymbol,
        ty_constraint: Option<SyndicateTypeConstraint>,
    ) -> CurrentSynSymbolIdx {
        self.syn_symbol_context
            .define_symbol(variable, ty_constraint)
    }

    #[inline(always)]
    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSynSymbol>,
        ty_constraint: Option<SyndicateTypeConstraint>,
    ) -> CurrentSynSymbolIdxRange {
        self.syn_symbol_context
            .define_symbols(variables, ty_constraint)
    }

    pub fn db(&self) -> &'a dyn SynExprDb {
        self.db
    }

    pub fn syn_expr_arena(&self) -> &SynExprArena {
        &self.syn_expr_arena
    }

    pub fn syn_symbol_context(&self) -> &SynSymbolContextMut<'a> {
        &self.syn_symbol_context
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn syn_pattern_expr_region(&self) -> &SynPatternExprRegion {
        &self.syn_pattern_expr_region
    }

    pub(crate) fn syn_expr_arena_mut(&mut self) -> &mut SynExprArena {
        &mut self.syn_expr_arena
    }

    pub(crate) fn alloc_expr(&mut self, syn_expr: SynExprData) -> SynExprIdx {
        self.syn_expr_arena.alloc_one(syn_expr)
    }

    pub(crate) fn alloc_inline_stmt(&mut self, syn_stmt: SynStmtData) -> SynStmtIdxRange {
        SynStmtIdxRange::new_single(self.syn_stmt_arena.alloc_one(syn_stmt))
    }

    pub(super) fn alloc_stmts(&mut self, syn_stmts: Vec<SynStmtData>) -> SynStmtIdxRange {
        self.syn_stmt_arena.alloc_batch(syn_stmts)
    }

    pub(crate) fn alloc_pattern_expr(&mut self, expr: SynPatternExpr) -> SynPatternExprIdx {
        self.syn_pattern_expr_region.alloc_one_pattern_expr(expr)
    }

    pub(crate) fn alloc_item_path_expr(
        &mut self,
        expr: SynPrincipalEntityPathExpr,
    ) -> SynPrincipalEntityPathExprIdx {
        self.syn_principal_entity_path_expr_arena.alloc_one(expr)
    }

    pub fn crate_root_path(&self) -> ModulePath {
        self.crate_root_path
    }

    pub fn set_has_self_lifetime(&mut self) {
        debug_assert!(!self.has_self_lifetime);
        self.has_self_lifetime = true;
    }

    pub fn set_has_self_place(&mut self) {
        debug_assert!(!self.has_self_place);
        self.has_self_place = true;
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }

    pub fn syn_principal_entity_path_expr_arena(&self) -> &SynPrincipalEntityPathExprArena {
        &self.syn_principal_entity_path_expr_arena
    }
}
