use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FormBody {
    children: AstIdxRange,
}

impl FormBody {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}
