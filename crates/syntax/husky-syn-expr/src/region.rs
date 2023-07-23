use crate::*;
use husky_vfs::{ModulePath, Toolchain};

#[salsa::tracked(db = SynExprDb, jar = SynExprJar)]
pub struct SynExprRegion {
    #[return_ref]
    pub data: SynExprRegionData,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynExprDb)]
pub struct SynExprRegionData {
    parent: Option<SynExprRegion>,
    path: RegionPath,
    expr_arena: SynExprArena,
    principal_entity_path_expr_arena: PrincipalEntityPathSynExprArena,
    stmt_arena: SynStmtArena,
    pattern_expr_region: PatternSynExprRegion,
    symbol_region: SynSymbolRegion,
    roots: Vec<SynExprRoot>,
}

impl SynExprRegionData {
    pub fn new(
        parent: Option<SynExprRegion>,
        path: RegionPath,
        expr_arena: SynExprArena,
        entity_path_expr_arena: PrincipalEntityPathSynExprArena,
        stmt_arena: SynStmtArena,
        pattern_expr_region: PatternSynExprRegion,
        symbol_region: SynSymbolRegion,
        roots: Vec<SynExprRoot>,
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

    pub fn expr_arena(&self) -> &SynExprArena {
        &self.expr_arena
    }

    pub fn principal_entity_path_expr_arena(&self) -> &PrincipalEntityPathSynExprArena {
        &self.principal_entity_path_expr_arena
    }

    pub fn pattern_expr_arena(&self) -> &PatternSynExprArena {
        self.pattern_expr_region.pattern_expr_arena()
    }

    pub fn stmt_arena(&self) -> &SynStmtArena {
        &self.stmt_arena
    }

    pub fn pattern_expr_region(&self) -> &PatternSynExprRegion {
        &self.pattern_expr_region
    }

    pub fn symbol_region(&self) -> &SynSymbolRegion {
        &self.symbol_region
    }

    pub fn roots(&self) -> &[SynExprRoot] {
        self.roots.as_ref()
    }

    pub fn return_ty(&self) -> Option<SynExprIdx> {
        self.roots
            .iter()
            .find_map(|root| (root.kind() == ExprRootKind::ReturnType).then_some(root.expr_idx()))
    }

    pub fn self_ty(&self) -> Option<SynExprIdx> {
        self.roots
            .iter()
            .find_map(|root| (root.kind() == ExprRootKind::SelfType).then_some(root.expr_idx()))
    }
}

impl std::ops::Index<SynExprIdx> for SynExprRegionData {
    type Output = SynExpr;

    fn index(&self, index: SynExprIdx) -> &Self::Output {
        &self.expr_arena[index]
    }
}

impl std::ops::Index<SynStmtIdx> for SynExprRegionData {
    type Output = SynStmt;

    fn index(&self, index: SynStmtIdx) -> &Self::Output {
        &self.stmt_arena[index]
    }
}
impl std::ops::Index<CurrentSymbolIdx> for SynExprRegionData {
    type Output = CurrentSymbol;

    fn index(&self, index: CurrentSymbolIdx) -> &Self::Output {
        &self.symbol_region[index]
    }
}
impl std::ops::Index<PatternSymbolIdx> for SynExprRegionData {
    type Output = PatternSynSymbol;

    fn index(&self, index: PatternSymbolIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}
impl std::ops::Index<PatternSynExprIdx> for SynExprRegionData {
    type Output = PatternSynExpr;

    fn index(&self, index: PatternSynExprIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}

impl SynExprRegion {
    pub fn toolchain(self, db: &dyn SynExprDb) -> Toolchain {
        // ad hoc
        match self.data(db).path {
            RegionPath::Snippet(module_path) => module_path.toolchain(db),
            RegionPath::Decr(id) => id.toolchain(db),
            RegionPath::Decl(id) | RegionPath::Defn(id) => id.toolchain(db),
        }
    }
}
