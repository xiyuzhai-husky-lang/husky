use crate::*;
use husky_vfs::{ModulePath, Toolchain};

#[salsa::tracked(db = ExprDb, jar = ExprJar)]
pub struct ExprRegion {
    #[return_ref]
    pub data: ExprRegionData,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct ExprRegionData {
    parent: Option<ExprRegion>,
    path: RegionPath,
    expr_arena: ExprArena,
    entity_path_expr_arena: EntityPathExprArena,
    stmt_arena: StmtArena,
    pattern_expr_region: PatternExprRegion,
    symbol_region: SymbolRegion,
    roots: Vec<ExprRoot>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ExprRoot {
    kind: ExprRootKind,
    expr: ExprIdx,
}

impl ExprRoot {
    pub fn new(kind: ExprRootKind, expr: ExprIdx) -> Self {
        Self { kind, expr }
    }

    pub fn kind(&self) -> ExprRootKind {
        self.kind
    }

    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExprRootKind {
    SelfType,
    Trait,
    ReturnType,
    FieldType,
    BlockExpr,
}

impl ExprRegionData {
    pub fn new(
        parent: Option<ExprRegion>,
        path: RegionPath,
        expr_arena: ExprArena,
        entity_path_expr_arena: EntityPathExprArena,
        stmt_arena: StmtArena,
        pattern_expr_region: PatternExprRegion,
        symbol_region: SymbolRegion,
        roots: Vec<ExprRoot>,
    ) -> Self {
        Self {
            parent,
            path,
            expr_arena,
            entity_path_expr_arena,
            stmt_arena,
            pattern_expr_region,
            symbol_region,
            roots,
        }
    }

    pub fn parent(&self) -> Option<ExprRegion> {
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

    pub fn entity_path_expr_arena(&self) -> &EntityPathExprArena {
        &self.entity_path_expr_arena
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
            .find_map(|root| (root.kind == ExprRootKind::ReturnType).then_some(root.expr))
    }

    pub fn self_ty(&self) -> Option<ExprIdx> {
        self.roots
            .iter()
            .find_map(|root| (root.kind == ExprRootKind::SelfType).then_some(root.expr))
    }
}

impl std::ops::Index<ExprIdx> for ExprRegionData {
    type Output = Expr;

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

impl ExprRegion {
    pub fn toolchain(self, db: &dyn ExprDb) -> Toolchain {
        // ad hoc
        match self.data(db).path {
            RegionPath::Snippet(toolchain) => toolchain,
            RegionPath::Decl(path) => match path {
                DeclExprPath::Entity(path) => path.toolchain(db),
                DeclExprPath::ImplBlock(impl_block) => impl_block.module_path().toolchain(db),
                DeclExprPath::AssociatedItem(item) => item.module_path().toolchain(db),
            },
            RegionPath::Defn(path) => match path {
                DefnExprPath::Entity(path) => path.toolchain(db),
                DefnExprPath::AssociatedItem(item) => item.module_path().toolchain(db),
            },
        }
    }
}
