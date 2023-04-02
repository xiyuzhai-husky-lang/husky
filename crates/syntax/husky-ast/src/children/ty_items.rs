use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeItems {
    children: AstIdxRange,
}

impl TypeItems {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}
