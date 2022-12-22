use husky_word::Identifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityUseExprTracker {
    ast_idx: AstIdx,
    accessibility: Accessibility,
    ident: Identifier,
    use_expr_idx: UseExprIdx,
    parent: Option<EntityPath>,
    resolved: bool,
}

impl EntityUseExprTracker {
    pub fn new_root(
        ast_idx: AstIdx,
        accessibility: Accessibility,
        ident: Identifier,
        use_expr_idx: UseExprIdx,
    ) -> Self {
        Self {
            ast_idx,
            accessibility,
            use_expr_idx,
            ident,
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

    pub fn resolved(&self) -> bool {
        self.resolved
    }

    pub fn ident(&self) -> Identifier {
        self.ident
    }
}
