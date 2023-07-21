use crate::*;
use husky_vfs::{ModulePath, Toolchain};

#[salsa::tracked(db = ExprDb, jar = SynExprJar)]
pub struct SynExprRegion {
    #[return_ref]
    pub data: ExprRegionData,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct ExprRegionData {
    parent: Option<SynExprRegion>,
    path: RegionPath,
    expr_arena: ExprArena,
    principal_entity_path_expr_arena: PrincipalEntityPathExprArena,
    stmt_arena: StmtArena,
    pattern_expr_region: PatternExprRegion,
    symbol_region: SymbolRegion,
    roots: Vec<ExprRoot>,
}

impl ExprRegionData {
    pub fn new(
        parent: Option<SynExprRegion>,
        path: RegionPath,
        expr_arena: ExprArena,
        entity_path_expr_arena: PrincipalEntityPathExprArena,
        stmt_arena: StmtArena,
        pattern_expr_region: PatternExprRegion,
        symbol_region: SymbolRegion,
        roots: Vec<ExprRoot>,
    ) -> Self {
        Self {
            parent,
            path,
            expr_arena,
            principal_entity_path_expr_arena: entity_path_expr_arena,
            stmt_arena,
            pattern_expr_region,
            symbol_region,
            roots,
        }
    }

    pub fn parent(&self) -> Option<SynExprRegion> {
        self.parent
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }

    pub fn path_ref(&self) -> &RegionPath {
        &self.path
    }

    pub fn expr_arena(&self) -> &ExprArena {
        &self.expr_arena
    }

    pub fn principal_entity_path_expr_arena(&self) -> &PrincipalEntityPathExprArena {
        &self.principal_entity_path_expr_arena
    }

    pub fn pattern_expr_arena(&self) -> &PatternExprArena {
        self.pattern_expr_region.pattern_expr_arena()
    }

    pub fn stmt_arena(&self) -> &StmtArena {
        &self.stmt_arena
    }

    pub fn pattern_expr_region(&self) -> &PatternExprRegion {
        &self.pattern_expr_region
    }

    pub fn symbol_region(&self) -> &SymbolRegion {
        &self.symbol_region
    }

    pub fn roots(&self) -> &[ExprRoot] {
        self.roots.as_ref()
    }

    pub fn return_ty(&self) -> Option<ExprIdx> {
        self.roots
            .iter()
            .find_map(|root| (root.kind() == ExprRootKind::ReturnType).then_some(root.expr_idx()))
    }

    pub fn self_ty(&self) -> Option<ExprIdx> {
        self.roots
            .iter()
            .find_map(|root| (root.kind() == ExprRootKind::SelfType).then_some(root.expr_idx()))
    }
}

impl std::ops::Index<ExprIdx> for ExprRegionData {
    type Output = SynExpr;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_arena[index]
    }
}

impl std::ops::Index<StmtIdx> for ExprRegionData {
    type Output = Stmt;

    fn index(&self, index: StmtIdx) -> &Self::Output {
        &self.stmt_arena[index]
    }
}
impl std::ops::Index<CurrentSymbolIdx> for ExprRegionData {
    type Output = CurrentSymbol;

    fn index(&self, index: CurrentSymbolIdx) -> &Self::Output {
        &self.symbol_region[index]
    }
}
impl std::ops::Index<PatternSymbolIdx> for ExprRegionData {
    type Output = PatternSymbol;

    fn index(&self, index: PatternSymbolIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}
impl std::ops::Index<PatternExprIdx> for ExprRegionData {
    type Output = PatternExpr;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}

impl SynExprRegion {
    pub fn toolchain(self, db: &dyn ExprDb) -> Toolchain {
        // ad hoc
        match self.data(db).path {
            RegionPath::Snippet(module_path) => module_path.toolchain(db),
            RegionPath::Decr(id) => id.toolchain(db),
            RegionPath::Decl(id) | RegionPath::Defn(id) => id.toolchain(db),
        }
    }
}
