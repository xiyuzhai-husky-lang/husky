use crate::*;
use husky_vfs::{ModulePath, Toolchain};

#[salsa::tracked(jar = ExprJar)]
pub struct ExprRegion {
    pub parent: Option<ExprRegion>,
    pub path: ExprRegionPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub stmt_arena: StmtArena,
    #[return_ref]
    pub pattern_expr_region: PatternExprRegion,
    #[return_ref]
    pub symbol_region: SymbolRegion,
}

impl ExprRegion {
    pub fn toolchain(self, db: &dyn ExprDb) -> Toolchain {
        // ad hoc
        match self.path(db) {
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
