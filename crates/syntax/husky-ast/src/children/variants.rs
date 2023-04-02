use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Variants {
    ast_idx_range: AstIdxRange,
}

impl Variants {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}
