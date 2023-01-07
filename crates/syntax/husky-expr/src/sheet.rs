use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprSheet {
    Decl(DeclExprSheet),
    Defn(DefnExprSheet),
}

impl From<DefnExprSheet> for ExprSheet {
    fn from(v: DefnExprSheet) -> Self {
        Self::Defn(v)
    }
}

impl From<DeclExprSheet> for ExprSheet {
    fn from(v: DeclExprSheet) -> Self {
        Self::Decl(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclExprSheet {
    ModuleItemDecl(ModuleItemDeclExprSheet),
    AssociatedItemDecl(AssociatedItemDeclExprSheet),
    VariantDecl(VariantDeclExprSheet),
}

impl From<AssociatedItemDeclExprSheet> for DeclExprSheet {
    fn from(v: AssociatedItemDeclExprSheet) -> Self {
        Self::AssociatedItemDecl(v)
    }
}

impl From<ModuleItemDeclExprSheet> for DeclExprSheet {
    fn from(v: ModuleItemDeclExprSheet) -> Self {
        Self::ModuleItemDecl(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefnExprSheet {
    ModuleItemDefn(ModuleItemDefnExprSheet),
    AssociatedItemDefn(AssociatedItemDefnExprSheet),
}

impl From<AssociatedItemDefnExprSheet> for DefnExprSheet {
    fn from(v: AssociatedItemDefnExprSheet) -> Self {
        Self::AssociatedItemDefn(v)
    }
}

impl From<ModuleItemDefnExprSheet> for DefnExprSheet {
    fn from(v: ModuleItemDefnExprSheet) -> Self {
        Self::ModuleItemDefn(v)
    }
}

#[salsa::tracked(jar = ExprJar)]
pub struct ModuleItemDeclExprSheet {
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub pattern_expr_arena: PatternExprSheet,
    #[return_ref]
    pub stmt_arena: StmtArena,
}

#[salsa::tracked(jar = ExprJar)]
pub struct ModuleItemDefnExprSheet {
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub pattern_expr_arena: PatternExprSheet,
    #[return_ref]
    pub stmt_arena: StmtArena,
}

#[salsa::tracked(jar = ExprJar)]
pub struct AssociatedItemDeclExprSheet {
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub pattern_expr_arena: PatternExprSheet,
    #[return_ref]
    pub stmt_arena: StmtArena,
}

#[salsa::tracked(jar = ExprJar)]
pub struct VariantDeclExprSheet {
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub pattern_expr_arena: PatternExprSheet,
    #[return_ref]
    pub stmt_arena: StmtArena,
}

#[salsa::tracked(jar = ExprJar)]
pub struct AssociatedItemDefnExprSheet {
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub pattern_expr_arena: PatternExprSheet,
    #[return_ref]
    pub stmt_arena: StmtArena,
}
