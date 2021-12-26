use word::StmtKeyword;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicStmt {
    pub attr: StmtAttr,
    pub atoms: Vec<Atom>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StmtAttr {
    pub keyword: Option<StmtKeyword>,
    pub is_head: bool,
}

// new
impl AtomicStmt {
    pub(crate) fn new(keyword: Option<StmtKeyword>, is_head: bool, atoms: Vec<Atom>) -> Self {
        Self {
            attr: StmtAttr { keyword, is_head },
            atoms,
        }
    }
}
