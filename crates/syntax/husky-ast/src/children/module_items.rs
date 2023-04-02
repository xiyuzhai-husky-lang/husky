use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ModuleItems {
    ast_idx_range: AstIdxRange,
}

impl ModuleItems {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}
