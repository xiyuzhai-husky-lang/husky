use crate::*;
use husky_vfs::{ModulePath, Toolchain};

#[salsa::tracked(jar = ExprJar)]
pub struct ExprRegion {
    #[return_ref]
    pub data: ExprRegionData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExprRegionData {
    parent: Option<ExprRegion>,
    path: ExprRegionPath,
    expr_arena: ExprArena,
    entity_path_expr_arena: EntityPathExprArena,
    stmt_arena: StmtArena,
    pattern_expr_region: PatternExprRegion,
    symbol_region: SymbolRegion,
}

impl ExprRegionData {
    pub fn new(
        parent: Option<ExprRegion>,
        path: ExprRegionPath,
        expr_arena: ExprArena,
        entity_path_expr_arena: EntityPathExprArena,
        stmt_arena: StmtArena,
        pattern_expr_region: PatternExprRegion,
        symbol_region: SymbolRegion,
    ) -> Self {
        Self {
            parent,
            path,
            expr_arena,
            entity_path_expr_arena,
            stmt_arena,
            pattern_expr_region,
            symbol_region,
        }
    }

    pub fn parent(&self) -> Option<ExprRegion> {
        self.parent
    }

    pub fn path(&self) -> ExprRegionPath {
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
}

impl ExprRegion {
    pub fn toolchain(self, db: &dyn ExprDb) -> Toolchain {
        // ad hoc
        match self.data(db).path {
            ExprRegionPath::Snippet(toolchain) => toolchain,
            ExprRegionPath::Decl(path) => match path {
                DeclExprPath::Entity(path) => path.toolchain(db),
                DeclExprPath::ImplBlock(impl_block) => impl_block.module_path(db).toolchain(db),
                DeclExprPath::AssociatedItem(item) => item.module_path(db).toolchain(db),
            },
            ExprRegionPath::Defn(path) => match path {
                DefnExprPath::Entity(path) => path.toolchain(db),
                DefnExprPath::AssociatedItem(item) => item.module_path(db).toolchain(db),
            },
        }
    }
}
