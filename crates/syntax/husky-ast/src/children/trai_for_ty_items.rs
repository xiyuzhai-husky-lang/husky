use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitForTypeItems {
    children: AstIdxRange,
}

impl TraitForTypeItems {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}
