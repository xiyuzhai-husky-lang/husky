use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityUseExprTracker {
    ast_idx: AstIdx,
    accessibility: Accessibility,
    use_expr_idx: UseExprIdx,
    parent: Option<EntityPath>,
    resolved: bool,
}

impl EntityUseExprTracker {
    pub fn new_root(
        ast_idx: AstIdx,
        accessibility: Accessibility,
        use_expr_idx: UseExprIdx,
    ) -> Self {
        Self {
            ast_idx,
            accessibility,
            use_expr_idx,
            parent: None,
            resolved: false,
        }
    }

    pub fn ast_idx(&self) -> AstIdx {
        self.ast_idx
    }

    pub fn accessibility(&self) -> Accessibility {
        self.accessibility
    }

    pub fn use_expr_idx(&self) -> UseExprIdx {
        self.use_expr_idx
    }
}
