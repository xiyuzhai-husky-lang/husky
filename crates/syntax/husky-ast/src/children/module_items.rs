use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ModuleItems {
    children: AstIdxRange,
}

impl ModuleItems {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}
