use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityUseExprRoot {
    ast_idx: AstIdx,
    accessibility: Accessibility,
    use_expr_idx: UseExprIdx,
}

impl EntityUseExprRoot {
    pub fn new(ast_idx: AstIdx, accessibility: Accessibility, use_expr_idx: UseExprIdx) -> Self {
        Self {
            ast_idx,
            accessibility,
            use_expr_idx,
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
