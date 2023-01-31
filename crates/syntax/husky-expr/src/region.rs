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

    pub fn expr(&self) -> ArenaIdx<Expr> {
        self.expr
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExprRootKind {
    Type,
    Trait,
    OutputType,
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

    pub fn output_ty(&self) -> Option<ExprIdx> {
        self.roots
            .iter()
            .find_map(|root| (root.kind == ExprRootKind::OutputType).then_some(root.expr))
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
